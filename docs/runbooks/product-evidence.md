# Local product evidence and remaining work

Implementation commit: `a8a5de5` (following initial slice `c12de2c`).

Executed `make check test test-integration` successfully. Twelve Rust tests passed,
including factory failure/recovery tests and MCP/privacy checks. The product
acceptance command additionally exercised real HTTP, PostgreSQL, OpenSearch and
a spawned stdio MCP process with separate authenticated test users.

Verified behaviors:

- Owner-only membership changes; outsiders and revoked members cannot retrieve
  evidence, write through old sessions or change project policy.
- Activation snapshots, explicit capture, duplicate delivery and conflicting keys.
- Personal/project privacy precedence; excluded contributions rejected before
  storage, even after project exclusions are relaxed.
- Scoped lexical retrieval and evidence inspection; canonical filtering excludes
  inactive hits while OpenSearch still contains them.
- Immutable manual synthesis, same-project active inputs, a 30-hop chain and a
  shared descendant; retraction retired 31 dependent outputs while preserving an
  independent source and memory.
- Author-only correction, retry returning the same replacement, and projection
  reconciliation reporting completion after the worker drains both outboxes.
- Real stdio MCP initialization and `spindle_status` round-trip to the HTTP API.

Clean-checkout qualification of `a8a5de5` passed check, test and integration;
local logs and the exact-SHA report are under `artifacts/<full-sha>/`.

This is partial M1 evidence, not whole-spec acceptance. Remaining work includes
host transcript spooling/hooks and real Codex/Claude/SDK validation; automatic
extraction/synthesis and embedding providers; entity adjacency; temporal semantics;
broader privacy classifiers/retention enforcement; resumable bounded rollback jobs; all other blueprints;
Briefing/Impact/Night Shift; and the real-agent gym. Formal reasoning stays deferred.
Manual synthesis is labeled as such; lineage alone does not establish factual
correctness. Retraction hides inactive content immediately in this slice; availability
of unaffected published versions during asynchronous rebuild still needs a richer
version/publication model to meet the full specification.

The attempted independent reviewer reached Codex but failed on an invalid saved
login (401/refresh rejected). It did not approve this implementation. The next
manual authentication step is recorded in [manual actions](manual-actions.md).
GitHub API works and draft PR #1 is open. No change has been merged or deployed.

Subsequent recovery/concurrency checks passed: two full acceptance processes ran
concurrently with separate schemas/indices; lost-index reconstruction preserved
active corrected memory and did not resurrect retired sources; independent database
connections raced synthesis publication with retraction and never left an active
unsupported output. Revoked membership also rejected writes through an already
bound session. The NeMo framework spike is recorded separately in [gym-spike.md](gym-spike.md).

Historical privacy remediation tests subsequently passed through real HTTP/storage:
policy changes purged an original plus paraphrased/superseded versions, cleared
canonical text, denied another user's status request, and reported completion only
after projection cleanup. A removed member could still remediate their own prior
contribution. The official external MCP Python client also initialized the Rust
server, discovered all 18 tools, and observed missing-credential failure. This is
protocol compatibility evidence, not an interactive Claude/Codex host smoke test.

The expanded workspace now has 13 passing Rust tests, including calibration of the
trusted retry-policy gym seed: its external oracle detects the intended legacy
retry defect (two oracle cases pass, one fails as expected). No live persona
performance or treatment/control benefit is inferred from that fixture result.

Final code checkpoint `91b4815b5335aa6a8197b06d265d36699c299670` passed clean-clone
qualification (format/Clippy/task validation, all Rust tests, real-storage acceptance).
The exact-SHA qualification report is retained in local `artifacts/91b4815.../`.
Independent model approval remains absent because the reviewer login failed.
