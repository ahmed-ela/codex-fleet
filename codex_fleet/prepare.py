from __future__ import annotations

import datetime as dt
import fcntl
import hashlib
import json
import os
import pathlib
import random
import re
import shlex
import shutil
import subprocess
import time
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:  # Python < 3.11
    try:
        import tomli as tomllib  # type: ignore[no-redef]
    except ModuleNotFoundError:
        tomllib = None  # type: ignore[assignment]


STATE_DIR = ".codex-fleet"
CACHE_DIR = "cache"
PREPARE_CACHE_DIR = "prepare"


class PrepareError(RuntimeError):
    pass


def now_iso() -> str:
    return dt.datetime.now(dt.timezone.utc).astimezone().isoformat(timespec="seconds")


def slugify(value: str, fallback: str = "job") -> str:
    value = value.strip().lower()
    value = re.sub(r"[^a-z0-9]+", "-", value)
    value = value.strip("-")
    return (value or fallback)[:60]


def shell_join(args: list[str]) -> str:
    return " ".join(shlex.quote(str(arg)) for arg in args)


def run_cmd(
    args: list[str],
    cwd: pathlib.Path | None = None,
    check: bool = True,
) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(
        args,
        cwd=str(cwd) if cwd else None,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if check and result.returncode != 0:
        raise PrepareError(
            f"Command failed ({result.returncode}): {shell_join(args)}\n{result.stderr.strip()}"
        )
    return result


def prepare_ignored_paths(worker_dir: pathlib.Path) -> list[str]:
    path = worker_dir / "prepare-ignored-paths.json"
    if not path.exists():
        return []
    try:
        value = json.loads(path.read_text())
    except json.JSONDecodeError:
        return []
    if not isinstance(value, list):
        return []
    return [str(item).strip("/") for item in value if str(item).strip("/")]


def path_is_under(path_text: str, ignored_path: str) -> bool:
    path_text = path_text.strip().strip("/")
    ignored_path = ignored_path.strip().strip("/")
    return path_text == ignored_path or path_text.startswith(ignored_path + "/")


def status_path_text(line: str) -> str:
    path_text = line[3:] if len(line) > 3 else line
    if " -> " in path_text:
        path_text = path_text.split(" -> ", 1)[1]
    return path_text.strip()


def filter_prepare_artifacts(text: str, ignored_paths: list[str]) -> str:
    if not ignored_paths:
        return text
    lines: list[str] = []
    for line in text.splitlines():
        path_text = status_path_text(line)
        if any(path_is_under(path_text, ignored) for ignored in ignored_paths):
            continue
        lines.append(line)
    return "\n".join(lines) + ("\n" if lines else "")


def filter_prepare_untracked(text: str, ignored_paths: list[str]) -> str:
    if not ignored_paths:
        return text
    lines = [
        line
        for line in text.splitlines()
        if not any(path_is_under(line, ignored) for ignored in ignored_paths)
    ]
    return "\n".join(lines) + ("\n" if lines else "")


PREPARE_PRUNE_DIRS = {
    ".git",
    ".jj",
    STATE_DIR,
    "node_modules",
    ".next",
    "dist",
    "build",
    "target",
    ".venv",
    "venv",
    "__pycache__",
}


def clean_rel_path(value: str, label: str) -> pathlib.PurePosixPath:
    text = str(value).strip()
    path = pathlib.PurePosixPath(text)
    if not text or path.is_absolute() or ".." in path.parts:
        raise PrepareError(f"Invalid prepare {label}: {value}")
    return path


def strip_toml_comment(line: str) -> str:
    in_single = False
    in_double = False
    escaped = False
    result: list[str] = []
    for char in line:
        if escaped:
            result.append(char)
            escaped = False
            continue
        if char == "\\" and in_double:
            result.append(char)
            escaped = True
            continue
        if char == '"' and not in_single:
            in_double = not in_double
        elif char == "'" and not in_double:
            in_single = not in_single
        elif char == "#" and not in_single and not in_double:
            break
        result.append(char)
    return "".join(result).strip()


def split_toml_array(value: str) -> list[str]:
    items: list[str] = []
    current: list[str] = []
    in_single = False
    in_double = False
    escaped = False
    for char in value:
        if escaped:
            current.append(char)
            escaped = False
            continue
        if char == "\\" and in_double:
            current.append(char)
            escaped = True
            continue
        if char == '"' and not in_single:
            in_double = not in_double
        elif char == "'" and not in_double:
            in_single = not in_single
        if char == "," and not in_single and not in_double:
            item = "".join(current).strip()
            if item:
                items.append(item)
            current = []
            continue
        current.append(char)
    item = "".join(current).strip()
    if item:
        items.append(item)
    return items


def parse_basic_toml_value(value: str) -> Any:
    value = value.strip()
    if value in {"true", "false"}:
        return value == "true"
    if value.startswith('"') and value.endswith('"'):
        return json.loads(value)
    if value.startswith("'") and value.endswith("'"):
        return value[1:-1]
    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        if not inner:
            return []
        return [parse_basic_toml_value(item) for item in split_toml_array(inner)]
    if re.fullmatch(r"-?\d+", value):
        return int(value)
    raise PrepareError(
        "TOML config needs Python 3.11+, the tomli package, or simple string/bool/array values."
    )


def basic_prepare_toml(text: str) -> dict[str, Any]:
    data: dict[str, Any] = {}
    current: dict[str, Any] | None = None
    for raw_line in text.splitlines():
        line = strip_toml_comment(raw_line)
        if not line:
            continue
        if line.startswith("[[") and line.endswith("]]"):
            table = line[2:-2].strip()
            if table != "prepare.steps":
                raise PrepareError(f"Unsupported TOML array table: {table}")
            prepare = data.setdefault("prepare", {})
            if not isinstance(prepare, dict):
                raise PrepareError("Invalid TOML prepare table.")
            steps = prepare.setdefault("steps", [])
            if not isinstance(steps, list):
                raise PrepareError("Invalid TOML prepare.steps table.")
            current = {}
            steps.append(current)
            continue
        if line.startswith("[") and line.endswith("]"):
            table = line[1:-1].strip()
            parts = [part.strip() for part in table.split(".") if part.strip()]
            current = data
            for part in parts:
                next_table = current.setdefault(part, {})
                if not isinstance(next_table, dict):
                    raise PrepareError(f"Invalid TOML table: {table}")
                current = next_table
            continue
        if "=" not in line:
            raise PrepareError(f"Invalid TOML line: {raw_line.strip()}")
        key, raw_value = line.split("=", 1)
        if current is None:
            current = data
        current[key.strip()] = parse_basic_toml_value(raw_value)
    return data


def load_toml(path: pathlib.Path) -> dict[str, Any]:
    text = path.read_text()
    if tomllib is not None:
        return tomllib.loads(text)
    return basic_prepare_toml(text)


def load_prepare_config(root: pathlib.Path) -> dict[str, Any]:
    for path in [root / ".codex-fleet.toml", root / ".codex-fleet" / "config.toml"]:
        if path.exists():
            data = load_toml(path)
            return data.get("prepare", {}) if isinstance(data.get("prepare"), dict) else {}

    json_path = root / ".codex-fleet" / "config.json"
    if json_path.exists():
        data = json.loads(json_path.read_text())
        return data.get("prepare", {}) if isinstance(data.get("prepare"), dict) else {}
    return {}


def package_json_manager_command(package_json: pathlib.Path, lockfile: str) -> list[str]:
    if lockfile in {"bun.lock", "bun.lockb"}:
        return ["bun", "install", "--frozen-lockfile"]
    if lockfile == "pnpm-lock.yaml":
        return ["pnpm", "install", "--frozen-lockfile"]
    if lockfile in {"package-lock.json", "npm-shrinkwrap.json"}:
        return ["npm", "ci"]
    if lockfile == "yarn.lock":
        try:
            package_data = json.loads(package_json.read_text())
        except json.JSONDecodeError:
            package_data = {}
        package_manager = str(package_data.get("packageManager") or "")
        match = re.match(r"yarn@(\d+)", package_manager)
        if match and int(match.group(1)) >= 2:
            return ["yarn", "install", "--immutable"]
        return ["yarn", "install", "--frozen-lockfile"]
    raise PrepareError(f"Unsupported JS lockfile: {lockfile}")


def iter_package_json_roots(root: pathlib.Path) -> list[pathlib.Path]:
    package_roots: list[pathlib.Path] = []
    for current, dirs, files in os.walk(root):
        dirs[:] = [
            name
            for name in dirs
            if name not in PREPARE_PRUNE_DIRS and not name.startswith(".cache")
        ]
        if "package.json" in files:
            package_roots.append(pathlib.Path(current))
    return sorted(package_roots)


def discover_prepare_specs(root: pathlib.Path) -> list[dict[str, Any]]:
    config = load_prepare_config(root)
    if config.get("enabled") is False:
        return []

    specs: list[dict[str, Any]] = []
    explicit_cwds: set[str] = set()
    for index, step in enumerate(config.get("steps") or []):
        if not isinstance(step, dict) or step.get("enabled") is False:
            continue
        cwd = clean_rel_path(str(step.get("cwd") or "."), "cwd")
        lockfiles = [
            str(clean_rel_path(str(item), "lockfile"))
            for item in step.get("lockfiles") or []
        ]
        outputs = [
            str(clean_rel_path(str(item), "output"))
            for item in step.get("outputs") or ["node_modules"]
        ]
        command = step.get("command") or step.get("install")
        if not command:
            raise PrepareError(f"Prepare step {step.get('name') or index + 1} is missing command/install.")
        specs.append(
            {
                "name": str(step.get("name") or cwd.name or f"step-{index + 1}"),
                "cwd": str(cwd),
                "command": command,
                "lockfiles": lockfiles,
                "outputs": outputs,
                "cache": bool(step.get("cache", True)),
                "source": "config",
            }
        )
        explicit_cwds.add(str(cwd))

    if config.get("auto", True) is False:
        return specs

    lockfile_priority = [
        "bun.lockb",
        "bun.lock",
        "pnpm-lock.yaml",
        "package-lock.json",
        "npm-shrinkwrap.json",
        "yarn.lock",
    ]
    for package_root in iter_package_json_roots(root):
        rel_cwd = package_root.relative_to(root).as_posix() or "."
        if rel_cwd in explicit_cwds:
            continue
        lockfile = next((name for name in lockfile_priority if (package_root / name).exists()), None)
        if not lockfile:
            continue
        lockfiles = ["package.json", lockfile]
        specs.append(
            {
                "name": f"{rel_cwd.replace('/', '-')}-{lockfile.split('.', 1)[0]}",
                "cwd": rel_cwd,
                "command": package_json_manager_command(package_root / "package.json", lockfile),
                "lockfiles": lockfiles,
                "outputs": ["node_modules"],
                "cache": True,
                "source": "auto",
            }
        )
    return specs


def prepare_command_label(command: Any) -> str:
    if isinstance(command, list):
        return shell_join([str(part) for part in command])
    return str(command)


def prepare_cache_key(root: pathlib.Path, spec: dict[str, Any]) -> str:
    digest = hashlib.sha256()
    digest.update(str(spec.get("cwd")).encode())
    digest.update(b"\0")
    digest.update(prepare_command_label(spec.get("command")).encode())
    digest.update(b"\0")
    package_root = root / str(spec["cwd"])
    for rel_lockfile in spec.get("lockfiles") or []:
        lock_path = package_root / rel_lockfile
        digest.update(str(rel_lockfile).encode())
        digest.update(b"\0")
        if lock_path.exists():
            digest.update(lock_path.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()[:24]


def copy_prepare_output(source: pathlib.Path, destination: pathlib.Path) -> None:
    if destination.exists() or destination.is_symlink():
        if destination.is_symlink() or destination.is_file():
            destination.unlink()
        else:
            shutil.rmtree(destination)
    destination.parent.mkdir(parents=True, exist_ok=True)
    if source.is_symlink():
        destination.symlink_to(os.readlink(source))
        return
    result = run_cmd(["cp", "-cR", str(source), str(destination)], check=False)
    if result.returncode == 0:
        return
    if source.is_dir():
        shutil.copytree(source, destination, symlinks=True)
    else:
        shutil.copy2(source, destination)


def run_prepare_command(
    command: Any,
    cwd: pathlib.Path,
    stdout_path: pathlib.Path,
    stderr_path: pathlib.Path,
) -> int:
    with stdout_path.open("wb") as stdout_file, stderr_path.open("wb") as stderr_file:
        if isinstance(command, list):
            result = subprocess.run(
                [str(part) for part in command],
                cwd=cwd,
                stdout=stdout_file,
                stderr=stderr_file,
            )
        else:
            result = subprocess.run(
                str(command),
                cwd=cwd,
                shell=True,
                stdout=stdout_file,
                stderr=stderr_file,
                executable=os.environ.get("SHELL") or None,
            )
    return int(result.returncode)


def write_prepare_ignored(worker_dir: pathlib.Path, ignored_paths: list[str]) -> None:
    existing = prepare_ignored_paths(worker_dir)
    combined = sorted(set(existing + ignored_paths))
    (worker_dir / "prepare-ignored-paths.json").write_text(json.dumps(combined, indent=2) + "\n")


def prepare_worktree_dependencies(
    root: pathlib.Path,
    state: pathlib.Path,
    worktree: pathlib.Path,
    worker_dir: pathlib.Path,
) -> dict[str, Any]:
    specs = discover_prepare_specs(root)
    prepare_dir = worker_dir / "prepare"
    prepare_dir.mkdir(parents=True, exist_ok=True)

    summary: dict[str, Any] = {
        "status": "skipped" if not specs else "succeeded",
        "started_at": now_iso(),
        "completed_at": None,
        "duration_seconds": 0.0,
        "steps": [],
        "ignored_paths": [],
    }
    start = time.monotonic()

    for index, spec in enumerate(specs, start=1):
        name = slugify(str(spec.get("name") or f"step-{index}"), f"step-{index}")
        step_dir = prepare_dir / f"{index:02d}-{name}"
        step_dir.mkdir(parents=True, exist_ok=True)
        package_root = worktree / str(spec["cwd"])
        if not package_root.exists():
            raise PrepareError(f"Prepare cwd does not exist in worktree: {spec['cwd']}")

        ignored_paths = []
        for output in spec.get("outputs") or []:
            if str(spec["cwd"]) == ".":
                ignored = pathlib.PurePosixPath(str(output)).as_posix()
            else:
                ignored = pathlib.PurePosixPath(str(spec["cwd"])).joinpath(str(output)).as_posix()
            ignored_paths.append(ignored[2:] if ignored.startswith("./") else ignored)
        summary["ignored_paths"].extend(ignored_paths)
        write_prepare_ignored(worker_dir, ignored_paths)

        key = prepare_cache_key(root, spec)
        cache_dir = state / CACHE_DIR / PREPARE_CACHE_DIR / key
        lock_path = state / CACHE_DIR / PREPARE_CACHE_DIR / f"{key}.lock"
        lock_path.parent.mkdir(parents=True, exist_ok=True)
        step_start = time.monotonic()
        step: dict[str, Any] = {
            "name": name,
            "source": spec.get("source"),
            "cwd": str(spec["cwd"]),
            "command": prepare_command_label(spec.get("command")),
            "cache_key": key,
            "cache_hit": False,
            "outputs": list(spec.get("outputs") or []),
            "ignored_paths": ignored_paths,
            "stdout_path": str(step_dir / "install.stdout"),
            "stderr_path": str(step_dir / "install.stderr"),
        }

        with lock_path.open("a+") as lock_file:
            fcntl.flock(lock_file.fileno(), fcntl.LOCK_EX)
            cache_ready = bool(spec.get("cache", True)) and all(
                (cache_dir / str(output)).exists() for output in spec.get("outputs") or []
            )
            if cache_ready:
                for output in spec.get("outputs") or []:
                    copy_prepare_output(cache_dir / str(output), package_root / str(output))
                step["cache_hit"] = True
            else:
                returncode = run_prepare_command(
                    spec.get("command"),
                    package_root,
                    step_dir / "install.stdout",
                    step_dir / "install.stderr",
                )
                step["exit_code"] = returncode
                if returncode != 0:
                    step["status"] = "failed"
                    step["duration_seconds"] = round(time.monotonic() - step_start, 2)
                    summary["steps"].append(step)
                    summary["status"] = "failed"
                    summary["completed_at"] = now_iso()
                    summary["duration_seconds"] = round(time.monotonic() - start, 2)
                    (prepare_dir / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
                    raise PrepareError(
                        f"Prepare command failed for {spec['cwd']}: {prepare_command_label(spec.get('command'))}"
                    )
                if spec.get("cache", True):
                    tmp_cache = cache_dir.with_name(
                        f"{cache_dir.name}.tmp-{os.getpid()}-{random.randrange(1000, 9999)}"
                    )
                    if tmp_cache.exists():
                        shutil.rmtree(tmp_cache)
                    tmp_cache.mkdir(parents=True)
                    for output in spec.get("outputs") or []:
                        source = package_root / str(output)
                        if source.exists() or source.is_symlink():
                            copy_prepare_output(source, tmp_cache / str(output))
                    if cache_dir.exists():
                        shutil.rmtree(cache_dir)
                    tmp_cache.rename(cache_dir)
            fcntl.flock(lock_file.fileno(), fcntl.LOCK_UN)

        step["status"] = "succeeded"
        step["duration_seconds"] = round(time.monotonic() - step_start, 2)
        summary["steps"].append(step)

    summary["ignored_paths"] = sorted(set(summary["ignored_paths"]))
    summary["completed_at"] = now_iso()
    summary["duration_seconds"] = round(time.monotonic() - start, 2)
    (prepare_dir / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    return summary
