# NeMo Gym and Daytona evaluation

Status: upstream contract tests and a two-persona local Docker diagnostic have
now run; see [executed spike evidence](../runbooks/gym-spike.md). Live model/product
episodes and Daytona remain unrun.
The initial proposal did not evaluate these tools. The revised recommendation is
to qualify **NeMo Gym as the evaluation foundation, local Docker as the default
sandbox provider, and Daytona as an optional remote provider**. Build the Spindle
experiment logic rather than another general agent/sandbox platform.

## Evidence inspected

Inspected NeMo Gym commit `0c7ac5119c99e0aa0dfeda7be59157c589680ffd`:

- [Overview](https://github.com/NVIDIA-NeMo/Gym/blob/0c7ac5119c99e0aa0dfeda7be59157c589680ffd/README.md): task datasets, agent harnesses, verification, stateful environments, evaluation/training and observability. Hosted inference is supported; library operation does not require a GPU.
- [Codex harness](https://github.com/NVIDIA-NeMo/Gym/blob/0c7ac5119c99e0aa0dfeda7be59157c589680ffd/responses_api_agents/codex_agent/README.md): runs `codex exec`, borrows the task sandbox, accepts MCP grants, captures events and supports explicit close/cleanup. The documented sandbox session has **one logical activation**; changed requests are rejected. This is not proof of native multi-turn Codex continuation across our scenario schedule.
- [Claude harness](https://github.com/NVIDIA-NeMo/Gym/blob/0c7ac5119c99e0aa0dfeda7be59157c589680ffd/responses_api_agents/claude_code_agent/README.md): runs `claude -p` and defaults to `--bare`, disabling automatic discovery of hooks/plugins/MCP and other ambient configuration. Explicitly wire and test Spindle's tools and capture settings; an out-of-box rollout does not test the installed product integration.
- [Sandbox interface](https://github.com/NVIDIA-NeMo/Gym/blob/0c7ac5119c99e0aa0dfeda7be59157c589680ffd/fern/versions/latest/pages/infrastructure/sandbox/index.mdx), [Docker provider](https://github.com/NVIDIA-NeMo/Gym/blob/0c7ac5119c99e0aa0dfeda7be59157c589680ffd/nemo_gym/sandbox/providers/docker/README.md), and [Daytona provider code](https://github.com/NVIDIA-NeMo/Gym/blob/0c7ac5119c99e0aa0dfeda7be59157c589680ffd/nemo_gym/sandbox/providers/daytona/provider.py): lifecycle, execution, file transfer and provider selection already exist. Daytona image/snapshot selection has provider-specific restrictions; not all capabilities are interchangeable.
- [Daytona access](https://github.com/NVIDIA-NeMo/Gym/blob/0c7ac5119c99e0aa0dfeda7be59157c589680ffd/fern/versions/latest/pages/infrastructure/sandbox/daytona-access.mdx): hosted credentials, sandbox permissions, quotas and organization-level access considerations.
- [SWE-Together environment](https://github.com/NVIDIA-NeMo/Gym/blob/0c7ac5119c99e0aa0dfeda7be59157c589680ffd/environment_servers/swe_together/README.md): an example of multi-step environment orchestration with a simulator and isolated grading. It uses OpenCode; it does not establish equivalent Codex continuation or Spindle's multi-persona memory lifecycle.

The [old Daytona public core README](https://github.com/daytonaio/daytona/blob/main/README.md)
states that maintenance ended and core development moved private in June 2026.
It directs users to the current Daytona organization. This does not establish that
the managed service is unavailable; it means cloning the old core is not a sound
default for maintained local infrastructure. Current hosted SDK/service versions,
pricing, regions and terms still require qualification. Do not use the obsolete
`daytonaio/sdk` repository as an installation reference; it redirects to the old
monorepo. Record the supported SDK package/version during the spike.

## Responsibility split

| Concern | Reuse | Spindle-specific work |
|---|---|---|
| Agent execution and rollout collection | NeMo Codex/Claude harnesses and telemetry | Install real Spindle integration, capture session boundaries, preserve observed memory exposure |
| Task execution and isolation | NeMo Docker/Daytona providers | Persona identities, separate private workspaces, image pinning, allowed service access |
| Evaluation lifecycle | NeMo environment/resource/verifier interfaces | Multi-session project episode, scheduling, retained shared Spindle state and final grading |
| Shared project state | Spindle service + controlled code/data publication | Reset per experiment arm; preserve across personas; don't accidentally isolate every session's memories |
| Quality scoring | NeMo verification/results infrastructure | Rust graders for provenance, numeric accuracy, downstream outcomes, retraction and retention |
| Experiment analysis | Existing rollout artifacts where useful | Matched treatment/control comparisons, longitudinal memory health, regression tasks and uncertainty |
| Spend and cleanup | Provider limits and lifecycle APIs | Aggregate model/sandbox/storage budget, deadlines, crash cleanup and leaked-resource detection |

NeMo Gym and Daytona serve different layers. NeMo supplies evaluation/harness
infrastructure; Daytona supplies remote execution environments. Together they do
not automatically define the shared-memory experiment, its oracles or its causal
comparison. No RL training stack or local model training is required for our goal.

## Options

| Approach | Fit | Decision |
|---|---|---|
| Entirely custom Rust gym | Maximum control; duplicates sandbox, harness and rollout machinery | Fallback only if existing framework fails the required lifecycle/observability contracts |
| NeMo + Docker | Local iteration and CI; uses existing Codex/Claude and sandbox infrastructure | Preferred first qualification target |
| NeMo + Daytona | Remote isolated workspaces and scalable parallel runs | Optional profile after local acceptance; not required for local development |
| NeMo + self-hosted old Daytona core | Additional control-plane maintenance based on an unmaintained public core | Do not adopt as the default |

Keep Spindle code Rust. Run pinned NeMo/Python and vendor agents as external tools;
use config, HTTP and JSONL adapters. Test whether a Rust environment/resource/verifier
service can satisfy the pinned NeMo contracts. If necessary glue must be Python,
surface that exception rather than silently changing the language choice. Do not
rewrite NeMo in Rust or make its internal Python classes the product API.

## Qualification spike and exit criteria

Start a bounded framework/protocol spike in M0 using existing upstream fixtures;
the product-backed acceptance below completes in M2 after M1 exists. An upstream
fixture proves framework operation, not Spindle quality. Live execution needs the
configured credentials and budgets. Pin a released NeMo version and agent/runtime
images after comparing it with the inspected commit. Research on `main` is not a
compatibility guarantee.

1. Run one real Codex session in local Docker with the actual Spindle MCP/capture
   integration, hosted model transport and externally inspected artifacts. Check
   configuration injection, supported model controls and usable session-end data.
   Separately exercise Claude's explicit settings/MCP configuration and capture
   path, accounting for its harness's bare-mode defaults. CLI evidence does not
   by itself validate the separately requested Claude Agent SDK integration.
2. Run two isolated personas against one retained Spindle project. A contributes;
   B can discover the resulting memory. Show that private transcripts and grader
   answers cannot be read by either persona. Test fresh sessions and interrupted
   capture separately from any promised native session resume.
3. Define one experiment episode to span those user sessions. The default per-task
   seed/close/verify lifecycle must not destroy shared memory after A finishes.
   Qualify controller restart and retained service state; upstream agent state is
   documented as process-local, so durable recovery is not automatic.
4. Verify retract/resynthesize/search behavior, then an isolated disabled-memory
   arm with the same shared-code schedule. Grade from outside the agent sandbox.
5. Exercise timeout, failed model call, duplicate delivery, crash, cancellation and
   artifact export. Obtain cleanup evidence; missing observations stay unknown.
6. Repeat a small scenario with Daytona. Its sandboxes must reach an authenticated
   test deployment of Spindle and any NeMo/model server they need; a remote sandbox
   cannot use the developer machine's localhost. Choose a private route or a
   temporary authenticated test endpoint, with no raw database/admin access.
7. Measure added setup burden, startup time, observability gaps and total cost.
   Keep the local route usable without a Daytona account. Adopt the platform when
   these contracts pass; extend an adapter before replacing the whole framework.

The Codex harness documentation requires full access *inside the task sandbox*.
Consequently isolation must come from that sandbox: never run it on the developer
host with broad credentials, Docker socket, grader mounts or unrestricted service
tokens. Daytona credentials stay in the controller, not in personas' environments;
different API keys alone do not establish cross-persona isolation. Verify actual
network/filesystem separation and capabilities on both providers.

The $100/four-hour ceiling applies to the full run including agent and grading
models, Spindle extraction/embeddings, Daytona compute/storage and relevant egress.
Use conservative reservations and a cleanup allowance. Stop new work before the
ceiling, terminate agent execution at the deadline, and use an external TTL/janitor
to remove leaked sandboxes after a controller crash. Log cleanup failures as
incomplete; a killed local process does not stop remote billing.

The subsequent spike installed the pinned external runtime and qualified selected
upstream contracts plus local Docker lifecycle/isolation. This is partial platform
evidence, not a complete Spindle gym or live-agent quality result.
