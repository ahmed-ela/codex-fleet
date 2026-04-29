from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from codex_fleet.jobs import (
    clean_job_title,
    compact_prompt_preview,
    create_job,
    job_from_file,
    split_capture,
)


class JobsTests(unittest.TestCase):
    def test_clean_job_title_removes_low_signal_prefixes(self) -> None:
        title = clean_job_title(
            "we still have the issue where its shwoing 'Add deck' in the header"
        )
        self.assertEqual(title, "its shwoing 'Add deck' in the header")

    def test_create_job_writes_label_frontmatter(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            state = Path(tmp)
            (state / "queue").mkdir()
            path = create_job(state, "make sure all local changes pushed", None, "default")
            job = job_from_file(path)

        self.assertEqual(job["title"], "all local changes pushed")
        self.assertEqual(job["label"], "all-local-changes-pushed")
        parts = job["id"].split("--")
        self.assertEqual(parts[1], "all-local-changes-pushed")
        self.assertEqual(len(parts[2]), 6)

    def test_split_capture_uses_separator_lines(self) -> None:
        self.assertEqual(split_capture("one\n---\ntwo\n", "---"), ["one", "two"])

    def test_compact_prompt_preview_summarizes_large_paste(self) -> None:
        preview = compact_prompt_preview("a\nb\nc", limit=80)
        self.assertIn("[pasted: 3 lines", preview)


if __name__ == "__main__":
    unittest.main()
