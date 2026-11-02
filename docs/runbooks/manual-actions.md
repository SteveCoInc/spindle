# Remaining manual actions

## Current environment access

The user published the domain additions. The active environment now enforces
`chatgpt.com` and `api.github.com`, and GitHub repository API access succeeded.
The implementation has draft PR [#1](https://github.com/steveash/spindle/pull/1).
Foundation CI completed successfully; newer product commits need their own CI.
A live reviewer was attempted after publication. Its model endpoint returned
401 and reported that the saved login could not be refreshed after logout/account
change. The managed CLI still reports “Logged in using ChatGPT”; that status alone
has not established working authentication for a separately spawned CLI process.
The current chat remains operational. No supported credential-renewal tool or
user-facing reconnect control has been verified for this hosted environment.

No local `codex login` action is required from the user: signing in on a laptop
would not repair this hosted process. Treat the failed nested CLI as an unresolved
runtime integration issue, not an outstanding user configuration step. Continue
implementation and deterministic acceptance checks. Do not reset or replace the
platform-managed authentication files.

For unattended external model calls, an alternative is an explicitly configured
API-backed runtime. If adopted, the user supplies a provider API credential through
the environment's supported secret controls; the agent configures and tests the
runtime and required network destinations. This may have separate API billing and
must retain the agreed budgets. No credential or billing change is required merely
to continue this chat's development work. Do not paste credentials into chat or Git.
No further network change is indicated for the observed ChatGPT 401 failure.

## GitHub controls after organization transfer

The repository is now `SteveCoInc/spindle`. Follow the exact browser/bootstrap/queue
steps in [github-governance.md](github-governance.md). Checked-in ruleset and settings
payloads are ready, but GitHub rejected administrative writes from this cloud
integration with HTTP 403. None of those settings is claimed applied. The existing
CI workflow is still in PR #1, so bootstrap main before requiring the merge queue.

## Live NeMo episodes (implementation continues without these credentials)

The pinned NeMo harness requires a configured sandbox-reachable Gym Model Server,
a supported hosted-model credential and known pricing/usage limits. The current
ChatGPT session does not by itself establish that separate model-server transport. The qualification
spike and remaining integration contract are in [gym-spike.md](gym-spike.md).
Keep model keys in the environment secret manager. No paid episode has started.
