from __future__ import annotations

import importlib.machinery
import importlib.util
import tempfile
import unittest
from pathlib import Path


def load_backend():
    root = Path(__file__).resolve().parents[1]
    path = root / "bin" / "codex-fleet-backend"
    loader = importlib.machinery.SourceFileLoader("codex_fleet_backend_for_tests", str(path))
    spec = importlib.util.spec_from_loader("codex_fleet_backend_for_tests", loader)
    if spec is None:
        raise RuntimeError("could not load backend spec")
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


class RetryTests(unittest.TestCase):
    def test_failed_worker_is_requeued_once_and_original_attempt_is_not_final(self) -> None:
        backend = load_backend()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "repo"
            state = root / ".codex-fleet"
            run_id = "run-1"
            run_dir = state / "runs" / run_id
            root.mkdir()
            run_dir.mkdir(parents=True)

            job = {
                "id": "job-1",
                "title": "fix the deck header",
                "label": "fix-the-deck-header",
                "layer": "default",
                "path": str(run_dir / "jobs" / "job-1.md"),
                "body": "fix the deck header",
                "meta": {},
            }
            worker = backend.worker_base_state(
                job,
                run_dir / "workers" / "job-1",
                state / "worktrees" / run_id / "job-1",
            )
            worker["status"] = "failed"

            run_state = {
                "run_id": run_id,
                "jobs": {"job-1": job.copy()},
                "workers": {"job-1": worker},
                "phase": "workers_running",
            }
            context = backend.RunContext(root, state, run_dir, run_state)

            retry_jobs = backend.queue_failed_worker_retries(context, retries=1)

            self.assertEqual(len(retry_jobs), 1)
            retry_job = retry_jobs[0]
            self.assertEqual(retry_job["attempt"], 2)
            self.assertEqual(retry_job["retry_of"], "job-1")
            self.assertEqual(retry_job["previous_worker_id"], "job-1")
            self.assertEqual(context.run_state["workers"]["job-1"]["status"], "retried")
            self.assertEqual(context.run_state["workers"]["job-1"]["retried_by"], retry_job["id"])
            self.assertEqual(context.run_state["workers"][retry_job["id"]]["status"], "queued")
            self.assertEqual(context.run_state["phase"], "workers_retrying")
            self.assertEqual(backend.failed_final_workers(context.run_state), [])
            self.assertEqual(
                [job["id"] for job in backend.active_jobs_for_prompt(context.run_state)],
                [retry_job["id"]],
            )

            context.run_state["workers"][retry_job["id"]]["status"] = "failed"
            self.assertEqual(backend.queue_failed_worker_retries(context, retries=1), [])


if __name__ == "__main__":
    unittest.main()
