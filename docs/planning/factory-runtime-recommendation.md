# Recommended unattended factory runtime

Research date: 2026-11-01 America/Los_Angeles. Recommendation only: no schedule,
paid execution, secrets, GitHub App or external host was configured by this research.

## Updated recommendation: use the existing Pro subscription

The user prioritizes existing Pro capacity over separately billed API inference.
For that constraint, prefer **Codex CLI signed in with ChatGPT on a persistent,
user-controlled machine, supervised by a qualified Symphony deployment with its
GitHub Issues adapter**. GitHub retains CI and the native merge queue. Use existing
Rust helpers for repo task eligibility and evidence; avoid writing a second general
scheduler. Symphony is an engineering preview, so first qualify one task and recovery
before increasing concurrency. Its GitHub tracker must respect repository TOML
eligibility rather than dispatch every open issue or treating the phase tracker as work.

The current official Codex README explicitly recommends Sign in with ChatGPT to
use Codex as part of Pro and other listed plans. This confirms subscription-backed
CLI login, not unlimited throughput, pooled subscriptions or an unattended-use SLA.
The official plan-limit and automation web pages remained inaccessible from this
workspace; current detailed limits/terms were not independently verified.

Start with one worker under the user's existing account. Keep authentication on its
host using the supported login flow. Back off on rate/usage limits, pause on auth
failure, preserve checkpoints, and do not automatically fall back to API billing or
rotate accounts. CI/test/queue polling uses no model capacity; reserve Codex for
implementation, bounded repair and substantive review. Reauthentication can still
occasionally be needed; do not promise permanent unattended login.

An existing always-on desktop or small dedicated Linux host is preferable to adding
another managed service initially. Model inference is remote; the host needs capacity
for Rust builds and optional local PostgreSQL/OpenSearch. GitHub-hosted CI can retain
full storage qualification. An always-on host is required for this selected persistent
service design, not for all possible factory designs. Do not export this ChatGPT cloud
workspace's managed credentials to the host or GitHub secrets. The observed nested
CLI 401 in this workspace does not establish that normal Pro CLI login on that host
will fail.

Suggested pilot: install pinned Codex and Symphony; complete supported ChatGPT login
on that host; configure narrow GitHub access and repo-owned workflow; allow one ready
task; verify PR, CI, queue, receipt and next-task pickup; test usage-limit pause and
restart recovery. No new API key or API billing is part of this recommendation.
No software was deployed and no subscription worker was started by this research.

Source: [official Codex README](https://github.com/openai/codex#using-codex-with-your-chatgpt-plan).

## Earlier API-backed alternative

Start with GitHub-hosted Actions and the official openai/codex-action, a dedicated
OpenAI API credential and a repository-scoped GitHub App. Use one builder initially.
Keep Spindle-owned task selection/reconciliation helpers in Rust. Do not build a
new general-purpose swarm daemon before qualifying this smaller execution path.

GitHub supplies durable scheduling and disposable execution. An always-on machine
is not necessary for this option; the earlier claim that one was required was too
restrictive. This hosted chat is not the scheduler and its managed login is not a
credential to export into Actions. API usage is separately billed; no subscription
pooling or unlimited automation entitlement is assumed.

## Alternatives checked

| Option | Fit and limitation |
|---|---|
| GitHub Actions + official Codex Action | Recommended first. Existing CI/queue and no additional server; requires API credentials and modest task/reconciliation glue. Scheduled execution is best effort, not an immediate always-on session. |
| OpenAI Symphony | Existing scheduler, bounded concurrency, retries and app-server integration. Current reference explicitly includes GitHub Issues. Requires an operated persistent host, model authentication and qualification of repo-task/merge policy integration. Engineering preview; restart recovery reconstructs state rather than restoring all in-memory scheduler state. |
| Agent Orchestrator / Orchestrator.inc | Existing long-running daemon, SQLite state, Codex sessions and SCM/CI integration; desktop and hosted-workspace offerings documented. Useful for interactive fleet management. Additional control-plane deployment/qualification; no measured reliability or hosted pricing claim from this review. |
| Custom Rust daemon | Maximum integration control, but duplicates scheduling/recovery work while the product remains incomplete. Retain the existing Rust helpers; defer a bespoke platform. |
| Codex app automations / Copilot agent | Current official web documentation was blocked here. Not independently qualified as a replacement for this repo's Codex task-to-native-queue loop. No assertion of unsupported capabilities or subscription eligibility. |

## Proposed execution contract

1. Run a controller on manual dispatch, selected completion events, and a proposed
   15-minute reconciliation schedule offset from the top of the hour. Scheduling
   is best effort; the periodic scan repairs missed events. Workflows live on main.
2. Read versioned tasks and receipts from accepted main. Git task definitions remain
   authoritative; GitHub issues are mirrors and run/PR metadata carries operational
   status. Never dispatch arbitrary public issue text as an authorized task.
3. Serialize the initial controller/builder with repository-wide concurrency and
   cancellation disabled. Before assigning work, reconcile workflow runs, branches,
   PR heads and merges. Persist attempt ID, task revision, base SHA and fencing
   generation before invoking the worker. Ephemeral SQLite or a cache alone is not
   durable authority; do not simply move the existing local ledger into an Actions
   cache and claim crash-safe scheduling. Parallel worker claims remain a later gate.
4. Run a pinned Codex Action/CLI in a disposable builder job. Preinstall dependencies.
   Its default protection removes sudo and Docker socket access: create services
   before the agent step and qualify explicit loopback access if the builder needs
   integration tests. Do not use unsafe mode merely to regain Docker. Independent
   CI runs the complete acceptance suite in a fresh job with no model credentials.
5. A separate trusted publisher verifies the bounded patch/scope and submits the PR
   using a short-lived GitHub App token. Keep the App private key and publisher token
   out of the builder. Pin trusted workflow/control code; agent changes to gates
   cannot grant themselves authority. Reject artifact paths/metadata that escape the
   attempt or misidentify the tested source.
6. Route failures back to the same attempt with bounded retries. After required
   checks and the configured review policy, enqueue using the expected PR head.
   Never bypass rules. Observe merge-group checks and accepted main before recording
   task completion and selecting another task. Independent model review is not yet
   a trusted required gate; qualify it separately rather than inventing approval.
7. Record durable evidence and reconcile stale ready task definitions. An issue
   being closed is insufficient. Preserve branches/checkpoints on failure; a timed
   out worker does not automatically release its claim for duplicate execution.
8. Start with one task per run, one builder, bounded repair attempts and a suggested
   45-minute worker timeout. Factory daily spend and task limits must be explicitly
   chosen before paid scheduling. Timeouts/run counts are not hard dollar caps;
   record actual usage and enforce admission limits, and qualify a metered gateway
   if a strict in-flight monetary cap is required. The gym's $100 budget is separate.

Use a GitHub App because GITHUB_TOKEN-generated events do not provide an ordinary,
reliable unattended PR-CI chain. Current GitHub source documents event suppression
and, for some PR events/versions, approval-required runs. Do not depend on someone
clicking approval for every factory PR. Qualify actual App-created PR and merge-group
runs in this repository. Limit the installation to Spindle, with contents/PR/issues
permissions needed by the publisher, Actions read for reconciliation, and no branch
rule administration or bypass. Runtime identity and merge authority are separate.

## Rollout and operator configuration

First implement an uncredentialed dry-run controller and deterministic failure tests.
Then create/install the repository-scoped GitHub App and configure Actions secrets
(e.g. SPINDLE_APP_ID, SPINDLE_APP_PRIVATE_KEY and OPENAI_API_KEY; App ID may be a
variable). Create the API credential in a dedicated provider project with appropriate
usage controls. This is a new explicit automation identity, not a repair of ChatGPT.
Do not paste credentials into chat or Git. No local laptop login fixes this hosted
CLI's credentials.

Before enabling the schedule, run one explicitly authorized bounded paid task via
workflow_dispatch through tests, PR, native queue and receipt reconciliation. Test
failure/retry, interrupted runs, stale heads, duplicate dispatch and pause behavior.
Only then enable the timer. Add a second worker after durable claims, workspace/test
isolation and reviewer/queue capacity are qualified. Do not purchase multiple Codex
subscriptions as a prerequisite.

## Authentication diagnosis: verified, narrower than earlier wording

A fresh bounded read-only probe of installed codex-cli 0.163.0 on 2026-11-01
America/Los_Angeles reproduced the nested CLI failure. `codex login status` reported
"Logged in using ChatGPT", but a model request returned HTTP 401 from
chatgpt.com/backend-api/codex with:

> Could not parse your authentication token. Please try signing in again.

The CLI then reported:

> Your access token could not be refreshed because you have since logged out or
> signed in to another account. Please sign in again.

There was no model response. The second message is the CLI's diagnostic, not proof
that the user actually logged out or switched accounts. The endpoint was reached;
this observation is not a network allowlist denial. Exact cause (managed credential
handoff, token format or another runtime integration defect) is unestablished.
This chat's execution and GitHub authentication work independently. The failure
belongs specifically to a separately launched CLI's model transport in this managed
workspace. No credentials were read, copied, reset or changed during diagnosis.

No supported managed-credential repair control was exposed by the available runtime
tools. If nested CLI execution is intended to be supported here, this is a platform
support/integration issue. A non-secret support report can include CLI version,
error text, probe time 2026-11-02T06:35:39Z, and request ID
req_dfac791a9985459680593ae80f3ad94c. Do not conclude that the user's ChatGPT account is
broken. Do not repeatedly request laptop login or silently replace managed credentials.
The recommended Actions runtime has its own explicitly configured authentication
and does not require this managed-runtime problem to be fixed first.

## Sources and evidence limits

Read source documentation, not an operational benchmark of these orchestrators:

- [Official Codex Action README](https://github.com/openai/codex-action/blob/13e9c17417b29e9a276f73ec6ed7fb574137d3c3/README.md): API credential requirement, permission profiles, Docker/sudo restrictions, pinned CLI inputs.
- [Codex Action security](https://github.com/openai/codex-action/blob/13e9c17417b29e9a276f73ec6ed7fb574137d3c3/docs/security.md): trusted triggering and input/configuration boundaries.
- [Symphony](https://github.com/openai/symphony/blob/be10a1b79df723d6d7612b5651c8522704dafb2e/README.md), [reference setup / GitHub adapter](https://github.com/openai/symphony/blob/be10a1b79df723d6d7612b5651c8522704dafb2e/elixir/README.md), [specification](https://github.com/openai/symphony/blob/be10a1b79df723d6d7612b5651c8522704dafb2e/SPEC.md).
- [Agent Orchestrator architecture](https://github.com/OrchestratorInc/agent-orchestrator/blob/d2c997ba935783d2f7105e2f5c6804834fbea9df/docs/architecture.md).
- [GitHub event reference source](https://github.com/github/docs/blob/main/content/actions/reference/workflows-and-actions/events-that-trigger-workflows.md): default-branch schedules and inactivity behavior.
- [GitHub workflow-token event behavior](https://github.com/github/docs/blob/main/data/reusables/actions/actions-do-not-trigger-workflows.md): event suppression and conditional approval-required PR runs.

Official developers.openai.com and docs.github.com pages were denied by the current
network policy. Their GitHub source documentation above was accessible. Current
Codex subscription terms, app automation capabilities and hosted orchestrator prices
were not verified. No network policy was modified.
