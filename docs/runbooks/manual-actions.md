# Remaining manual actions

## Current environment access

The user published the domain additions. The active environment now enforces
`chatgpt.com` and `api.github.com`, and GitHub repository API access succeeded.
The implementation has draft PR [#1](https://github.com/steveash/spindle/pull/1).
Foundation CI completed successfully; newer product commits need their own CI.
A live reviewer was attempted after publication. Its model endpoint returned
401 and reported that the saved login could not be refreshed after logout/account
change. Reconnect the Codex login using the platform's supported credential flow
(or `codex login` on your own development machine), then rerun `make review`.
Do not paste credentials into chat or Git. This blocks live review/gym execution;
it does not block continued implementation or deterministic acceptance checks.
No further network draft changes are required by these observations.

## GitHub controls, after first remote CI run

The workflow defines `Rust checks and tests` and `Storage integration`. Once they
have actually appeared and passed on the PR, add them to the default branch's
required checks in Settings → Rules → Rulesets. Enable merge-queue integration if
available for this repository. Do not add a nonexistent AI-review check: protected
CI review authentication is not configured yet. No schedule is necessary for the
current explicit runner, and no automated merge should be enabled before its gates
and review credentials are qualified.

## Live NeMo episodes (implementation continues without these credentials)

The pinned NeMo harness requires a configured sandbox-reachable Gym Model Server,
a supported hosted-model credential and known pricing/usage limits. Repairing the
interactive Codex login alone does not establish that transport. The qualification
spike and remaining integration contract are in [gym-spike.md](gym-spike.md).
Keep model keys in the environment secret manager. No paid episode has started.
