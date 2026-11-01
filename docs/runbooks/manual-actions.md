# Remaining manual actions

## Environment network policy

The current instance can read GitHub via Git and pull packages/images, but its
policy denied both the authenticated Codex endpoint (`chatgpt.com`) and the GitHub
API (`api.github.com`). Existing Codex sign-in was detected; a missing API key was
not assumed. The existing GitHub API binding could not be qualified through the
denied network route.

1. Open this cloud environment's settings.
2. Review the saved draft: it adds `chatgpt.com` and `api.github.com`, preserving
   existing package-manager network presets. No credential values were added.
3. Save the changes and publish the environment as required by the platform to
   activate the draft. The agent has not published it.
4. After reconnection, run the local runbook's toolchain exports, `make doctor`,
   and `make review`. A successful draft save alone does not prove runtime access.
5. Retry `gh api repos/steveash/spindle --jq '{default_branch,permissions}'`.
   If that now returns an authentication error, repair the GitHub connection in
   environment settings. Do not copy credentials into chat or repository files.

Until step 4 succeeds, independent model review and a real Codex worker round-trip
are blocked. Until step 5 succeeds, PR API and repository-settings automation are
unverified. These do not block deterministic local tests.

## GitHub controls, after first remote CI run

The workflow defines `Rust checks and tests` and `Storage integration`. Once they
have actually appeared and passed on the PR, add them to the default branch's
required checks in Settings → Rules → Rulesets. Enable merge-queue integration if
available for this repository. Do not add a nonexistent AI-review check: protected
CI review authentication is not configured yet. No schedule is necessary for the
current explicit runner, and no automated merge should be enabled before its gates
and review credentials are qualified.
