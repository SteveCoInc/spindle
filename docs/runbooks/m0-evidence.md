# M0 verification evidence

The implementation is on `codex/m0-foundation`. The validated code and pinned
storage images are at commit `ad47930`; later documentation updates do not imply
that independent model review has passed.

Executed in the managed cloud workspace:

- `make check test`: Rust formatting, Clippy with warnings denied, task validation,
  and 10 tests passed. Tests cover claim contention, stale/expired leases,
  persistence, pause, task dependencies, strict review reports, failed/dirty
  qualification, isolated worker handoff, scope rejection, and worker timeout.
- `make up`: digest-pinned PostgreSQL 17.7 and OpenSearch 2.19.4 became healthy;
  migration 1 applied successfully, including repeated application.
- `make test-integration`: a PostgreSQL transaction and actual OpenSearch
  create/index/search/delete cycle passed. Test transaction data is rolled back.
- `make qualify`: a fresh clone of `ad47930efa2ab3e63b629a34815aa89c93f72e5a`
  passed check, test, and storage integration. Logs and `qualification.json` are
  retained locally under `artifacts/<full-commit-sha>/`; independent review is
  explicitly still required.
- `make doctor`: tool availability and both storage connections passed.
- HTTP `/healthz` returned 200; `/readyz` returned 503 while search was absent,
  then 200 when both dependencies were available.

Runner tests use explicit fixture executables. They are not evidence of a live
Codex worker or a successful independent model review. A real authenticated Codex
probe failed because the environment proxy denied `chatgpt.com`; GitHub API access
was similarly denied for `api.github.com`. Git transport worked and the branch
was pushed. Remote CI results and PR creation have not been verified.

The [manual actions](manual-actions.md) identify the network draft that must be
activated before live worker/reviewer validation and PR automation can continue.
No scheduler, automatic merger, AWS deployment, product memory pipeline, or live
gym has been enabled. This is a tested foundation, not completion of M0's live
agent acceptance criteria or the product prototype.
