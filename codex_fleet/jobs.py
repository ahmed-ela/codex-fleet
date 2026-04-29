from __future__ import annotations

import datetime as dt
import pathlib
import random
import re
from typing import Any


QUEUE_DIR = "queue"
JOB_TITLE_LIMIT = 96
JOB_ID_SLUG_LIMIT = 52


class JobError(RuntimeError):
    pass


def now_iso() -> str:
    return dt.datetime.now(dt.timezone.utc).astimezone().isoformat(timespec="seconds")


def compact_time() -> str:
    return dt.datetime.now().strftime("%Y%m%d-%H%M%S")


def slugify(value: str, fallback: str = "job", limit: int = 60) -> str:
    value = value.strip().lower()
    value = re.sub(r"[^a-z0-9]+", "-", value)
    value = value.strip("-")
    return (value or fallback)[:limit]


def parse_frontmatter(path: pathlib.Path) -> tuple[dict[str, str], str]:
    text = path.read_text(errors="replace")
    if not text.startswith("---\n"):
        return {}, text

    lines = text.splitlines()
    end_index: int | None = None
    for index in range(1, len(lines)):
        if lines[index].strip() == "---":
            end_index = index
            break
    if end_index is None:
        return {}, text

    meta: dict[str, str] = {}
    for line in lines[1:end_index]:
        if ":" not in line:
            continue
        key, value = line.split(":", 1)
        meta[key.strip()] = value.strip()
    body = "\n".join(lines[end_index + 1 :]).lstrip("\n")
    if text.endswith("\n"):
        body += "\n"
    return meta, body


def compact_whitespace(value: str) -> str:
    return " ".join(value.split())


def clean_job_title(value: str, fallback: str = "job", limit: int = JOB_TITLE_LIMIT) -> str:
    title = compact_whitespace(value)
    title = title.strip("`*_ \t\r\n")
    title = re.sub(r"^#+\s*", "", title).strip()
    title = re.sub(r"\s+([,.;:!?])", r"\1", title)

    prefixes = [
        r"^(please\s+)?(can|could|would)\s+you\s+",
        r"^(please\s+)?make\s+sure\s+",
        r"^we\s+still\s+have\s+the\s+issue\s+where\s+",
        r"^we\s+still\s+have\s+issue\s+where\s+",
        r"^we\s+have\s+the\s+issue\s+where\s+",
        r"^we\s+have\s+issue\s+where\s+",
        r"^there\s+(is|are)\s+(still\s+)?(an?\s+)?issue\s+where\s+",
        r"^the\s+issue\s+is\s+that\s+",
    ]
    for pattern in prefixes:
        updated = re.sub(pattern, "", title, count=1, flags=re.IGNORECASE).strip()
        if updated and updated != title:
            title = updated
            break

    if len(title) > limit:
        candidate = title[: limit + 1]
        break_at = max(candidate.rfind("."), candidate.rfind(";"), candidate.rfind(","))
        if break_at >= 36:
            candidate = candidate[:break_at]
        else:
            space_at = candidate.rfind(" ")
            if space_at >= 36:
                candidate = candidate[:space_at]
        title = candidate.rstrip(" ,;:-")

    title = title.strip(" \"'`.,;:-")
    return title or fallback


def infer_job_title(body: str, fallback: str = "job") -> str:
    for raw in body.splitlines():
        line = raw.strip()
        if line.startswith("#"):
            return clean_job_title(line, fallback)
        if line:
            break
    return clean_job_title(body, fallback)


def job_label(title: str) -> str:
    return slugify(title, fallback="job", limit=JOB_ID_SLUG_LIMIT)


def make_job_id(title: str) -> str:
    return f"{compact_time()}--{job_label(title)}--{random.randrange(100000, 999999)}"


def compact_prompt_preview(body: str, limit: int = 96) -> str:
    normalized = compact_whitespace(body)
    is_large = "\n" in body or len(body) > 240
    if is_large:
        lines = body.count("\n") + 1 if body else 0
        if lines > 1:
            label = f"[pasted: {lines} lines, {len(body)} chars]"
        else:
            label = f"[pasted: {len(body)} chars]"
        text = f"{label} {normalized}" if normalized else label
    else:
        text = normalized
    if len(text) <= limit:
        return text
    return text[: max(0, limit - 3)] + "..."


def queue_files(state: pathlib.Path) -> list[pathlib.Path]:
    queue = state / QUEUE_DIR
    return sorted(p for p in queue.glob("*.md") if p.is_file())


def flush_queue(state: pathlib.Path) -> int:
    removed = 0
    for path in queue_files(state):
        path.unlink()
        removed += 1
    return removed


def job_from_file(path: pathlib.Path) -> dict[str, Any]:
    meta, body = parse_frontmatter(path)
    job_id = meta.get("id") or path.stem
    title = meta.get("title") or infer_job_title(body, path.stem)
    layer = meta.get("layer") or "default"
    return {
        "id": job_id,
        "title": title,
        "label": meta.get("label") or job_label(title),
        "layer": layer,
        "path": str(path),
        "body": body,
        "meta": meta,
    }


def create_job(state: pathlib.Path, body: str, title: str | None, layer: str) -> pathlib.Path:
    body = body.strip()
    if not body:
        raise JobError("Refusing to enqueue an empty prompt.")

    inferred_title = clean_job_title(title or infer_job_title(body, "job"))
    label = job_label(inferred_title)
    job_id = make_job_id(inferred_title)
    path = state / QUEUE_DIR / f"{job_id}.md"
    content = job_file_content(job_id, body, inferred_title, layer, label)
    path.write_text(content)
    return path


def job_file_content(job_id: str, body: str, title: str, layer: str, label: str | None = None) -> str:
    resolved_label = label or job_label(title)
    return (
        "---\n"
        f"id: {job_id}\n"
        f"title: {title}\n"
        f"label: {resolved_label}\n"
        f"layer: {layer}\n"
        f"created_at: {now_iso()}\n"
        "---\n\n"
        f"{body.strip()}\n"
    )


def split_capture(text: str, separator: str) -> list[str]:
    chunks: list[list[str]] = [[]]
    for line in text.splitlines():
        if line.strip() == separator:
            chunks.append([])
        else:
            chunks[-1].append(line)
    return ["\n".join(chunk).strip() for chunk in chunks if "\n".join(chunk).strip()]
