# Proposed evaluation gym

## Three independent systems

Separate the **Spindle service under test**, the **agent/project world**, and the
**controller/evaluator**. They can run on one laptop. "Three systems" does not mean
three copies of the entire product per agent. Start with one Spindle instance per
experiment arm and separate identities/workspaces for its agents. Use separate
service state, relational graph data, search indices and volumes for treatment/control arms.

The preferred foundation is NeMo Gym, using its Codex CLI harness with local
Docker and optional Daytona remote sandboxes, after the
[platform qualification spike](gym-platform-evaluation.md). Spindle-owned scenario
control and graders are Rust; NeMo and vendor runtimes remain external tools.
Claude Code and Claude Agent SDK integrations follow the same product contract. Scripted fixtures and recorded replay
remain useful for fast tests but are always labeled as such.

The scenario controller exposes start/step/stop and explicitly qualified resume
semantics: the inspected NeMo Codex session supports one logical activation, so
native multi-turn continuation and crash recovery must not be presumed. Preserve
project state across fresh persona sessions. Capture observable tool/session events, completion,
cost and artifact metadata. Use production MCP tools and upload/injection adapters.
Session completion and partial-crash handling are part of the experiment. The
controller may schedule user prompts, time advances and synthetic external events;
agents do not receive a privileged backchannel to other personas or the grader.

## Shared work without accidental information leakage

Each persona has its own workspace, agent home/session, identity, task narrative
and private knowledge. All work belongs to the same logical project, but filesystem
isolation prevents reading a teammate's private transcript. Publish shared code or
data through an explicit repository/artifact schedule, equally in all comparison
arms. Define whether collaboration happens sequentially or concurrently; do not
let agents racing on one directory accidentally communicate via local state.

Record actual knowledge exposures: memory injected, search results seen, files
read, code commits published, messages delivered. A downstream agent may learn a
fact from code rather than Spindle; do not credit memory for that effect. Synthetic
tools and external systems run as scenario-controlled adapters with specified
responses and fault schedules. Production Spindle functionality remains real.

## First scenarios

| Scenario | Work performed by agents | External grading |
|---|---|---|
| Retry-policy transfer | Architect records legacy/v2 constraints; implementer changes a small service; tester verifies endpoint behavior | Hidden behavioral tests, scope-sensitive retrieval, cited sources and clean rollback |
| Numeric extraction | Analysts discuss thresholds, percentages, currencies, units and effective dates | Exact values/units/denominators/intervals, appropriate abstention, source span alignment |
| ETL/schema drift | Agents build transforms, encounter changed schemas and late data, create a report | Dataset checksums, SQL/data quality tests, declared lineage, reproducible report values |
| BI definition drift | Separate sessions change a metric/cohort definition and update a visualization | Query outputs and artifact metadata; distinguish old/new definition and valid period |
| Privacy and retraction | A session includes synthetic sensitive material; later evidence has independent support | No unauthorized disclosure; full lineage cleanup; bounded residual disclosure; retention accounting |
| Memory aging | Many sessions introduce updates, errors, low-value duplicates and Night Shift passes | Relevance over time, unsupported insight rate, stale advice, independent-source preservation |

Add research/evidence-quality scenarios as blueprint coverage grows. Each scenario
has a versioned seed project, roles/private facts, task sequence, permitted tools,
starting Spindle state, time model, success criteria and artifact retention rules.
Use real small software/data projects whose outcomes can be checked independently,
not conversations whose only success condition is that the agent says it succeeded.

## Experimental design

Use at least disabled-memory versus Spindle-enabled arms with the same initial
project, task schedule, agent/model versions, token budgets and non-memory tools.
Later add retrieval-only, no-Night-Shift, lexical-only, different extraction and
gold-memory diagnostic arms. Reset all persistent state between repetitions;
do not reuse a service containing treatment memories for control.

Pin seeds where supported but do not promise deterministic LLM runs. Repeat
matched scenarios and report paired differences, confidence intervals, raw sample
counts, failures, and cost. A single successful demo proves connectivity, not
product benefit. Predeclare quality thresholds after pilot measurements and freeze
them before the confirmatory comparison. Keep development and held-out scenarios
separate to reduce overfitting.

Measure extraction precision/recall with scoped units/time labels; retrieval
Recall@k and nDCG on judged queries; factual/scope fidelity; supported versus
unsupported synthesis; observed memory use; downstream task pass rate; time,
tokens and cost; retention/retraction invariants; and long-horizon contamination.
Do not collapse these into a single score or treat retrieval as causal benefit.

The observer grades code/data outputs and service snapshots outside agent control.
Use deterministic oracles for invariants and numeric facts. Model judges can help
assess nuance, but must be calibrated against labeled examples, versioned and
reported separately. Builders cannot modify the held-out grader or see its answers.

## Runs and artifacts

Proposed per-run layout: manifest, sanitized event stream, private trajectory
references, state checkpoints, code/data diffs, search exposures, ledger/projection
validation reports, individual grades, usage/cost records and a summary report.
Record service commit, runner version, scenario commit, model/config/prompt hashes,
data seed, treatment assignment, clocks and start/end status. Never request or
depend on hidden model reasoning; capture observable messages/tools and supported
telemetry. Private trajectories have a deletion/retention policy, not Git storage.

Classify product failures, agent task failures, harness faults, provider failures,
timeouts and budget stops separately. Run IDs and checkpoints enable diagnosis;
resuming cannot relabel a failed attempt as though it never happened. Replay checks
the pipeline against captured inputs, not reproducibility of the live agent.

The user's initial live-run ceiling is **$100 and four hours per full run**. This is
a maximum, not a spending target. Default pilot runs should be much smaller. Include
all personas, extraction, embeddings, synthesis, judging, sandbox compute/storage
and relevant egress in accounting. Daytona needs explicit remote cleanup, TTLs
and an external janitor so controller failure does not leave billable sandboxes. Track
usage against a pinned price table with a conservative reserve for in-flight calls;
stop launching calls before exhausting the ceiling and terminate at the deadline.
Bound per-call tokens/time and concurrent sessions. If pricing or usage is unknown,
fail closed or require an explicit prepaid allocation; do not claim an exact hard
dollar cap from delayed provider accounting. Provider-side limits are additional
protection. Record budget/time stops as incomplete, not passing.

## Improvement loop

Run a baseline → identify a failure cluster → create a committed task linked to
trajectory/state evidence → change Spindle → run deterministic regression checks →
compare a fresh matched gym run → retain or reject the candidate. Add a minimized,
sanitized regression case for confirmed failures. Hold-out results gate promotion;
repeatedly tuning against them turns them into development data and requires a new
holdout. Prompt/model changes are versioned software changes, not silent tuning.

The factory builds Spindle; the gym evaluates it. A gym participant never gets to
deploy the product, rewrite its own evaluator, or approve its own candidate. This
separation makes iterative autonomous work inspectable without pretending that
an automated judge is infallible.
