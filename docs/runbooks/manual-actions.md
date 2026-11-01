# Remaining manual actions

## Current environment access

The user published the domain additions. The active environment now enforces
`chatgpt.com` and `api.github.com`, and GitHub repository API access succeeded.
The implementation has draft PR [#1](https://github.com/steveash/spindle/pull/1).
Foundation CI completed successfully; newer product commits need their own CI.
Live Codex validation must still be exercised before claiming its review gate works.
No further network draft changes are required by these observations.

## GitHub controls, after first remote CI run

The workflow defines `Rust checks and tests` and `Storage integration`. Once they
have actually appeared and passed on the PR, add them to the default branch's
required checks in Settings → Rules → Rulesets. Enable merge-queue integration if
available for this repository. Do not add a nonexistent AI-review check: protected
CI review authentication is not configured yet. No schedule is necessary for the
current explicit runner, and no automated merge should be enabled before its gates
and review credentials are qualified.
