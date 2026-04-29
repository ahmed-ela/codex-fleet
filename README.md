# codex-fleet

Parallel Codex CLI worker queue with isolated git worktrees, telemetry, and optional tmux/Ratatui frontends.

## Usage

From inside a git or jj-backed project:

```sh
bin/codex-fleet
```

Default interactive behavior:

- `Enter` starts a worker immediately in the current batch.
- `Tab` queues a message for the next batch after the current batch finishes and applies/reconciles.
- `Ctrl-D` closes input and waits for workers/reconciliation.
- `Ctrl-C` clears input, or cancels and cleans when input is empty.

The backend defaults to `gpt-5.5` with `xhigh` reasoning and uses git worktrees for worker isolation.

## Build Ratatui Frontend

```sh
cd tools/codex-fleet-tui
cargo build --release
```

The `bin/codex-fleet tui` command uses the built binary at `tools/codex-fleet-tui/target/release/codex-fleet-tui` unless `CODEX_FLEET_TUI` is set.

## Install Locally

Symlink or copy `bin/codex-fleet` somewhere on your `PATH`.
