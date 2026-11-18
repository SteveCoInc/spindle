# AWS Context Spindle interactive prototype — v8

Open **index.html** in a modern browser. No server, installation, API key, model
call, or network connection is needed. Fonts and icons are embedded. Changes are
saved in localStorage when available; restricted previews work in memory.

This is a product design demonstration, not an implemented AWS integration or
an announcement of a launched service. The prototype follows the requested
positioning: Spindle is a new feature of AWS Context. The standalone Rust backend
plan is unchanged.

## Walkthrough

The outer shell has only Introduction, Claude Code, and Spindle UI tabs. Start
with the introduction, then explore the seven sample Claude sessions. Each is a
read-only transcript; its **Try this command** button prefills the interactive
input. Enter and Send execute supported deterministic demo commands.

The example project, **Agent-ready Support Data**, follows Avery, Sam, and Jordan
building an ETL dataset for a support-analysis agent. It demonstrates exact
schema, deduplication, freshness, and PII decisions; voluntary participation;
source-backed notes; corrections; and shared tasks and milestones.

Spindle UI keeps workspace navigation visible at **Your workspace**. Picking a
project adds project navigation. Every project has a **Project Board**, shown as a
folder browser with descendant counts and compact note tiles. Search and
20-item pages keep large boards bounded; clicking an item opens its full text,
attributes, sources, and relationships. Recent activity highlights sit above the
browser. Subscription controls are intentionally absent from this page. No enablement prompt is needed. Project settings includes
personal exclusions and project PII/credential filters. Night Shift explains
maintenance checks, focus, and review behavior.

Claude examples distinguish user turns, responses, tool activity, numbered choice
panels, and passive hooks. The extraction example submits partial-session updates
and records notes asynchronously without asking the user to capture. Users can
inspect, correct, revert, or retract later. Tool-result hooks record observed use;
no manual cite command is required. These are illustrative integration transcripts,
not live shell executions or installed hooks. The typed search demo records its
returned note references as observed use, separately from retrieval exposure.

## Try it

```text
context help
/context new-project
My agent dataset
data
Build a fresh, PII-safe support dataset for agents.
yes
/context note Freshness uses event_time, not ingested_at.
/context board
/context board add task Validate late-event fixtures.
/context impact
```

The creation dialogue asks for a name, blueprint, goal and
confirmation. Use `cancel` to abandon it. `/context create` remains an alias.
`/context use` opens a numbered project picker. Blueprint selection includes a
custom name and extraction focus. `/context impact` shows colored terminal tables separating the whole project from your
notes and corrections. Searching and automatic demo reference recording update the same impact state shown
in the UI. Other useful commands:

```text
/context use Agent-ready Support Data
/context search freshness
/context board move N-103 /decisions/quality
/context board subscribe Watch /delivery tag=validation for status changes.
/context board status B-201 done
/context rollback S-104
```

Use **Reset demo** to restore fixtures. Browser state uses the v8 storage key. On first open it imports available v7 state from the same browser origin. The sandbox fix
uses explicit button and keyboard handlers instead of native form submission,
which is blocked in iframe previews without `allow-forms`.

## Verification

With Playwright and Chromium installed:

```sh
node prototype/smoke.cjs
node prototype/edge-cases.cjs
node prototype/refinements.cjs
node prototype/connections.cjs
```

Set `PLAYWRIGHT_MODULE` and `PLAYWRIGHT_BROWSERS_PATH` if using an existing install.
Tests block external HTTPS and cover CLI input, inert transcripts, guided
creation, hierarchical board state, filtered event delivery, custom blueprints, revert, impact, privacy filters, reserved metadata,
rollback, session subscriptions, archive/restore, reload persistence, and mobile
layout. A separate sandboxed iframe within the smoke test verifies both Enter
and Send with `sandbox="allow-scripts"` and no storage access. Screenshots capture
the introduction, terminal, board, impact, maintenance, and mobile layouts.
These checks validate the browser prototype, not the Rust backend or live hooks.

The optional board subscriptions use a demo notification. The sample steering
hook is illustrative. Night Shift uses prepared examples and does not call a
model, execute ETL, or automatically modify knowledge. Impact records exposure
and explicit use; it does not invent time savings.

Fonts: DM Sans and Manrope, SIL Open Font License; see FONT-LICENSES.txt.

## v6 walkthrough refinements

“Coordinate on the board” follows a delegated validation sub-agent using context
note tools to discover ownership, create a task, watch relevant changes, publish
validation evidence, and leave a handoff. It avoids duplicating Sam’s work and
keeps the release blocked while Jordan’s check is outstanding. No slash commands
are used in this illustrative transcript; no live sub-agent is launched.

The ETL decision links N-104 to the existing AWS Context graph entity for Redshift
table `analytics.support.support_events_gold` through an optional `applies_to`
relationship. Note details let you inspect the illustrative entity metadata and
return to the note. The graph reference is distinct from S-104 source evidence;
it neither copies the table nor proves the note’s claim. Notes without links
remain valid. This demonstrates a product relationship, not formal reasoning or
a working AWS graph connector.

Browser acceptance includes 245 additional test-only notes to verify bounded
pagination and folder browsing. Synthetic scale fixtures are not shipped as demo
content. The introduction retains the requested copy-before-concepts order.

## v7 refinements

Natural requests such as `inspect N-104`, `correct N-104: ...`, `revert N-104`,
and `show me my notes about redacting pii` simulate selection of context tools.
Bare numeric references such as `inspect 1` resolve to N-1 in the active project.
Newly created notes use project-local sequential IDs (N-1, N-2, ...); existing
fixture references remain stable and are skipped by the allocator. IDs are not
reused after retraction. New projects start a fresh sequence.

The privacy walkthrough explicitly distinguishes note IDs from source-session
IDs: `revert the note S-104` opens a withdrawal preview instead of silently
reverting a different note. Version revert restores text; session withdrawal
removes source support. The illustrative table maps each note to its source.

Project guardrails are editable detector records with a name, type (LLM instruction
or Regex), definition, and exclusion/redaction action. `/context privacy` shows
those definitions and prepared example detections without displaying secret
values. Built-in email and YubiKey modhex OTP patterns have deterministic preview
checks. LLM rules use limited deterministic surrogates; arbitrary custom rules
are stored but not executed in this prototype. Detected content is withheld as a
whole rather than claiming production-grade redaction.

Graph references include a labeled static visualization of four neighboring
entities within two hops. It is illustrative metadata, not a live graph query.
Folder, kind, and tag fields offer typed suggestions and accept new values.
Custom attributes have a key/value editor, with advanced JSON on creation;
existing notes can edit organization and attributes without replacing identity
or source history. Attribute identity/permission fields remain reserved.

## Project Connections in v8

In Spindle UI, open **Agent-ready Support Data → Connections**. Search for the events table or paste `https://context.example/wiki/support-contract`, inspect the result and attach it. The source-access selector previews the fixture permissions and personal suggestions for each member. Jordan can read the events table. Avery and Sam can also read the contract. Source access preview applies to Connections and source retrieval throughout the demo.

Try the same flow in Claude Code.

```text
/context sources search Redshift
/context sources retrieve events
/context connections add events
/context sources resolve https://context.example/wiki/support-contract
/context sources retrieve contract
/context connections add contract
/context search --project How should we handle replayed events?
/context suggestions list
/context suggestions inspect quality
/context suggestions accept quality
/context suggestions reject legacy obsolete pipeline, use v2
/context connections list
/context connections remove contract
```

A project search triggers prepared suggestions in the background. Ignore the inline notice and continue typing, or ask “Why was that suggested?” Dismissing the legacy monitor with feedback mentioning v2 prioritizes the freshness contract on that viewer’s list. In blended search, attached sources receive a two-point boost among matching catalog sources. Project-only searches include attached sources and project notes. General searches query the fixture catalog with no project boost. Every source read uses the selected member’s fixture access. All provider results and background discovery run locally.
