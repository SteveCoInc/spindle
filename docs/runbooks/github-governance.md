# GitHub setup for agent development

Repository: [SteveCoInc/spindle](https://github.com/SteveCoInc/spindle).

## Current verified state

The user applied ruleset **26340388**, active on the default branch, with no bypass
actors. Main is protected; PR #1 is merged and main contains the merge-group-aware
workflow. Both named CI checks are required. Strict branch-up-to-date checking is
off, as appropriate when the queue handles candidate validation. The live queue
uses squash, two concurrent builds, groups of 1–5 PRs, a 5-minute minimum-group
wait setting and a 60-minute check timeout. These are valid user-selected settings;
the one-PR/45-minute values below are conservative recommendations, not a claim
that those exact values are currently configured. Auto-merge, squash-only repository
merging and merged-branch deletion are enabled. Live queue execution is being
qualified with the next note-tool compatibility PR.

## Initial setup history and reusable configuration
The repository is public and organization-owned. The authenticated account has
admin role, but the cloud integration's token receives **403 Resource not accessible
by integration** on repository administration and Actions-settings APIs. Attempts
to create the ruleset, update merge settings and change Actions defaults were all
rejected. None of those settings was applied by this session. At that time, a branch
read confirmed `main.protected = false`; the current verified state above supersedes it. This is a GitHub App
permission boundary, not a missing local login or a network allowlist issue.

## Apply the initial controls in GitHub

1. Open [repository rules](https://github.com/SteveCoInc/spindle/settings/rules).
   Use **New ruleset → Import a ruleset**, if available, with
   [main-bootstrap.ruleset.json](../../.github/governance/main-bootstrap.ruleset.json).
   Otherwise create an active branch ruleset named **Spindle main admission**
   targeting the default branch and set the values below. Leave bypass actors empty.
2. Require pull requests and resolved review conversations. Set required approvals
   to **0**, dismiss stale approvals, and leave required code-owner/last-push approval
   off for this initial single-account setup. Do not require signed commits or
   deployments before those facilities exist.
3. Require these exact GitHub Actions checks: **Rust checks and tests** and
   **Storage integration**. Both have completed successfully on PR #1's earlier
   commits. The JSON pins their source to the GitHub Actions integration (15368).
   Enable **Require branches to be up to date before merging** for this bootstrap
   configuration. Block force pushes and deletion; require linear history.
4. In [General settings](https://github.com/SteveCoInc/spindle/settings), allow squash
   merge only, enable auto-merge and automatic deletion of merged branches, and
   allow updating PR branches. Use PR title/body for squash messages. Equivalent
   values are in [repository-settings.json](../../.github/governance/repository-settings.json).
5. In [Actions settings](https://github.com/SteveCoInc/spindle/settings/actions),
   select read-only contents/packages default token permissions. Leave **Allow
   GitHub Actions to create and approve pull requests** off. Existing CI needs no
   secrets or write token. An organization policy may lock these controls.
6. Keep PR #1 a draft until its implementation is ready for acceptance. Once accepted,
   make it ready and merge it through the required checks under the bootstrap rule.
   This installs the CI workflow on `main`. The setup task does not itself approve
   or merge that implementation; its independent model review remains unresolved.

The repository-settings and workflow-permissions JSON files are API payloads,
not ruleset imports. Only the two `*.ruleset.json` files are ruleset configurations.
These files are desired configuration, not proof that GitHub settings are active.

## Activate the merge queue after CI exists on main

Open the same ruleset and enable **Require merge queue**, or update the existing
ruleset from [main-queue.ruleset.json](../../.github/governance/main-queue.ruleset.json)
using an admin-capable API connection. Do not leave both versions active as
separate overlapping rulesets. Preserve the two required checks and other controls.

Use these starting values:

| Queue setting | Value |
|---|---|
| Merge method | Squash |
| Build concurrency | 2 |
| Minimum / maximum PRs merged per group | 1 / 1 |
| Minimum group wait | 0 minutes |
| Check response timeout | 45 minutes |
| Check grouping | All PRs must pass (`ALLGREEN`) |
| Require branch up-to-date outside queue | Off once queue is required |

Queue validation tests the candidate against the latest base. It replaces repeated
manual branch-update loops; it does not replace the required checks. Public
organization repositories are eligible for GitHub's native queue, subject to the
available organization/product controls. If the setting is absent or the API rejects
it, record that result and use serialized up-to-date PR merging until resolved;
do not call that fallback a native merge queue.

The workflow handles `pull_request`, `merge_group: checks_requested`, main pushes,
and manual dispatch. Feature-branch pushes use PR CI, avoiding duplicate push/PR
runs. Newer PR commits cancel obsolete PR runs; merge-group runs are not cancelled
by that convenience setting. Both jobs use read-only tokens and avoid persisting
checkout credentials. Dependabot configuration proposes weekly Cargo and Actions
updates; it is effective after merging, not merely because this file exists.

After enabling the queue, use a small accepted documentation PR as the first queue
exercise. Check that GitHub creates a merge-group candidate, runs both checks on
that candidate SHA and merges only after they pass. Also qualify a failing check
and a changed PR head/base on a disposable repository before relying on unattended
admission. No queue execution has been claimed or tested by this setup session.

## Agent workflow and trust limits

Agents use isolated branches and draft PRs, commit task/evidence documentation,
run local qualification, and push updates to the same PR. Mark ready only after
acceptance gates pass; enqueue through the native queue (or enable PR auto-merge
where appropriate). Do not merge directly to main, bypass failed checks, count
stale commit evidence, or share writable test state. A merge queue schedules merges,
not agent sessions; no continuous agent scheduler is enabled by these settings.

The current single-account identity cannot independently approve its own PRs.
For autonomous builds with trusted review, install a dedicated builder GitHub App
with narrowly scoped contents/PR access, **without Administration permission**.
Keep repository settings authority with the maintainer. Use a separate trusted
review identity or CI attestation service for independent review; a second model
using the same GitHub account is not a second reviewer identity.

CODEOWNERS names the verified maintainer, `@steveash`, for workflow/factory/grader/spec
and dependency-control files. It is **advisory in the bootstrap configuration**.
After builder and reviewer identities are distinct, enable required code-owner
review for those paths. Expect maintainer involvement for gate changes. If fully
automated approval is required, first qualify a protected external reviewer workflow
and its credentials/cost controls; do not require a nonexistent review check.

Required CI defined in the candidate repository is not tamper-proof: a builder can
propose edits to workflow, tests or graders. This initial setup does not solve that
trust boundary. Protect gate changes with independent ownership/review and move
held-out graders or mandatory trusted workflows outside builder control as the
factory matures. Never grant agents ruleset bypass or Actions approval permission
merely to make a blocked queue proceed. Enable available Dependabot alerts and
secret-scanning/push-protection controls in repository security settings; do not
silently enable paid products or expose model credentials to untrusted PR code.

## If the cloud GitHub connection gains administration permission

Only the organization/app owner can grant permissions the installed GitHub App
actually requests. In organization settings, inspect **GitHub Apps / Installed
GitHub Apps** for this connection and repository access. If the app does not offer
repository Administration and Actions settings permissions, installation approval
cannot manufacture them. Use the browser steps above rather than exporting a
personal token into chat or trying to repair local Codex authentication.

With a separately supported admin-capable `gh` connection, the checked-in payloads
can be applied directly:

```sh
gh api --method POST repos/SteveCoInc/spindle/rulesets --input .github/governance/main-bootstrap.ruleset.json
gh api --method PATCH repos/SteveCoInc/spindle --input .github/governance/repository-settings.json
gh api --method PUT repos/SteveCoInc/spindle/actions/permissions/workflow --input .github/governance/actions-workflow-permissions.json
```

Create only if the named ruleset does not already exist. Inspect its ID via
`gh api repos/SteveCoInc/spindle/rulesets`. Once the bootstrap prerequisites pass,
update that ID with `PUT repos/SteveCoInc/spindle/rulesets/RULESET_ID` and the queue
payload. Re-read repository settings, workflow permissions and the active ruleset
to verify actual enforcement. Successful JSON validation or a role of `admin` is
not evidence the integration can write those settings.

## Validation of this configuration change

Both ruleset payloads, repository settings and Actions permissions validated against
GitHub's official REST OpenAPI request schemas. `actionlint` 1.7.12 accepted the
updated CI workflow. `make check test` also passed (13 Rust tests). Existing check names and GitHub Actions integration ID were
read from successful checks on PR #1; main was confirmed not to contain the workflow.
The Git remote now uses the verified organization URL. These validations do not
establish remote rule enforcement or a successful merge-group execution.

Reference: [GitHub REST API schemas](https://github.com/github/rest-api-description/tree/main/descriptions/api.github.com).
