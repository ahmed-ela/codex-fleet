# codex-fleet

Parallel Codex CLI worker queue with isolated git worktrees, telemetry, and a Ratatui frontend.

## Usage

From inside a git or jj-backed project:

```sh
bin/codex-fleet
```

Default interactive behavior:

- `Enter` starts a worker immediately in the current batch.
- `Tab` queues a message for the next batch after the current batch finishes and applies/reconciles.
- `Ctrl-D` requests idle exit; input stays open while workers/reconciliation continue.
- `Ctrl-C` clears input, or cancels and cleans when input is empty.

Bare `codex-fleet` opens the Ratatui UI by default. The older tmux controller is still available as `codex-fleet headful`.

The backend defaults to `gpt-5.5` with `xhigh` reasoning and uses git worktrees for worker isolation.

## Worker Prepare

Before each worker starts Codex, `codex-fleet` runs deterministic prepare steps in that worker worktree. This keeps dependency setup out of the model prompt and logs setup time separately from model time.

By default it auto-detects JavaScript package roots by walking for `package.json` with a lockfile, skipping generated folders such as `node_modules`, `.next`, `dist`, `build`, `target`, and `.venv`.

Detected lockfiles map to:

- `bun.lock` / `bun.lockb`: `bun install --frozen-lockfile`
- `pnpm-lock.yaml`: `pnpm install --frozen-lockfile`
- `package-lock.json` / `npm-shrinkwrap.json`: `npm ci`
- `yarn.lock`: `yarn install --immutable` for Yarn 2+, otherwise `yarn install --frozen-lockfile`

Prepared outputs are cached under `.codex-fleet/cache/prepare` by command and lockfile contents, then copied into each worker worktree. Prepare outputs are ignored when collecting worker diffs.

Optional repo config can live at `.codex-fleet.toml`, `.codex-fleet/config.toml`, or `.codex-fleet/config.json`:

```toml
[prepare]
auto = true

[[prepare.steps]]
name = "app"
cwd = "apps/web"
command = "bun install --frozen-lockfile"
lockfiles = ["package.json", "bun.lock"]
outputs = ["node_modules"]
```

Set `enabled = false` under `[prepare]` to disable prepare entirely, or `auto = false` to use only explicit steps.

## Build Ratatui Frontend

```sh
cd tools/codex-fleet-tui
cargo build --release
```

The `bin/codex-fleet tui` command uses the built binary at `tools/codex-fleet-tui/target/release/codex-fleet-tui` unless `CODEX_FLEET_TUI` is set.

## Install Locally

Symlink or copy `bin/codex-fleet` somewhere on your `PATH`.
