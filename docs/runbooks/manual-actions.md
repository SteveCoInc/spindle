# Remaining manual actions

## Current environment access

The user published the domain additions. The active environment now enforces
`chatgpt.com` and `api.github.com`, and GitHub repository API access succeeded.
PR #1 and note-compatibility PR #4 are merged. Each new commit still needs its own qualification.
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

The user enabled the merge queue and branch rules on `SteveCoInc/spindle`. Verified:
main is protected, PR #1 is merged, merge-group CI exists on main, both checks are
required, and strict branch-up-to-date checking is off. No further manual queue
configuration is currently needed. See [github-governance.md](github-governance.md)
for the live settings and prior integration-permission limitations. PR #4 exercised the queue successfully: both merge-group checks passed and
commit df2e729d1d77aa76f01b6baef8b122ab7e7a90f1 merged without bypass.

## Live NeMo episodes (implementation continues without these credentials)

The pinned NeMo harness requires a configured sandbox-reachable Gym Model Server,
a supported hosted-model credential and known pricing/usage limits. The current
ChatGPT session does not by itself establish that separate model-server transport. The qualification
spike and remaining integration contract are in [gym-spike.md](gym-spike.md).
Keep model keys in the environment secret manager. No paid episode has started.

## Continuous factory dispatch

No manual configuration is needed for the current active chat to pick up tasks,
create PRs and use the merge queue. The active-session loop is in factory.md.
For work to start while no chat is active, a persistent runner and supported model
authentication are still required; no scheduled builder is installed. The unresolved
managed nested-CLI login is not something a laptop login can repair. Do not add a
cron job to this ephemeral workspace or upload its credential cache to GitHub.

If API-backed unattended operation is chosen later, the concrete setup must specify
runner hosting, a secret name and supported provider endpoint, per-task timeout and
factory daily spend cap, followed by a one-task qualification before enabling a timer.
Those choices are not required to continue implementation now, and the gym's $100
budget does not automatically authorize ongoing factory API spend.
