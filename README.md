# codex-fleet

Parallel Codex CLI worker queue with jj-first local snapshots, isolated git worktrees, telemetry, and a Ratatui frontend.

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

The backend defaults to `gpt-5.5` with `xhigh` reasoning. In a jj-backed repo, Fleet snapshots `jj @` before a batch and uses that exact commit as the worker base. Workers still use git worktrees for broad tool compatibility, and Fleet records the final jj snapshot after direct apply or reconciliation. Git/GitHub remain the publish layer; push to `main` after the final reconciled checkout is the state you want.

## Reconcile And Hygiene

Multi-worker batches run through:

```text
Workers -> Reconciler -> Hygiene -> Done
```

The reconciler merges successful worker changes into the main checkout. Hygiene then does a final stabilization pass for common LLM-generated-code residue: inconsistent naming, partial renames, stale imports, duplicate implementations, broken call sites, formatting/check failures, and oversized modules created by overlap.

Hygiene preserves product intent and is skipped for the single-worker direct-apply path. Disable it when needed:

```sh
codex-fleet run --no-hygiene
codex-fleet hygiene --run <run-id>
```

## Publish

Fleet treats GitHub as the trunk/publish layer, not as a branch per worker. After a local batch is reconciled and the checkout looks right, publish it explicitly:

```sh
codex-fleet publish -m "Apply fleet batch"
```

In a jj-backed repo this snapshots `jj @`, sets the `main` bookmark to that commit, and pushes `main` to `origin` with jj's remote safety checks. In a plain Git repo, publish requires a clean committed checkout and pushes `HEAD` to `origin/main`.

Useful options:

```sh
codex-fleet publish --dry-run
codex-fleet publish --remote origin --branch main -m "Ship UI cleanup"
```

## Project Layout

- `bin/codex-fleet`: launcher; defaults to the Ratatui UI.
- `bin/codex-fleet-backend`: CLI orchestration, worker lifecycle, reconciliation, health, and cleanup commands.
- `codex_fleet/prepare.py`: deterministic worker worktree prepare/cache logic.
- `tools/codex-fleet-tui`: Ratatui frontend.

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
