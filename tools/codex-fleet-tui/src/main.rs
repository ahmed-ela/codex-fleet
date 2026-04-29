use std::{
    cmp,
    collections::VecDeque,
    env, fs,
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{
        self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use serde_json::{json, Value};

const DEFAULT_WORKERS: usize = 3;
const MAX_LOG_LINES: usize = 500;

#[derive(Debug)]
enum BackendEvent {
    Line(String),
    Done(i32),
}

#[derive(Debug, Clone)]
struct SubmittedJob {
    id: Option<String>,
    title: String,
    status: String,
    duration_seconds: Option<f64>,
    files_changed: usize,
    submitted_at: Instant,
    preview_lines: Vec<String>,
}

#[derive(Debug, Clone)]
struct ReconcilerStats {
    status: String,
    duration_seconds: Option<f64>,
    started_at: Option<Instant>,
    preview_lines: Vec<String>,
}

struct PreviewPane {
    title: String,
    status: String,
    lines: Vec<String>,
}

struct App {
    input: String,
    input_cursor: usize,
    jobs: Vec<SubmittedJob>,
    message_queue: VecDeque<String>,
    logs: VecDeque<String>,
    status: String,
    run_dir: Option<PathBuf>,
    phase: Option<String>,
    reconciler: Option<ReconcilerStats>,
    started_at: Instant,
    last_stats_refresh: Instant,
    history: Vec<String>,
    history_index: Option<usize>,
    history_draft: String,
    receiver: Receiver<BackendEvent>,
    sender: Sender<BackendEvent>,
    backend_stdin: Option<ChildStdin>,
    canceling: bool,
    done: bool,
    should_quit: bool,
    exit_code: i32,
}

impl App {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut app = Self {
            input: String::new(),
            input_cursor: 0,
            jobs: Vec::new(),
            message_queue: VecDeque::new(),
            logs: VecDeque::new(),
            status: "starting backend".to_string(),
            run_dir: None,
            phase: None,
            reconciler: None,
            started_at: Instant::now(),
            last_stats_refresh: Instant::now(),
            history: Vec::new(),
            history_index: None,
            history_draft: String::new(),
            receiver,
            sender: sender.clone(),
            backend_stdin: None,
            canceling: false,
            done: false,
            should_quit: false,
            exit_code: 0,
        };
        app.start_backend(sender);
        app
    }

    fn start_backend(&mut self, sender: Sender<BackendEvent>) {
        let backend = env::var("CODEX_FLEET_BACKEND")
            .unwrap_or_else(|_| "/Users/ahmedelamin/.dotfiles/bin/codex-fleet-backend".to_string());

        let mut child = match Command::new(&backend)
            .args([
                "live",
                "--danger",
                "--search",
                "--allow-dirty",
                "--flush-queue",
                "--auto-reconcile-when-idle",
                "--auto-reconcile-delay",
                "1",
                "-j",
                &DEFAULT_WORKERS.to_string(),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                self.status = format!("backend failed to start: {error}");
                self.exit_code = 1;
                self.done = true;
                return;
            }
        };

        self.backend_stdin = child.stdin.take();

        if let Some(stdout) = child.stdout.take() {
            let sender = sender.clone();
            thread::spawn(move || {
                for line in BufReader::new(stdout).lines() {
                    let _ = sender.send(BackendEvent::Line(
                        line.unwrap_or_else(|error| format!("stdout read error: {error}")),
                    ));
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let sender = sender.clone();
            thread::spawn(move || {
                for line in BufReader::new(stderr).lines() {
                    let _ = sender.send(BackendEvent::Line(format!(
                        "stderr: {}",
                        line.unwrap_or_else(|error| format!("stderr read error: {error}"))
                    )));
                }
            });
        }

        let sender = sender.clone();
        thread::spawn(move || {
            let code = match child.wait() {
                Ok(status) => status.code().unwrap_or(1),
                Err(error) => {
                    let _ =
                        sender.send(BackendEvent::Line(format!("backend wait failed: {error}")));
                    1
                }
            };
            let _ = sender.send(BackendEvent::Done(code));
        });
        self.status = "Enter starts now; Tab queues later".to_string();
        self.add_log(format!("backend: {backend}"));
    }

    fn add_log(&mut self, line: impl Into<String>) {
        let line = line.into();
        self.ingest_backend_line(&line);
        self.logs.push_back(line);
        while self.logs.len() > MAX_LOG_LINES {
            self.logs.pop_front();
        }
    }

    fn ingest_backend_line(&mut self, line: &str) {
        if let Some(path) = line.strip_prefix("Telemetry: ") {
            self.run_dir = Some(PathBuf::from(path.trim()));
            return;
        }

        let Some(rest) = line.strip_prefix("queued ") else {
            return;
        };
        let Some((job_id, title)) = rest.split_once(": ") else {
            return;
        };
        let exact_index = self
            .jobs
            .iter()
            .position(|job| job.id.is_none() && job.title == title);
        let fallback_index = self.jobs.iter().position(|job| job.id.is_none());
        if let Some(job) = exact_index
            .or(fallback_index)
            .and_then(|index| self.jobs.get_mut(index))
        {
            job.id = Some(job_id.to_string());
            job.status = "running".to_string();
        }
    }

    fn drain_events(&mut self) {
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                BackendEvent::Line(line) => self.add_log(line),
                BackendEvent::Done(code) => {
                    self.backend_stdin = None;
                    if !self.canceling && code == 0 && !self.message_queue.is_empty() {
                        let queued = self.message_queue.drain(..).collect::<Vec<_>>();
                        self.jobs.clear();
                        self.run_dir = None;
                        self.phase = None;
                        self.reconciler = None;
                        self.done = false;
                        self.exit_code = 0;
                        self.status = format!(
                            "starting next batch from {} queued message(s)",
                            queued.len()
                        );
                        self.add_log(format!(
                            "message queue: starting next batch from {} queued message(s)",
                            queued.len()
                        ));
                        self.start_backend(self.sender.clone());
                        for prompt in queued {
                            self.submit_prompt_text(prompt, "message queue");
                        }
                        return;
                    }
                    self.done = true;
                    self.exit_code = if self.canceling { 0 } else { code };
                    self.status = if self.canceling {
                        "canceled and cleaned; exiting".to_string()
                    } else if code == 0 {
                        "finished; exiting".to_string()
                    } else {
                        format!("backend exited with {code}; press q to exit")
                    };
                    if self.canceling || code == 0 {
                        self.should_quit = true;
                    }
                }
            }
        }
    }

    fn submit_prompt(&mut self) {
        if self.done || self.canceling {
            return;
        }

        let prompt = self.input.trim().to_string();
        if prompt.is_empty() {
            self.status = "empty prompt; type something first".to_string();
            return;
        }

        if !self.backend_accepts_immediate() {
            self.queue_prompt_for_next_batch(prompt, "current batch is already applying");
            return;
        }

        self.submit_prompt_text(prompt, "submitted");
    }

    fn submit_prompt_text(&mut self, prompt: String, source: &str) {
        if self.done || self.canceling {
            return;
        }

        let Some(stdin) = self.backend_stdin.as_mut() else {
            self.status = "backend stdin is not available".to_string();
            return;
        };

        let message = json!({
            "command": "prompt",
            "prompt": prompt,
        })
        .to_string();

        if let Err(error) = writeln!(stdin, "{message}") {
            self.status = format!("failed to send prompt: {error}");
            return;
        }
        if let Err(error) = stdin.flush() {
            self.status = format!("failed to flush prompt: {error}");
            return;
        }

        let title = prompt.chars().take(72).collect::<String>();
        self.jobs.push(SubmittedJob {
            id: None,
            title: title.clone(),
            status: "submitted".to_string(),
            duration_seconds: None,
            files_changed: 0,
            submitted_at: Instant::now(),
            preview_lines: Vec::new(),
        });
        if self.history.last() != Some(&prompt) {
            self.history.push(prompt);
        }
        self.history_index = None;
        self.history_draft.clear();
        self.input.clear();
        self.input_cursor = 0;
        self.status = format!("job {} started", self.jobs.len());
        self.add_log(format!("{source} job {}: {title}", self.jobs.len()));
    }

    fn queue_message(&mut self) {
        if self.done || self.canceling {
            return;
        }

        let prompt = self.input.trim().to_string();
        if prompt.is_empty() {
            self.status = "empty prompt; type something first".to_string();
            return;
        }

        self.queue_prompt_for_next_batch(prompt, "queued message");

        if self.jobs.is_empty() {
            self.start_message_queue_now();
        }
    }

    fn queue_prompt_for_next_batch(&mut self, prompt: String, source: &str) {
        if self.history.last() != Some(&prompt) {
            self.history.push(prompt.clone());
        }
        self.history_index = None;
        self.history_draft.clear();
        self.message_queue.push_back(prompt);
        self.input.clear();
        self.input_cursor = 0;
        self.status = format!("queued for next batch ({})", self.message_queue.len());
        self.add_log(format!(
            "{source}: queued message {} for after this batch",
            self.message_queue.len()
        ));
    }

    fn start_message_queue_now(&mut self) {
        if self.message_queue.is_empty() || !self.backend_accepts_immediate() {
            return;
        }
        let queued = self.message_queue.drain(..).collect::<Vec<_>>();
        self.status = format!("starting {} queued message(s)", queued.len());
        self.add_log(format!(
            "message queue: starting batch from {} queued message(s)",
            queued.len()
        ));
        for prompt in queued {
            self.submit_prompt_text(prompt, "message queue");
        }
    }

    fn backend_accepts_immediate(&self) -> bool {
        if self.backend_stdin.is_none() {
            return false;
        }
        !matches!(
            self.phase.as_deref(),
            Some(
                "reconciler_queued"
                    | "reconciling"
                    | "completed"
                    | "canceled"
                    | "reconciler_failed"
                    | "direct_apply_failed"
                    | "no_successful_workers"
                    | "workers_completed"
            )
        )
    }

    fn input_len(&self) -> usize {
        self.input.chars().count()
    }

    fn input_byte_index(&self) -> usize {
        if self.input_cursor == self.input_len() {
            return self.input.len();
        }
        self.input
            .char_indices()
            .nth(self.input_cursor)
            .map(|(index, _)| index)
            .unwrap_or(self.input.len())
    }

    fn insert_input_char(&mut self, ch: char) {
        let index = self.input_byte_index();
        self.input.insert(index, ch);
        self.input_cursor += 1;
        self.history_index = None;
        self.history_draft.clear();
    }

    fn insert_input_text(&mut self, text: &str) {
        if self.done || self.canceling {
            return;
        }
        let text = text.replace("\r\n", "\n").replace('\r', "\n");
        if text.is_empty() {
            return;
        }
        let index = self.input_byte_index();
        self.input.insert_str(index, &text);
        self.input_cursor += text.chars().count();
        self.history_index = None;
        self.history_draft.clear();
        self.status = format!("pasted {} chars", text.chars().count());
    }

    fn delete_input_before_cursor(&mut self) {
        if self.input_cursor == 0 {
            return;
        }
        let end = self.input_byte_index();
        self.input_cursor -= 1;
        let start = self.input_byte_index();
        self.input.replace_range(start..end, "");
        self.history_index = None;
        self.history_draft.clear();
    }

    fn delete_input_previous_word(&mut self) {
        if self.input_cursor == 0 {
            return;
        }

        let chars = self.input.chars().collect::<Vec<_>>();
        let end = self.input_cursor;
        let mut start = end;
        while start > 0 && chars[start - 1].is_whitespace() {
            start -= 1;
        }
        while start > 0 && !chars[start - 1].is_whitespace() {
            start -= 1;
        }

        let start_byte = char_to_byte_index(&self.input, start);
        let end_byte = char_to_byte_index(&self.input, end);
        self.input.replace_range(start_byte..end_byte, "");
        self.input_cursor = start;
        self.history_index = None;
        self.history_draft.clear();
    }

    fn move_input_previous_word(&mut self) {
        if self.input_cursor == 0 {
            return;
        }

        let chars = self.input.chars().collect::<Vec<_>>();
        let mut start = self.input_cursor;
        while start > 0 && chars[start - 1].is_whitespace() {
            start -= 1;
        }
        while start > 0 && !chars[start - 1].is_whitespace() {
            start -= 1;
        }
        self.input_cursor = start;
    }

    fn move_input_next_word(&mut self) {
        let chars = self.input.chars().collect::<Vec<_>>();
        let mut end = self.input_cursor;
        while end < chars.len() && !chars[end].is_whitespace() {
            end += 1;
        }
        while end < chars.len() && chars[end].is_whitespace() {
            end += 1;
        }
        self.input_cursor = end;
    }

    fn set_input_from_history(&mut self, value: String) {
        self.input = value;
        self.input_cursor = self.input_len();
    }

    fn browse_history(&mut self, direction: i32) {
        if self.history.is_empty() {
            return;
        }

        let next_index = match (self.history_index, direction) {
            (None, d) if d < 0 => {
                self.history_draft = self.input.clone();
                Some(self.history.len() - 1)
            }
            (None, _) => None,
            (Some(index), d) if d < 0 => Some(index.saturating_sub(1)),
            (Some(index), _) if index + 1 < self.history.len() => Some(index + 1),
            (Some(_), _) => {
                let draft = std::mem::take(&mut self.history_draft);
                self.history_index = None;
                self.set_input_from_history(draft);
                return;
            }
        };

        if let Some(index) = next_index {
            let value = self.history[index].clone();
            self.history_index = Some(index);
            self.set_input_from_history(value);
        }
    }

    fn delete_input_to_start(&mut self) {
        if self.input_cursor == 0 {
            return;
        }
        let end = self.input_byte_index();
        self.input.replace_range(0..end, "");
        self.input_cursor = 0;
        self.history_index = None;
        self.history_draft.clear();
    }

    fn finish(&mut self) {
        if self.done || self.canceling {
            return;
        }

        if self.jobs.is_empty() && self.message_queue.is_empty() {
            if let Some(stdin) = self.backend_stdin.as_mut() {
                let _ = writeln!(stdin, "{}", json!({ "command": "finish" }));
                let _ = stdin.flush();
            }
            self.status = "no queued work; exiting".to_string();
            return;
        }

        if self.jobs.is_empty() {
            self.start_message_queue_now();
            return;
        }

        self.status = "will exit after the queue drains; input stays open".to_string();
    }

    fn abort(&mut self) {
        if self.done {
            self.should_quit = true;
            return;
        }
        if self.canceling {
            return;
        }

        self.canceling = true;
        self.status = "canceling workers and cleaning worktrees".to_string();

        let sent_cancel = if let Some(mut stdin) = self.backend_stdin.take() {
            let _ = writeln!(stdin, "{}", json!({ "command": "cancel" }));
            let _ = stdin.flush();
            true
        } else {
            false
        };

        if !sent_cancel {
            self.should_quit = true;
        }
    }

    fn refresh_stats(&mut self) {
        if self.last_stats_refresh.elapsed() < Duration::from_millis(500) {
            return;
        }
        self.last_stats_refresh = Instant::now();

        let Some(run_dir) = self.run_dir.clone() else {
            return;
        };
        let run_json_path = run_dir.join("run.json");
        let Ok(text) = fs::read_to_string(run_json_path) else {
            return;
        };
        let Ok(value) = serde_json::from_str::<Value>(&text) else {
            return;
        };

        self.phase = value
            .get("phase")
            .and_then(Value::as_str)
            .map(str::to_string);

        if let Some(workers) = value.get("workers").and_then(Value::as_object) {
            for (job_id, worker) in workers {
                let title = worker
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or(job_id)
                    .to_string();
                let status = worker
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
                    .to_string();
                let duration_seconds = worker.get("duration_seconds").and_then(Value::as_f64);
                let files_changed = worker
                    .get("files_changed")
                    .and_then(Value::as_array)
                    .map(Vec::len)
                    .unwrap_or(0);
                let preview_lines = read_codex_preview(
                    &run_dir.join("workers").join(job_id).join("stdout.jsonl"),
                    &run_dir.join("workers").join(job_id).join("stderr.log"),
                    80,
                );

                if let Some(job) = self
                    .jobs
                    .iter_mut()
                    .find(|job| job.id.as_deref() == Some(job_id.as_str()))
                {
                    job.title = title;
                    job.status = status;
                    job.duration_seconds = duration_seconds;
                    job.files_changed = files_changed;
                    job.preview_lines = preview_lines;
                    continue;
                }

                if let Some(job) = self
                    .jobs
                    .iter_mut()
                    .find(|job| job.id.is_none() && job.title == title)
                {
                    job.id = Some(job_id.to_string());
                    job.status = status;
                    job.duration_seconds = duration_seconds;
                    job.files_changed = files_changed;
                    job.preview_lines = preview_lines;
                    continue;
                }

                self.jobs.push(SubmittedJob {
                    id: Some(job_id.to_string()),
                    title,
                    status,
                    duration_seconds,
                    files_changed,
                    submitted_at: Instant::now(),
                    preview_lines,
                });
            }
        }

        if let Some(reconciler) = value.get("reconciler").and_then(Value::as_object) {
            let status = reconciler
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string();
            let duration_seconds = reconciler.get("duration_seconds").and_then(Value::as_f64);
            let started_at = if status == "running" {
                self.reconciler
                    .as_ref()
                    .and_then(|stats| stats.started_at)
                    .or_else(|| Some(Instant::now()))
            } else {
                self.reconciler.as_ref().and_then(|stats| stats.started_at)
            };
            let preview_lines = read_codex_preview(
                &run_dir.join("reconciler").join("stdout.jsonl"),
                &run_dir.join("reconciler").join("stderr.log"),
                80,
            );
            self.reconciler = Some(ReconcilerStats {
                status,
                duration_seconds,
                started_at,
                preview_lines,
            });
        }
    }
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, EnableBracketedPaste)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), DisableBracketedPaste, LeaveAlternateScreen);
    }
}

fn main() -> Result<()> {
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let result = run_app(&mut terminal);
    terminal.show_cursor()?;
    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new();

    loop {
        app.drain_events();
        app.refresh_stats();
        terminal.draw(|frame| render(frame, &app))?;

        if app.should_quit {
            return Ok(());
        }

        if event::poll(Duration::from_millis(80))? {
            match event::read()? {
                Event::Key(key) => handle_key(&mut app, key),
                Event::Paste(text) => app.insert_input_text(&text),
                _ => {}
            }
        }
    }
}

fn handle_key(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        if !app.input.is_empty() && !app.done && !app.canceling {
            app.input.clear();
            app.input_cursor = 0;
            app.history_index = None;
            app.history_draft.clear();
            app.status = "input cleared".to_string();
            return;
        }
        app.abort();
        return;
    }

    if app.done {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => app.should_quit = true,
            _ => {}
        }
        return;
    }

    if app.canceling {
        return;
    }

    if key.modifiers.contains(KeyModifiers::ALT) {
        match key.code {
            KeyCode::Backspace => app.delete_input_previous_word(),
            KeyCode::Left | KeyCode::Char('b') => app.move_input_previous_word(),
            KeyCode::Right | KeyCode::Char('f') => app.move_input_next_word(),
            _ => {}
        }
        return;
    }

    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('a') => app.input_cursor = 0,
            KeyCode::Char('e') => app.input_cursor = app.input_len(),
            KeyCode::Char('d') => app.finish(),
            KeyCode::Char('u') => app.delete_input_to_start(),
            KeyCode::Char('w') => app.delete_input_previous_word(),
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Esc => app.abort(),
        KeyCode::Enter => app.submit_prompt(),
        KeyCode::Tab => app.queue_message(),
        KeyCode::Backspace => app.delete_input_before_cursor(),
        KeyCode::Home => app.input_cursor = 0,
        KeyCode::End => app.input_cursor = app.input_len(),
        KeyCode::Left => app.input_cursor = app.input_cursor.saturating_sub(1),
        KeyCode::Right => app.input_cursor = cmp::min(app.input_cursor + 1, app.input_len()),
        KeyCode::Up => app.browse_history(-1),
        KeyCode::Down => app.browse_history(1),
        KeyCode::Char('q') if app.input.is_empty() => app.abort(),
        KeyCode::Char(ch) => app.insert_input_char(ch),
        _ => {}
    }
}

fn char_to_byte_index(value: &str, char_index: usize) -> usize {
    if char_index == value.chars().count() {
        return value.len();
    }
    value
        .char_indices()
        .nth(char_index)
        .map(|(index, _)| index)
        .unwrap_or(value.len())
}

fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "Fleet",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format_duration(app.started_at.elapsed().as_secs_f64()),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw("  "),
        Span::raw(app.status.as_str()),
    ]))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    render_preview_dashboard(frame, chunks[1], app);

    let input_area = chunks[2];
    let input_inner = inner(input_area);
    let prompt = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Prompt").borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    frame.render_widget(prompt, input_area);

    if !app.done && !app.canceling {
        frame.set_cursor_position(input_cursor_position(app, input_inner));
    }

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            "Enter",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" start  "),
        Span::styled(
            "Tab",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" later  "),
        Span::styled(
            "Ctrl-D",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" done  "),
        Span::styled(
            "Ctrl-C",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" clear/cancel"),
    ]))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[3]);
}

fn input_cursor_position(app: &App, input_inner: Rect) -> Position {
    let mut row = 0usize;
    let mut col = 0usize;
    for ch in app.input.chars().take(app.input_cursor) {
        if ch == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    Position::new(
        input_inner.x + cmp::min(col, input_inner.width.saturating_sub(1) as usize) as u16,
        input_inner.y + cmp::min(row, input_inner.height.saturating_sub(1) as usize) as u16,
    )
}

fn render_preview_dashboard(frame: &mut Frame, area: Rect, app: &App) {
    let panes = build_preview_panes(app);
    render_preview_panes(frame, area, &panes);
}

fn build_preview_panes(app: &App) -> Vec<PreviewPane> {
    if app.jobs.is_empty() {
        let mut lines = vec![
            format!("phase: {}", app.phase.as_deref().unwrap_or("starting")),
            "workers: 0".to_string(),
            format!("next batch: {}", app.message_queue.len()),
        ];
        if let Some(run_dir) = app.run_dir.as_ref() {
            lines.push(format!("telemetry: {}", run_dir.display()));
        } else {
            lines.push("telemetry: starting".to_string());
        }
        if !app.message_queue.is_empty() {
            lines.push(String::new());
            lines.extend(
                app.message_queue
                    .iter()
                    .enumerate()
                    .map(|(index, message)| {
                        format!("#{} {}", index + 1, truncate_line(message, 180))
                    }),
            );
        } else if let Some(last_log) = app.logs.back() {
            lines.push(format!("last: {}", truncate_line(last_log, 180)));
        }
        return vec![PreviewPane {
            title: "Run".to_string(),
            status: "idle".to_string(),
            lines,
        }];
    }

    let mut panes = app
        .jobs
        .iter()
        .enumerate()
        .map(|(index, job)| {
            let duration = job_display_duration(job);
            let meta = format!(
                "{} {}{}",
                job.status,
                format_duration(duration),
                if job.files_changed > 0 {
                    format!(" {} files", job.files_changed)
                } else {
                    String::new()
                },
            );
            let mut lines = job.preview_lines.clone();
            if lines.is_empty() {
                lines.push("waiting for Codex output...".to_string());
            }
            PreviewPane {
                title: format!("#{} [{}] {}", index + 1, meta, job.title),
                status: job.status.clone(),
                lines,
            }
        })
        .collect::<Vec<_>>();

    if !app.message_queue.is_empty() {
        panes.push(PreviewPane {
            title: format!("Message Queue [{} pending]", app.message_queue.len()),
            status: "queued".to_string(),
            lines: app
                .message_queue
                .iter()
                .enumerate()
                .map(|(index, message)| format!("#{} {}", index + 1, truncate_line(message, 180)))
                .collect(),
        });
    }

    let workers_done = app.jobs.iter().all(|job| is_terminal_status(&job.status));
    if let Some(reconciler) = app.reconciler.as_ref() {
        let duration = reconciler_display_duration(reconciler);
        let mut lines = reconciler.preview_lines.clone();
        if lines.is_empty() {
            lines.push("waiting for reconciler output...".to_string());
        }
        panes.push(PreviewPane {
            title: format!(
                "Reconciler [{} {}]",
                reconciler.status,
                format_duration(duration)
            ),
            status: reconciler.status.clone(),
            lines,
        });
    } else if workers_done {
        panes.push(PreviewPane {
            title: "Reconciler [pending]".to_string(),
            status: "queued".to_string(),
            lines: vec!["workers are done; waiting for reconciler to start...".to_string()],
        });
    }

    panes
}

fn render_preview_panes(frame: &mut Frame, area: Rect, panes: &[PreviewPane]) {
    let count = panes.len().max(1);
    let cols = preview_column_count(count, area.width);
    let rows = count.div_ceil(cols);
    let row_constraints = vec![Constraint::Ratio(1, rows as u32); rows];
    let row_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(area);

    for row_index in 0..rows {
        let start = row_index * cols;
        let end = cmp::min(start + cols, count);
        let visible_cols = end - start;
        let col_constraints = vec![Constraint::Ratio(1, visible_cols as u32); visible_cols];
        let col_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(row_chunks[row_index]);

        for (offset, pane) in panes[start..end].iter().enumerate() {
            render_preview_pane(frame, col_chunks[offset], pane);
        }
    }
}

fn preview_column_count(count: usize, width: u16) -> usize {
    if count <= 1 || width < 100 {
        return 1;
    }
    if width < 150 {
        return cmp::min(count, 2);
    }
    match count {
        2 => 2,
        3 => 3,
        4 => 2,
        _ => 3,
    }
}

fn render_preview_pane(frame: &mut Frame, area: Rect, pane: &PreviewPane) {
    let max_lines = area.height.saturating_sub(2) as usize;
    let start = pane.lines.len().saturating_sub(max_lines);
    let text = pane.lines[start..].join("\n");
    let block = Block::default()
        .title(pane.title.as_str())
        .borders(Borders::ALL)
        .border_style(style_for_status(&pane.status));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn inner(area: Rect) -> Rect {
    Rect {
        x: area.x.saturating_add(1),
        y: area.y.saturating_add(1),
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

fn job_display_duration(job: &SubmittedJob) -> f64 {
    job.duration_seconds.unwrap_or_else(|| {
        if matches!(
            job.status.as_str(),
            "submitted" | "queued" | "preparing" | "running"
        ) {
            job.submitted_at.elapsed().as_secs_f64()
        } else {
            0.0
        }
    })
}

fn reconciler_display_duration(reconciler: &ReconcilerStats) -> f64 {
    reconciler.duration_seconds.unwrap_or_else(|| {
        if reconciler.status == "running" {
            reconciler
                .started_at
                .map(|started| started.elapsed().as_secs_f64())
                .unwrap_or(0.0)
        } else {
            0.0
        }
    })
}

fn format_duration(seconds: f64) -> String {
    let total = seconds.max(0.0).round() as u64;
    let minutes = total / 60;
    let seconds = total % 60;
    if minutes >= 60 {
        format!("{}h{:02}m", minutes / 60, minutes % 60)
    } else if minutes > 0 {
        format!("{minutes}m{seconds:02}s")
    } else {
        format!("{seconds}s")
    }
}

fn style_for_status(status: &str) -> Style {
    match status {
        "succeeded" => Style::default().fg(Color::Green),
        "failed" => Style::default().fg(Color::Red),
        "canceled" => Style::default().fg(Color::Yellow),
        "running" | "preparing" | "queued" | "submitted" => Style::default().fg(Color::Cyan),
        _ => Style::default().fg(Color::Gray),
    }
}

fn is_terminal_status(status: &str) -> bool {
    matches!(status, "succeeded" | "failed" | "canceled")
}

fn read_codex_preview(stdout_path: &Path, stderr_path: &Path, max_lines: usize) -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(text) = fs::read_to_string(stdout_path) {
        let raw_lines = tail_text_lines(&text, 180);
        for raw in raw_lines {
            append_codex_event_preview(raw, &mut lines);
        }
    }

    if lines.is_empty() {
        if let Ok(text) = fs::read_to_string(stderr_path) {
            lines.extend(
                tail_text_lines(&text, max_lines)
                    .into_iter()
                    .map(|line| format!("stderr: {}", truncate_line(line.trim(), 180))),
            );
        }
    }

    let start = lines.len().saturating_sub(max_lines);
    lines[start..].to_vec()
}

fn append_codex_event_preview(raw: &str, out: &mut Vec<String>) {
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return;
    };
    let event_type = value.get("type").and_then(Value::as_str).unwrap_or("");
    match event_type {
        "thread.started" => {
            if let Some(thread_id) = value.get("thread_id").and_then(Value::as_str) {
                out.push(format!("thread {}", truncate_line(thread_id, 80)));
            }
        }
        "turn.started" => out.push("turn started".to_string()),
        "turn.completed" => out.push("turn completed".to_string()),
        "item.started" => {
            if let Some(item) = value.get("item") {
                append_item_preview(item, true, out);
            }
        }
        "item.completed" => {
            if let Some(item) = value.get("item") {
                append_item_preview(item, false, out);
            }
        }
        _ => {}
    }
}

fn append_item_preview(item: &Value, started: bool, out: &mut Vec<String>) {
    let item_type = item.get("type").and_then(Value::as_str).unwrap_or("");
    match item_type {
        "agent_message" => {
            if let Some(text) = item.get("text").and_then(Value::as_str) {
                append_text_preview(text, out, 10);
            }
        }
        "command_execution" => {
            if started {
                if let Some(command) = item.get("command").and_then(Value::as_str) {
                    out.push(format!("$ {}", truncate_line(command, 180)));
                }
                return;
            }

            if let Some(output) = item.get("aggregated_output").and_then(Value::as_str) {
                append_text_preview(output, out, 8);
            }
            let exit_code = item.get("exit_code").and_then(Value::as_i64);
            let status = item.get("status").and_then(Value::as_str).unwrap_or("");
            if status == "failed" || exit_code.is_some_and(|code| code != 0) {
                out.push(format!("command exited {}", exit_code.unwrap_or(-1)));
            }
        }
        "file_change" => {
            let verb = if started { "editing" } else { "edited" };
            let mut paths = item
                .get("changes")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|change| change.get("path").and_then(Value::as_str))
                .take(4)
                .map(|path| truncate_line(path, 120))
                .collect::<Vec<_>>();
            if paths.is_empty() {
                paths.push("files".to_string());
            }
            out.push(format!("{verb} {}", paths.join(", ")));
        }
        _ => {}
    }
}

fn append_text_preview(text: &str, out: &mut Vec<String>, max_lines: usize) {
    for line in tail_text_lines(text, max_lines) {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            out.push(truncate_line(trimmed, 180));
        }
    }
}

fn tail_text_lines(text: &str, max_lines: usize) -> Vec<&str> {
    let mut lines = text.lines().rev().take(max_lines).collect::<Vec<_>>();
    lines.reverse();
    lines
}

fn truncate_line(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let mut result = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    result.push_str("...");
    result
}
