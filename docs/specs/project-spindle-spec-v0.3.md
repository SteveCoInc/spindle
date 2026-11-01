# Context Projects — Product and Technical Specification

**Status:** Working draft, v0.3 — notes, optional project board, and session subscriptions  
**Date:** November 1, 2026  
**Working feature name:** Context Projects  
**Internal codename:** Project Spindle (not a customer-facing product name)  
**Product context:** Standalone Spindle project knowledge service; “Context” remains the illustrative agent-tool namespace  
**Audience:** Product, design, engineering, security, and developer-experience teams  
**Revision note:** v0.3 adopts **notes** as the common term for previously named memories, adds the optional project board and active-session subscriptions, and adds a data-project collaboration hero scenario. It incorporates the subsequently confirmed standalone Rust/PostgreSQL/OpenSearch direction. v0.2 remains unchanged as the historical source.  
**Source:** The product-design conversation from which this document was requested. This is a proposed feature specification, **not** a statement of released AWS functionality or official AWS API contracts.

> **Product promise:** Start a project in your coding agent, keep working normally, and let useful team knowledge accumulate—with clear visibility, personal privacy controls, evidence, and the ability to correct or retract what was learned.

**New in v0.3:** [Board collaboration hero](#6b-second-hero-scenario-an-agent-team-builds-a-trustworthy-revenue-dataset) · [Board and subscription contract](#30-optional-project-board-and-live-session-subscriptions) · [New acceptance scenarios](#at-25--optional-board-and-one-note-identity)  

**Quick navigation:** [Customer storyboard](#6-golden-path-customer-experience-a-storyboard) · [User stories](#7-user-stories) · [Functional requirements](#8-functional-requirements) · [Architecture](#11-architectural-model) · [Rollback and provenance](#14-provenance-correction-and-rollback) · [Briefing and Impact](#15-feedback-note-effectiveness-and-demonstrating-value) · [Night Shift](#16-night-shift--daily-project-intelligence-maintenance) · [Blueprints](#17-built-in-project-blueprints) · [Agent interface](#18-agent-native-ux-command-grammar-and-tool-discoverability) · [Open questions](#23-open-design-questions-and-brainstorming-backlog) · [Acceptance tests](#24-acceptance-scenarios-and-system-tests) · [Prototype handoff](#25-prototype-handoff-what-the-later-clickable-html-mock-should-demonstrate) · [Formal claims and reasoning — future phase](#28-future-phase-structured-claims-and-formal-reasoning) · [Terminology decision](#29-terminology-decision-and-compatibility)


### Revision contract and compatibility

**Agreed:** “Note” replaces “memory” throughout the product vocabulary. Extracted
notes, authored notes and derived notes share a project knowledge model; the
optional board organizes those notes without introducing another kind of content.
Blueprint-governed attributes, flexible custom attributes, folders, tags, valid time,
dependencies and session subscriptions are part of the non-formal-reasoning scope.

Existing `MEM-*` requirement IDs and AT-01–24 are retained for traceability; the
`MEM` prefix is a historical identifier, not a second object type. New requirements
use `BRD-*` and `SUB-*`. Proposed APIs use `context.notes.*` and `noteId` terminology.
Existing implementation names such as `publish_memory`, `note_versions` and
`context.memories.*` require a compatibility/migration plan: accept documented
legacy aliases during transition, return canonical note names with versioned
schemas, and preserve object IDs, lineage, events and idempotency semantics.
This specification does not rename deployed storage or claim any new tool shipped.

**Confirmed implementation baseline:** Standalone Rust services, PostgreSQL as the
canonical authority for notes, lineage and board state, and OpenSearch as a
rebuildable retrieval projection. Local-first, both Codex and Claude Code/SDK,
Codex gym first, NeMo/Docker qualification with optional Daytona; AWS comes later.
Aurora DSQL is a candidate requiring qualification, not a drop-in assumption.
Dedicated graph engines, Cypher/SPARQL endpoints and formal reasoning remain deferred.
Historical AWS reasoning references in §28 are future design background, not cloud
availability guarantees. Blueprint attribute validation is not formal reasoning.
The repository [delivery plan](../planning/plan.md) governs implementation staging;
this document specifies desired behavior, not completed acceptance coverage.


---

## 1. Problem and opportunity

Developers and other knowledge workers make consequential decisions while working with agents, but the reasoning, constraints, discoveries, and corrections from those sessions are scattered across ephemeral conversations and individuals' context. A shared context graph can retrieve information, but it does not by itself define which observations matter to a particular collaborative effort, whose knowledge should be combined, how changes over time should be handled, or how an individual can inspect and retract something contributed accidentally. **Context Projects** provides an agent-native project knowledge and collaboration boundary: a project has a goal, members, and optional steering instructions; associated agent sessions contribute traceable project notes; retrieval blends those notes with the user's broader accessible graph; and maintenance, feedback, and correction continuously improve the shared knowledge. The experience should be almost effortless to begin and progressively reveal its advanced controls only when needed.

### 1.1 One-minute product description

A user with a configured Spindle instance opens Claude Code, Codex, or another supported agent, asks to create or join a project, and optionally selects a blueprint such as **Software Development**. Context activates that project for subsequent sessions, visibly confirms that activation, and ingests session trajectories through plugin hooks. It extracts relevant decisions, facts, constraints, lessons, and inferred insights into a shared project knowledge layer. Teammates get more useful answers and briefings because their agent can retrieve project knowledge alongside other authorized context. Users can inspect their contributions, correct mistakes, or roll back an entire session and its derived effects. A daily **Night Shift** maintains and improves the project knowledge. A **Briefing** explains what changed, and **Impact** shows when contributions actually helped or led others astray.

### 1.2 Standalone system and boundary

Spindle includes its own context/project backend, identity and membership boundary,
notes, retrieval, lineage, feedback, correction and maintenance. There is no assumed
existing AWS Context backend. A local context instance groups authorized projects
and general knowledge; later adapters may integrate external sources.

Start with one-machine Rust services, PostgreSQL and OpenSearch. PostgreSQL owns
canonical versions, source identity, relationships, policies, board state and
transactional events. OpenSearch supplies retrieval, not authoritative ownership or
rollback state. A note needs complete provenance whether or not it has semantic
entity links. The optional board and subscriptions use the same backend.


---

## 2. Product tenets and design decisions

### 2.1 Non-negotiable experience principles

1. **Simple from the first minute.** A user should need to understand only *project*, *join/activate*, and *shared notes* to see value. Everything else is optional or discovered at the moment it is useful.
2. **Agent-native, conversational, and self-discovering.** The full workflow should be possible from Claude Code or Codex through natural language, tools, and a small number of memorable slash commands. A new web console is not required.
3. **Visible participation, no surprises.** A project remains active for future sessions after activation, but users must have an obvious indication of the currently active project and its contribution behavior.
4. **Value must be observable.** Show what changed, which notes were used, which teammates benefited, and where knowledge caused confusion or was corrected. Avoid claiming impact when only retrieval exposure was measured.
5. **Personal privacy takes priority.** Global personal exclusions apply to *every* project and cannot be relaxed by project or blueprint instructions. Projects may impose additional restrictions.
6. **Shared knowledge is editable and attributable.** Members can see project notes; editing rules are configurable. Contributors can audit their own sources and retract accidental contributions.
7. **Evidence survives synthesis.** Keep original extracted source observations separate from canonical/synthesized notes, with provenance sufficient to remove an input and re-synthesize its descendants.
8. **Temporal truth matters.** A newer observation need not invalidate an older one if their valid time, environment, or scope differs.
9. **Maintenance improves rather than invents authority.** Night Shift can infer new insights, but every inferred insight remains explicitly labeled and grounded in evidence.
10. **Advanced machinery stays in the background.** Blueprints, defaults, and progressive disclosure should hide the complexity of policies, ontologies, graph traversal, and rollback plans.
11. **Reasoning is additive, not a new burden.** Any later formal-claim layer supplements natural-language note; inference must remain qualified, evidence-backed, reversible, and invisible by default.

### 2.2 Recorded decisions versus proposals

This document uses three labels:

- **Agreed:** Directly established in the conversation.
- **Proposed:** A concrete design recommendation to make the agreed behavior implementable.
- **Open:** A decision that requires validation, prioritization, or further brainstorming.

**Agreed:** Each project is associated with one context instance provided by the standalone backend. A project note belongs to **one project only** for now. Members of a project can read its project notes, including those contributed by other members. Membership and collaboration controls are deliberately simple in the first release. A user activates a project once, rather than opting in for every session; later sessions remain associated by default until activation is changed. Original extracted contributions must be retained as individually traceable evidence for as long as they are valid inputs. Shared or derived notes must preserve their input lineage. The plugin must clearly show the active project. Rollback has full, bounded, and bounded-plus-best-effort-semantic variants. Pending rollback does not make all affected notes unsearchable; progress and incomplete-removal risks are visible to the initiator. Project and personal steering, Night Shift, and built-in blueprints are part of the desired feature.

**Proposed:** Store the active-project selection per user and agent workspace, with one selected project at a time for that workspace, so unrelated repositories are less likely to be captured accidentally. Preserve it across restarts and show it at every session start, plus in a status surface if the host supports one. The exact activation scope remains an open implementation choice; whatever scope is selected must meet the agreed persistent-and-visible behavior.

---

## 3. Goals, success criteria, and non-goals

### Goals

- Capture and refine useful *project-specific* knowledge from agent work with essentially no extra manual documentation burden.
- Give project members relevant answers that combine current project notes with permitted context from the existing graph.
- Preserve separate source contributions when multiple people reach overlapping or partially conflicting conclusions; synthesize the best nuanced shared note without erasing evidence.
- Make project notes temporally aware, correctable, auditable, and reversible from a single session through arbitrary derivation depth.
- Demonstrate concrete individual and team value through briefing, impact, and feedback.
- Support different project types out of the box and allow optional ongoing steering without building a configuration-heavy product.

### Experience success criteria (targets to validate, not promised metrics)

- A user with an installed Context plugin can create and activate a project in a short conversational exchange; role and advanced settings can all be skipped.
- The active project is obvious on entry to every associated agent session and immediately discoverable later.
- The first meaningful session results in reviewable extracted notes without requiring a separate application.
- A teammate can retrieve a relevant project decision, its temporal qualifier, and its supporting evidence while working in a different session.
- An author can preview the impact of rolling back an accidentally associated session and follow the resulting job to completion.
- Briefing and Impact distinguish *note produced* from *note retrieved*, *note actually used*, and *note later found misleading*.

### Non-goals for the initial scope

- Requiring an existing AWS backend, a dedicated graph engine, or cloud deployment for local use.
- Automatically sharing the same note into multiple projects; one project owns one note.
- Enterprise-wide cross-project governance, elaborate classification/approval hierarchies, or fine-grained inherited source-ACL enforcement on each shared project note.
- Requiring users to manage sessions, extraction rules, or provenance graphs manually to get started.
- Presenting unverified inferred insights as established facts.
- Building a standalone web UI as the primary control surface. A later interactive HTML prototype is a design artifact, not a prerequisite for the product.

---

## 4. Core concepts and mental model

| Concept | What the user needs to know | Behind the scenes |
|---|---|---|
| **Context graph instance** | Their knowledge base | PostgreSQL authority + OpenSearch retrieval, with logical relationship traversal |
| **Project** | A shared space organized around a goal | Stable ID, graph-instance association, membership, policies, blueprint snapshot |
| **Active project** | The project their agent work is contributing to | Persistent user/agent activation binding; snapshotted on each session |
| **Project member** | Someone who can benefit from and contribute to shared notes | Membership, role, and collaboration mode |
| **Blueprint** | A sensible starting point, e.g. Software or Research | Default ontology, extraction, curation, insights, Night Shift, and retrieval settings |
| **Project board (optional)** | Organized shared notes for stateful collaboration | Placements, attributes, tags, typed dependencies and revision checks |
| **Session subscription** | Updates about notes this active session is watching | Scoped selectors, durable cursors and host-specific steering delivery |
| **Original note** | Something Context learned directly from a source session | Immutable/versioned extracted evidence with source/session identity |
| **Shared/canonical note** | The reconciled project understanding | Versioned synthesis supported by one or more originals or derived inputs |
| **Briefing** | What changed since I last worked here? | Personalized summary of new decisions, corrections, unresolved conflicts, and Night Shift |
| **Impact** | Where did project knowledge help or mislead? | Evidence-based attribution using retrieval, usage, feedback, and later contradictions |
| **Night Shift** | Daily knowledge maintenance and insight discovery | Scheduled, auditable, project-steered maintenance job |
| **Rollback** | Undo information learned from a session or note | Impact preview, provenance traversal, targeted retirement/re-synthesis, progress tracking |

A typical user does **not** need to distinguish original from canonical note or understand provenance DAGs until inspecting or correcting information. Those distinctions are essential in the system, but should not dominate the product language.

---

## 5. Personas

- **Architect or project lead:** Sets goals, makes design decisions, reviews implementation, contributes constraints and architectural reasoning more than code details.
- **Implementing engineer:** Uses Claude Code to modify modules/services, learns of previous decisions through retrieval, contributes discoveries and implementation nuances.
- **Collaborator in another discipline:** Researcher, ETL engineer, data analyst, or business-intelligence analyst using a different blueprint and vocabulary.
- **Project owner/admin:** Invites members, selects collaboration model, steers shared extraction and Night Shift, audits changes and runs maintenance.
- **Privacy-conscious contributor:** Needs global rules, visibility, and the ability to retract a mistaken session after the fact without silently exposing private source traces.

---

## 6. Golden-path customer experience: a storyboard

The following is the **hero scenario**, deliberately written as a customer journey rather than a backend tutorial. Example names, projects, outputs, and syntax are illustrative product-design proposals.

### Scene 0 — A configured project knowledge service

**Avery** uses a configured local Spindle instance and an installed Claude Code or
Codex integration. Their team is modernizing a billing API; decisions are scattered
across sessions and reviews. The project board is disabled and is unnecessary for
this first hero scenario. The local context instance is the parent resource in
which Avery can create a project. Hosted model calls may be used; AWS is not required.

### Scene 1 — Create the project with one request

Avery says:

```text
Create a Context project called "Billing API Modernization" using the
Software Development blueprint. Our goal is to move billing requests to
the new gateway without introducing duplicate charges. Remember key
architecture decisions, constraints, API contracts, and unresolved risks.
```

Claude Code uses Context tools to create the project, associates it with Avery's configured context instance, and activates it.

```text
✓ Created: Billing API Modernization
  Blueprint: Software Development
  Context: Your existing engineering graph
  Status: ACTIVE for future Claude Code sessions in this workspace

I'll capture relevant project knowledge from your sessions. Extracted
project notes will be visible to members you invite. Personal privacy
exclusions are on by default.

Optional: What's your role, and what kinds of things do you want your
sessions to contribute? You can skip this.
```

**Time to value:** No ontologies, graph schema, policy editor, or onboarding wizard. The blueprint supplies reasonable defaults; the goal gives the extraction system a focus.

### Scene 2 — Optional personal contribution focus

Avery replies:

```text
I'm the technical lead. I'm mostly reviewing architecture, security,
reliability and major tradeoffs. Prioritize decisions and rationale from
my sessions, not routine implementation details.
```

This becomes **Avery's contribution focus for this project**. It does not change the project's shared goals or other members' extraction settings. If Avery skips this, extraction still works. The system may later suggest refinements based on useful or rejected contributions; it cannot silently loosen privacy exclusions.

### Scene 3 — The project is unmistakably active

On the next Claude Code session, the plugin surfaces:

```text
Context project: Billing API Modernization  ● ACTIVE
New sessions contribute project notes. /context status | /context leave
```

Ideally this appears in a host-supported status line or persistent agent UI. If the host cannot support that, the plugin must present an explicit session-start announcement and make the current project quickly retrievable. A transient announcement alone should be tested for sufficient visibility.

**Important:** Avery does not activate the project separately for each session. An explicit switch or leave command changes the future default. The session records which project was active *when that session started* so later switches cannot silently reassign old work.

### Scene 4 — Work normally; an architectural decision becomes note

Avery and Claude Code review gateway retry behavior. They decide:

```text
Until the new gateway issues durable idempotency keys, do not retry
non-idempotent charge-creation requests automatically. Read-only calls
can use bounded exponential-backoff retries.
```

Avery continues working; there is no special save gesture. When the session ends, the plugin's session-end hook submits the trajectory or a configured, filtered representation. Context applies global personal exclusions, project policy, and Avery's contribution focus. It extracts original observations including the decision, rationale, scope, and time, then creates or updates an associated shared note. It links the note to existing service entities when those references exist in the graph.

Later, `/context notes mine --recent` might show:

```text
From your last session: 3 proposed notes, 2 accepted, 1 merged.
• Decision: No automatic retries for non-idempotent charge creation
  until durable idempotency keys are available. [source session S-104]
• Constraint: Read-only gateway requests may use bounded backoff. [S-104]
• Existing shared note updated: Gateway reliability policy. [S-104]

Review or correct any item: /context notes mine
```

The actual CLI need not expose every stage by default; this is a discoverable audit view.

### Scene 5 — A teammate gets the answer at the right moment

Avery invites **Sam** and **Jordan** to the project. Sam joins from Claude Code with an optional contribution focus such as “gateway implementation and integration testing.” Project notes are shared with members even if their original source authors are different.

A few days later Sam asks Claude Code:

```text
Can we automatically retry failed charge-creation requests?
```

Context searches the active project's notes **and** the broader graph Sam is already allowed to access. It finds the project decision, prioritizes its relevance, and traverses graph links to related gateway components where available. Claude Code replies in substance:

```text
Not currently for charge creation. The project decision is to avoid
automatic retries of non-idempotent writes until durable idempotency
keys are in place. Read-only requests can use bounded exponential
backoff. This was decided in the gateway review on Oct 29.

Source: Billing API Modernization / Gateway reliability policy
Related context: BillingGateway, ChargeCreationHandler
```

**Moment of value:** Sam avoids implementing retry logic that could create duplicate charges, without knowing which teammate made the decision or which agent session contained it.

### Scene 6 — Nuance, disagreement, and time are preserved

Jordan's integration-testing session discovers that one new gateway operation **does** now support durable idempotency keys. That is not automatically a contradiction to the earlier statement: it may be a new capability valid only for one endpoint and only after a particular deployment.

Context records both original observations, checks their scope and valid times, and synthesizes a qualified project note:

```text
Charge-creation retries remain disabled for legacy endpoints.
For gateway v2 create-charge requests with confirmed idempotency-key
support, bounded retries may be enabled after rollout verification.
```

If rollout status or evidence is insufficient, the combined note explicitly says **unresolved / needs review** rather than manufacturing certainty. Both contributors' source evidence remains traceable.

### Scene 7 — Return to the project: `/context briefing`

After a few days away, Avery enters Claude Code and asks:

```text
/context briefing
```

The briefing answers what changed since Avery last worked on the project:

- The gateway v2 idempotency capability is new; retry guidance now differs by endpoint.
- Jordan confirmed a new API constraint in integration testing.
- An older retry policy is superseded for a specific operation, not globally.
- Night Shift found an unresolved risk involving retry behavior in a background worker.
- Two notes await review because new evidence partly conflicts.

Every claim can be opened to its project note and evidence. The briefing is not merely a list of documents; it is an actionable orientation to the project's evolving state.

### Scene 8 — See value: `/context impact`

Avery asks:

```text
/context impact
```

An example response:

```text
Your contribution impact — last 30 days
• 12 original notes accepted; 7 informed shared canonical notes
• 19 teammate retrievals exposed those notes
• 6 cases include a later explicit citation or positive feedback
• 2 decisions were updated as new evidence arrived
• 1 retrieved note was later flagged misleading and corrected

Most reused: Gateway retry policy (used by Sam and Jordan)
Team insight: Reusing this decision helped align gateway implementation
and tests. Exact time saved cannot be established from retrievals alone.
```

**Moment of value:** Avery can see useful reuse and correction signals. A retrieval impression is not falsely presented as proof of successful downstream action; later evidence can confirm usefulness or reveal a misleading outcome.

### Scene 9 — Audit, correct, or undo an accidental session

Avery realizes a previous session was mistakenly associated with the billing project. They say:

```text
Show my sessions that contributed to Billing API Modernization yesterday.
I think session S-108 should not have been part of this project.
```

Context shows the session and resulting original/shared notes. Before changing anything, it presents a **rollback preview**:

```text
Session S-108: 4 original contributions
Directly supported: 3 shared notes
Transitive dependencies: 27 derived notes across 8 levels
Other independent sources support: 2 of the affected shared notes

Choose:
1. Full recursive rollback (complete active-note cleanup; slower)
2. Bounded rollback (choose depth; deeper influence may remain)
3. Bounded + semantic leakage scan (best effort beyond depth)
```

Avery can inspect the implications and start the job. The system retracts S-108's original inputs, re-synthesizes notes with their **remaining clean evidence**, and retires unsupported ones. Avery watches progress and any failures; other members' unrelated evidence is not discarded. During processing, notes remain generally searchable, as agreed, but Avery sees that the rollback is **not yet complete** and that sensitive information may remain until processing finishes.

### Scene 10 — Night Shift maintains the shared understanding

Overnight, **Night Shift** reviews the project's goals, new notes, stale facts, temporal changes, contradictions, and retrieval feedback. It may consolidate overlapping notes, deprecate stale ones, correct well-supported errors when policy permits, and **propose or publish novel inferred insights** with explicit evidence and confidence.

In the morning, the briefing includes:

```text
Night Shift insight (inferred; review suggested):
The background reconciliation worker appears to assume retry behavior
that differs from the gateway v2 policy. Compare the worker's retry
configuration with the rollout decision before enabling automatic retries.

Supported by: Gateway retry policy; reconciliation-worker note
```

An owner may run Night Shift on demand or steer what it should watch for. Avery still does not need to learn the job's internal implementation to benefit.

### Scene 11 — The same mental model works outside software engineering

A researcher creates a **Collaborative Research** project with a blueprint tuned for claims, methods, evidence, uncertainties, and competing hypotheses. An ETL team chooses **Data & Pipelines** for data lineage, schemas, transformations, and data-quality findings. A BI team chooses **Business Intelligence** for metrics, definitions, assumptions, and evolving insights. In all cases, the interaction remains: create or join → work normally → retrieve/brief → inspect/correct as needed.

---

## 6B. Second hero scenario: an agent team builds a trustworthy revenue dataset

### Scene 1 — Enable coordination without changing the basic project

Morgan creates **Daily Revenue Reliability** with the Data Engineering blueprint.
The project already extracts useful notes from ordinary work. Morgan asks:

> “Enable a board for this project. Organize milestones, pipeline work, and insights.
> Keep the revenue dataset's definitions and validation evidence easy to find.”

The agent enables the optional board and creates `/milestones/daily-revenue`,
`/pipelines/orders`, and `/insights/revenue`. Existing extracted notes stay where
and as they were until explicitly placed; their IDs and evidence do not change.
Three independent active sessions—Avery handling ingestion, Sam implementing
transforms, and Jordan validating the dashboard—share the project through tools.
Their private workspaces and transcripts remain separate. Morgan selects collaborative
editing for operational task/milestone notes; source-note correction retains the
project's author-controlled policy. Both policies are explicit blueprint settings.

### Scene 2 — Turn the goal into shared state

Avery creates a milestone note: **Publish a validated daily revenue dataset**.
Its acceptance criteria require a repeatable transform, reconciliation against a
known fixture, and documented refund/currency rules. Three task notes track source
contract validation, transformation, and dashboard validation. `depends_on` links
order the work; they are not evidence that any financial statement is true.

The blueprint validates task `status`, `assignee`, and dataset references. The team
adds a custom `backfillWindowDays: 14` field without requiring an ontology redesign.
Each note has a stable ID, a folder, tags such as `milestone:daily-revenue` and
`schema`, and attribution. The agents list pending work under `/pipelines/orders`
and query notes carrying the milestone tag. An atomic claim prevents two agents
from both winning the same transform task; a losing agent rereads the updated state.

### Scene 3 — Subscribe before taking independent steps

Sam subscribes to notes under `/pipelines/orders` carrying both the milestone tag
and `schema`. Jordan subscribes to the milestone note ID and separately to
`quality`-tagged notes. Registration returns the initial authorized snapshot and
cursor, so a change concurrent with registration cannot be lost. The harness
reports whether delivery is live steering or only at the next turn/tool boundary.
The agents continue working without a special agent-to-agent chat channel.

### Scene 4 — Preserve a time-qualified discovery and notify the right worker

Avery discovers that `gross_amount` in schema v1 is measured in cents, while schema
v2, effective November 1, is measured in major currency units. An extracted source
note records the evidence, observation time, schema scope, and valid periods. Avery
places it under `/pipelines/orders/contracts`, adds the subscribed tags, and links
it to the ingestion task. A related insight note explains the risk of a 100× error;
its derivation cites the exact source-note version.

Sam's harness delivers:

> “Project note changed: Orders amount units, revision 3. Matches your schema watch.
> Read the current note before continuing the transform.”

This is a change notification, not a command from Avery or an instruction to trust
the note uncritically. Sam fetches the authorized note, checks its evidence and
valid time, and implements version-aware conversion. The source-contract task is
marked done with a validation artifact. The dependent transform task becomes ready
under the blueprint's declared rule, rather than because a notification was read.

### Scene 5 — Track actual work, conflicts, and progress

Sam records the transform commit and fixture results on the task note. Jordan
receives the relevant change and validates the published dataset and dashboard.
Two updates based on the same old revision cannot silently overwrite each other:
a stale update returns a conflict, and the agent refetches before retrying. Task
claims do not expire into false completion; an expired/reassigned claim rejects a
stale worker's later completion attempt.

Jordan finds that refunds are counted twice and creates a `quality`-tagged pending
work note with failing-query evidence. The milestone remains incomplete. A proposed
fix note can have flexible investigation attributes while retaining the blueprint's
required task fields. A notification about new work does not itself assign that
work, execute a tool, or mark the milestone accepted.

### Scene 6 — Night Shift refines knowledge without inventing progress

Night Shift flags an old “all amounts are cents” note as stale, preserves its
historical v1 applicability, and proposes clearer scope. It detects an abandoned
quality task and a milestone whose done-state claim lacks current validation.
It creates an evidence-backed review note or policy-authorized `needs_recheck`
update. It does not fabricate a passing test, seize a live lease, or silently close
someone else's task. These accepted note changes use the same event stream.

### Scene 7 — Correct a source without destroying unrelated work

Avery retracts a mistaken test-environment observation. The system previews actual
derivation descendants, rebuilds or retires unsupported insight text, and preserves
independent production evidence. Operational dependency edges do not erase the
transform task's separately evidenced commit history. If its completion assertion
relied on that observation, the affected assertion is withdrawn/revalidated and
the task is visibly marked for review under policy. Subscribers receive safe
retraction/update notices and refetch; purged text is not replayed from old envelopes.

### Scene 8 — Reconnect, complete, and learn from the episode

Sam's session briefly disconnects. Within its lease it resumes after its last
acknowledged cursor, deduplicating events. A move out of its watched prefix produces
a leave-selection notice; an expired cursor requires a refreshed snapshot. When
the session ends, its subscriptions expire. A later session explicitly subscribes
again rather than inheriting invisible watches.

Jordan attaches passing reconciliation results, and an authorized revision-checked
transition completes the milestone. Briefing reports the accepted work, unresolved
risks, and updated insights with evidence. Impact distinguishes delivered changes,
observed note reads, and validated downstream fixes; it does not attribute success
to the board merely because a notification was emitted.

**Gym evidence for this scenario:** isolated real-agent sessions, a versioned seed
ETL/data project, externally checked query/dataset outputs, recorded note reads and
injected events, conflicts/reconnects, and a source-retraction test. Compare matched
board-enabled and board-disabled arms while holding ordinary note retrieval,
models, task prompts, and shared-code publication constant. An additional
notifications-disabled arm can isolate the effect of steering. A board-disabled
project must still complete ordinary note capture/retrieval workflows. This
scenario is a product contract, not a report of a completed live run.

## 7. User stories

| ID | Persona / user story | Acceptance intent |
|---|---|---|
| US-01 | As a user with a configured Spindle instance, I can create a project in Claude Code without learning a new UI. | One conversational flow creates a project linked to a context instance; advanced settings are optional. |
| US-02 | As a creator, I can select a built-in blueprint and provide a goal. | Useful extraction defaults exist before manual customization. |
| US-03 | As a contributor, I can join and stay active across future sessions. | No per-session opt-in; clear persistent status and explicit leave/switch. |
| US-04 | As a contributor, I can optionally describe my role and expected knowledge contribution. | My extraction is better targeted without affecting another member's focus. |
| US-05 | As a privacy-conscious person, I can define global never-share rules. | Rules apply in all projects and have higher priority than project defaults. |
| US-06 | As a member, I can read project notes contributed by teammates. | The project sharing boundary is clear, simple, and visible at join time. |
| US-07 | As an engineer, I get project-relevant knowledge during normal graph search. | Blended retrieval boosts relevant project context without ignoring the wider graph. |
| US-08 | As a researcher or analyst, I can search only this project's knowledge when desired. | An explicit project-only search mode is available. |
| US-09 | As a contributor, I can see which sessions and notes I contributed. | Each original note has a source session and accountable author. |
| US-10 | As a member, I can flag misleading knowledge. | Flag creates feedback and can trigger correction/review. |
| US-11 | As a contributor, I can correct or retract my own information. | Shared syntheses are updated and source evidence is not silently overwritten. |
| US-12 | As a project owner, I can choose author-controlled or collaborative corrections. | Permissions are enforced and all changes auditable. |
| US-13 | As a contributor, I can retract an entire accidentally included session. | Preview shows transitive blast radius; all supported rollback modes are offered. |
| US-14 | As a contributor, I can choose how exhaustive rollback should be. | Full, bounded, and bounded + semantic scan state costs and guarantees accurately. |
| US-15 | As an auditor, I can follow source/derived note lineage. | Original evidence and transformations remain available where authorized. |
| US-16 | As a team member, I can ask for a project briefing. | Includes relevant changes since my last interaction and new Night Shift insights. |
| US-17 | As a contributor, I can see how my notes affected my team. | Impact separates exposure, demonstrated usefulness, corrections, and uncertainty. |
| US-18 | As an owner, I can steer project extraction and automatic correction. | Natural-language policies support high-confidence auto-correction or review-only. |
| US-19 | As an owner, I can schedule or manually run Night Shift. | Daily maintenance and on-demand execution have inspectable results. |
| US-20 | As an owner, I can change blueprint-derived instructions over time. | Config changes are versioned and do not silently invalidate existing history. |
| US-21 | As a contributor, I can add a stronger global exclusion later. | Context offers to inspect existing contributions and initiate remediation. |
| US-22 | As a member, I can inspect temporally scoped decisions and contradictions. | Old and new facts coexist where scope/time justify it. |
| US-23 | As a member, I can discover useful tools conversationally. | The agent suggests only relevant commands, not a wall of settings. |

## 8. Functional requirements

**Priority legend:** **P0** = necessary for a credible core experience or safety invariant; **P1** = expected complete feature scope but may be staged after the first vertical slice; **P2** = useful enhancement. Priorities are proposed, not a shipping commitment.

### 8.1 Project lifecycle, membership, and activation

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| PRJ-01 | P0 | Create, list, describe, update, archive, and leave projects via agent tools. | No web UI required for the core lifecycle. |
| PRJ-02 | P0 | Bind every project to exactly one Context graph instance. | Project creation validates the graph association and tenant boundary. |
| PRJ-03 | P0 | Store project name, description, goal, blueprint version, shared steering, and owners. | Project configuration is retrievable and editable by authorized users. |
| PRJ-04 | P0 | Invite and join teammates using simple project membership. | Members can search all shared project notes as agreed. |
| PRJ-05 | P0 | Support persistent project activation in the plugin. | Subsequent sessions attach automatically until explicit switch or leave. |
| PRJ-06 | P0 | Make the active project and capture status unmistakable. | Users can identify the project at session start and via an anytime command. |
| PRJ-07 | P0 | Snapshot project binding at session start. | Changing active project later does not rewrite an earlier session's provenance. |
| PRJ-08 | P1 | Provide an easy path to enumerate prior associated sessions. | Contributor can find an incorrect session after it ends. |
| PRJ-09 | P1 | Support project archival without erasing required retained provenance. | Archived projects stop accepting new sessions by default; retrieval/retention behavior is documented. |

### 8.2 Ingestion, extraction, synthesis, and time

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| MEM-01 | P0 | Ingest agent session traces via hooks/integration. | Session-end ingestion is idempotent; failure/retry paths exist. |
| MEM-02 | P0 | Apply personal privacy exclusions before storing or sharing extracted content. | Test cases for secrets, PHI/PII-like content, and custom patterns do not become shared notes. |
| MEM-03 | P0 | Extract typed original notes suited to project goals and blueprint ontology. | Decisions, facts, constraints, risks, discoveries, and skills can be distinguished. |
| MEM-04 | P0 | Preserve an individually attributable original for each source contribution. | Two agreeing contributors remain distinguishable even after a merge. |
| MEM-05 | P0 | Synthesize canonical notes from one or multiple originals. | Unified text preserves qualifiers, exceptions, and meaningful disagreement. |
| MEM-06 | P0 | Keep provenance for every derivation, including recursively derived notes. | Reverse dependency traversal finds every recorded descendant. |
| MEM-07 | P0 | Track effective time, observed time, and modification time separately. | Search and contradiction handling can distinguish changed reality from incompatible claims. |
| MEM-08 | P0 | Support existing link and contradiction detection pipelines. | Project note can link to wider graph entities without losing project ownership. |
| MEM-09 | P1 | Use feedback to update or deprecate low-quality notes. | Later contradictory evidence can trigger review or configured correction. |
| MEM-10 | P1 | Preserve source/derived history while permitting content purging for retractions. | Clean evidence remains; deleted or excluded content does not survive as readable audit text. |

### 8.3 Search, sharing, and permissions

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| RET-01 | P0 | Search project notes together with the caller's permitted wider graph by default. | Project relevance is boosted but not treated as absolute authority. |
| RET-02 | P0 | Accept optional project context on retrieval. | Explicit modes support blended, project-only, and non-project search. |
| RET-03 | P0 | Enforce project membership and graph access at server boundaries. | Cross-project/unauthorized queries do not return protected material. |
| RET-04 | P0 | Return provenance, freshness, temporal scope, and confidence where meaningful. | Agent can qualify answers and provide evidence links. |
| RET-05 | P0 | Store an unlinked note in OpenSearch without forcing a semantic node. | Retrieval handles notes with or without semantic node IDs. |
| RET-06 | P1 | Materialize graph nodes/edges when entity linkage/traversal adds value. | Linked project notes participate in existing graph exploration. |
| ACL-01 | P0 | Project members can see all project notes under the agreed simple sharing model. | A teammate can read contributed shared notes without raw-trace access. |
| ACL-02 | P0 | Support **author-controlled** and **collaborative** correction modes. | Unauthorized edits are denied; all accepted edits are auditable. |
| ACL-03 | P0 | Limit original session trace access and source-session rollback to authorized actors. | Shared-note access does not imply read access to another member's raw agent transcript. |
| ACL-04 | P1 | Make membership and data-sharing implications clear at join. | The user is told extracted project knowledge is shared with members. |

### 8.4 Visibility, privacy, and rollback

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| PRI-01 | P0 | Offer global personal never-extract/never-share steering with safe defaults. | Project/blueprint rules cannot relax global exclusions. |
| PRI-02 | P0 | Support personal contribution steering per project. | Optional role instructions influence quality but are not mandatory. |
| PRI-03 | P1 | Support additional project restrictions and regex/pattern rules. | Clear configuration, test/preview, and versioning are available in-agent. |
| PRI-04 | P1 | On tighter global exclusions, offer a scan of existing contributions. | Findings can feed the same rollback/remediation process. |
| AUD-01 | P0 | List original and canonical notes contributed by user/session. | Each displayed item includes identity, status, and evidence/source linkage. |
| AUD-02 | P0 | Support individual-note correction, retraction, and review flags. | Corrected or removed information updates its derived dependents. |
| ROL-01 | P0 | Preview direct and transitive rollback blast radius before execution. | Counts, depth, independent support, and approximate cost are shown. |
| ROL-02 | P0 | Support full transitive rollback with resynthesis from allowed inputs. | On completion, active project outputs do not rely on retracted evidence. |
| ROL-03 | P1 | Support user-selected bounded-depth rollback. | Result explicitly states that deeper influence may remain. |
| ROL-04 | P1 | Add optional best-effort semantic leakage scan to bounded rollback. | Matches are searched beyond chosen depth and checked against provenance. |
| ROL-05 | P0 | Display job progress, partial failures, and completion state. | User can identify what is complete, pending, or beyond chosen scope. |
| ROL-06 | P0 | Keep affected notes generally searchable during processing. | Availability is preserved; initiator sees pending exposure caveat. |
| ROL-07 | P0 | Remove retracted source support without removing independent evidence. | Multi-author synthesis retains valid surviving nuances after rollback. |

### 8.5 Feedback, Night Shift, and blueprints

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| VAL-01 | P1 | Implement `/context briefing`. | New and changed knowledge since last visit is personalized and evidenced. |
| VAL-02 | P1 | Implement `/context impact`. | Exposure, observed usage, favorable signals, and misleading/corrected cases remain distinct. |
| VAL-03 | P1 | Track note retrieval events and subsequent session feedback. | A later contradiction can be associated with a previously retrieved note. |
| VAL-04 | P1 | Feed quality signals back into extraction and correction. | Accepted/rejected/incorrect notes inform improvement without weakening privacy. |
| NGT-01 | P1 | Run Night Shift daily and on demand. | Scheduled runs and explicit runs have auditable job records. |
| NGT-02 | P1 | Detect staleness, contradictions, duplicates, and consolidation opportunities. | Review/auto-correction follows project policy. |
| NGT-03 | P1 | Produce new inferred insights when grounded in project evidence. | Inferences are labeled, confidence-qualified, and have provenance. |
| NGT-04 | P1 | Steer Night Shift using project goals and optional instructions. | Owner can prioritize focus areas without a complex settings UI. |
| BLU-01 | P0 | Offer built-in Software, Collaborative Research, Data & Pipelines, and BI blueprints. | Each produces useful defaults with no follow-up tuning. |
| BLU-02 | P1 | Allow project-level overrides and later refinement of blueprint defaults. | Changes are versioned, previewable, and do not mutate the stock blueprint. |
| BLU-03 | P1 | Give blueprints typed extraction hints/ontology defaults. | Relevant entities/relationships differ across project categories. |

### 8.6 Future-phase structured claims and reasoning (not a V1 prerequisite)

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| RSN-01 | Future | Optionally extract typed, atomic **claims** alongside readable notes, each with stable ID, source evidence, temporal/scope qualifiers, and policy versions. | A note remains valid without any structured claims; structured claims point to the exact supporting note/source versions. |
| RSN-02 | Future | Define a versioned, blueprint-extensible predicate/entity vocabulary with an RDF-aligned export/mapping. | The same assertion has a stable meaning across extraction, graph materialization, and inference. |
| RSN-03 | Future | Validate claims against explicit schemas/shapes and classify uncertain or contradictory extractions before publishing. | Bad entity types, missing scopes, sensitive content, and unsupported claims are not silently promoted into trusted facts. |
| RSN-04 | Future | Run controlled external inference and retain **each independent justification** for every inferred claim. | Withdrawing one premise removes only conclusions whose remaining valid justifications are insufficient. |
| RSN-05 | Future | Apply project ACLs, time/scope compatibility, and source retraction consistently across text, formal claims, graph links, and inference results. | Rollback and temporal tests pass with both text and formal representations enabled. |
| RSN-06 | Future | Keep structured reasoning invisible in the default agent UX. | Project creation, normal search, Briefing, and Night Shift remain useful without ontology or logic terminology. |

---

### 8.7 Optional board and unified notes (new in v0.3)

P0 in this table means required for the board capability when offered; it does not
make board activation a prerequisite for ordinary project use. `MEM-*` IDs above
are retained for traceability, with their objects now named notes.

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| BRD-01 | P0 | Board opt-in, disable/re-enable, no duplicate note store. | Board-off workflows pass; placement retains the original note ID. AT-25. |
| BRD-02 | P0 | Read/create/update/list notes, folder prefixes and tags. | Scoped pagination, segment-aware prefixes, explicit tag all/any semantics. AT-26. |
| BRD-03 | P0 | Hierarchical placement and moves independent of content lineage. | Moves preserve identity and reject invalid paths/cycles. AT-26, AT-32. |
| BRD-04 | P0 | Blueprint-validated attributes plus flexible custom JSON attributes. | Invalid typed writes fail; custom fields cannot override protected fields. AT-27. |
| BRD-05 | P0 | Pin schemas and support explicit migrations. | A failed migration retains readable old-version notes and attributes. AT-27. |
| BRD-06 | P0 | Valid time independent of creation/update/deadline. | As-of queries and unknown/open bounds do not invent task completion. AT-28. |
| BRD-07 | P0 | Distinguish operational dependencies, semantic links, and derivation. | Cross-project edges/cyclic blocking dependencies fail; rollback follows only actual derivations. AT-29. |
| BRD-08 | P0 | Revision-checked, idempotent mutations with authorship. | Conflicting writes fail explicitly; replay does not duplicate notes/events. AT-30. |
| BRD-09 | P1 | Optional task claims with explicit lease/fencing semantics. | One claim winner; stale workers cannot complete reassigned work. AT-30. |
| BRD-10 | P0 | Same privacy, authorization, source and rollback rules for authored/extracted board notes. | Placing or editing a note cannot bypass content controls. AT-35, AT-36. |
| BRD-11 | P1 | Policy-bound Night Shift refinement and stale-work detection. | No invented completion, evidence, assignment or lease renewal. AT-37. |
| BRD-12 | P0 | Make board-enabled data collaboration an independently graded gym scenario. | Real output checks, exposure records and matched comparison arms. AT-40. |

### 8.8 Active-session change subscriptions (new in v0.3)

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| SUB-01 | P0 | Subscribe by note IDs, tags, prefix, or explicit combinations. | AND across dimensions; tag all/any; explicit whole-project opt-in. AT-31. |
| SUB-02 | P0 | Notify entry, update and exit from selection, including move/tag removal. | ID watches follow moves; exit emits a safe minimal event. AT-32. |
| SUB-03 | P0 | Atomic mutation/event recording and gap-free snapshot-to-stream registration. | A concurrent write is observed in snapshot or replay, never missed. AT-33. |
| SUB-04 | P0 | Durable cursors, deduplication, bounded replay and explicit resync. | Crash/retry, duplicate delivery and expired cursor tests pass. AT-34. |
| SUB-05 | P0 | Authorize live-session ownership and every delivery/refetch. | Revocation cancels queued disclosure; private transcripts never leak. AT-35. |
| SUB-06 | P0 | Expire subscriptions with session/project/board lifecycle. | No silent inheritance or retargeting after session end or project switch. AT-38. |
| SUB-07 | P0 | Deliver through actual host steering capabilities with honest fallback. | Hook injection and polling paths are tested separately; unsupported push is disclosed. AT-39. |
| SUB-08 | P0 | Separate transport receipt, harness injection and agent handling. | No false acknowledgment or claim of attention/use on socket write. AT-34, AT-39. |
| SUB-09 | P0 | Bound event queues, rate and payload size; expose coalescing/overflow. | Retain refresh signals and latest state rather than silently losing work. AT-34. |
| SUB-10 | P0 | Treat all note notifications as untrusted data and include purge in retention controls. | No instruction escalation or replay of purged text. AT-36, AT-39. |

## 9. Permissions, sharing, and trust boundary

### 9.1 V1 sharing rule (agreed)

A project's members can access **all active shared project notes**, irrespective of which member contributed them or whether that member's broader source graph contains material inaccessible to the reader. **Contributing extracted knowledge to the project is an explicit sharing action within that project boundary.** This deliberately simple rule must be explained during project creation/join and during invitation.

It is important to distinguish three surfaces:

1. **Shared project note:** Visible to every member, subject to project retention and correction policies.
2. **Original extracted note / audit view:** The shared claim and its evidence metadata can be surfaced appropriately; detailed source content is subject to the source owner's access rights and deletion policy.
3. **Raw session trace and unrelated graph resources:** **Not** automatically readable by project members. Existing access controls still apply to those underlying resources and to non-project graph search results.

An extracted note may summarize information originating from otherwise restricted data. Since the project explicitly shares that derived knowledge, this is a trust choice with a real disclosure risk; project membership must not be presented as magically inheriting fine-grained source ACLs. Stronger governance and propagation controls are potential later work.

### 9.2 Project roles (proposed minimum)

| Role | Create/invite/configure | Read all project notes | Change shared notes | Retract source sessions |
|---|---|---|---|---|
| **Owner/admin** | Yes | Yes | Yes | Own sessions; others only through explicit authorized administrative/remediation workflow |
| **Member, author-controlled mode** | No, except permitted invitation tasks | Yes | Own contributions and resulting corrections; flag others | Own sessions |
| **Member, collaborative mode** | No, except permitted invitation tasks | Yes | Any shared project note, with audit trail | Own sessions, not others' raw source sessions |

**Agreed:** The correction collaboration model is configurable at project creation and changeable later. **Proposed default:** Author-controlled, with an easy choice of collaborative for a small close-knit team. Collaborative changes do **not** grant access to or permission to rewrite someone else's private trace. An authorized correction to a canonical note creates a new version with its own evidence and authorship, not a silent rewrite of the original source observation.

### 9.3 Privacy and extraction policy hierarchy

The following layers are evaluated together. The most restrictive applicable exclusion always wins.

| Priority | Scope | Purpose | Examples |
|---|---|---|---|
| 1 — Mandatory exclusion | Personal global privacy | What this person never wants contributed to any project | Secrets, credential patterns, sensitive personal or health information, user custom patterns |
| 2 — Additional restrictions | Project-level safety/exclusion policy | What this project never captures or shares | Customer identifiers, protected repositories, project-specific regexes |
| 3 — Selection / steering | Project goals and blueprint instructions | What kinds of knowledge the project values | Architectural decisions, schema changes, hypotheses |
| 4 — Contributor focus | Member-specific instructions in this project | What this person's sessions are likely to contribute | Architecture and tradeoffs rather than code-level details |
| 5 — Adaptive optimization | Learned relevance hints from feedback | Improve precision/recall of extraction | Favor decisions later reused by teammates |

The numbering expresses enforcement precedence for **exclusions**; contributor focus and project goals are complementary ranking signals, not overrides of privacy. A project can tighten privacy but cannot weaken an individual's global exclusions. Adaptation must not turn a once-blocked information class into an allowed class without an explicit user policy change.

**Default privacy posture (proposed):** Filter likely credentials/secrets, highly identifying personal information, protected-health-information-like content, and common customer-sensitive patterns, with configurable false-positive handling. Combine rule-based patterns (including optional regex) with classifiers and contextual analysis. Do not claim automatic PHI/PII detection is perfect. Apply filtering as close to the collection point as feasible, then validate server-side before a shared note is published. Avoid echoing excluded values in diagnostics or logs.

**Retroactive protection:** When a user strengthens their global policy, Context offers: “Scan your previous project contributions for material covered by this rule?” The scan presents findings without needlessly redisplaying sensitive content, estimates the impact, and can initiate corresponding retraction/resynthesis jobs. New rules affect future contributions immediately; historical remediation is asynchronous and explicitly tracked.

### 9.4 Example configurations

**Global personal instruction:**

```text
Never extract or share API keys, authentication tokens, passwords,
protected health information, or personally identifying details about
customers. Also exclude strings matching our internal account-ID rule.
```

**Project shared instruction:**

```text
Prioritize architecture decisions, reliability constraints, API contract
changes, rollout assumptions and unresolved risks. Correct high-confidence
errors automatically unless the note represents a policy decision,
compliance requirement, or unresolved design disagreement; propose those
for review.
```

**Member's project contribution focus:**

```text
I lead architecture reviews. Extract decision rationale, rejected
alternatives and implications. De-emphasize mundane code edits.
```

Each layer is optional to edit because safe, functional defaults already exist.

---

## 10. Note semantics and temporal model

### 10.1 Note categories

Built-in types (extensible by blueprint) include:

- **Fact / observation:** A supported statement about the world or system.
- **Decision record:** Decision, rationale, alternatives, decision makers if known, scope, and date.
- **Constraint / policy:** A rule or condition affecting implementation or interpretation.
- **Discovery / lesson / skill:** A reusable finding, method, or operational practice.
- **Risk / issue / open question:** A concern that should not be represented as a resolved fact.
- **Hypothesis / inferred insight:** A derived interpretation whose confidence and evidence must be explicit.
- **Temporal change / supersession:** A fact that modifies the applicable period or applicability of another note.

### 10.2 Original evidence versus shared synthesis

**Original extracted note:** A durable source observation extracted from one input session (or, in future, another approved source). It has an author, session ID, relevant excerpt/reference, extraction version, and temporal metadata. Do not merge two people's originals into an indistinguishable source item.

**Canonical/shared note:** A versioned, readable project-level synthesis. It may be supported by multiple independent original observations, and those originals may introduce complementary qualifiers or apparent contradictions. The synthesized note should state the narrowest justified scope and explicitly preserve disagreement when reconciliation is impossible.

**Derived note:** A synthesis or inferred insight whose parents may themselves be canonical or derived notes. The provenance dependency structure for derived content must be **acyclic**, even though separate semantic graph relationships (e.g. “related to”) can contain cycles.

Explicitly authored notes are first-class notes too. Their original authored
version records the principal/session and any supplied evidence without claiming
transcript extraction or factual verification. Board placement is metadata, not
a new content origin. Derived content cites exact input versions regardless of
whether its inputs were authored, extracted, or themselves derived (see §30).

### 10.3 Required time fields

At minimum, distinguish:

- `observed_at`: When the source session or supporting evidence observed the condition.
- `recorded_at`: When Context accepted that note or revision.
- `valid_from` / `valid_to`: When the claim is believed to apply in the domain.
- `last_confirmed_at`: Most recent independent confirmation, if any.
- `superseded_at` and `superseded_by`: When a replacement applies, without necessarily deleting historical truth.
- `scope`: Environment, service, endpoint, customer segment, experiment, or other qualifiers as appropriate.

Do not infer `valid_from` solely from `recorded_at`; uncertainty should be representable. Contradiction detection should evaluate temporal overlap and scope compatibility before classifying claims as genuinely incompatible.

### 10.4 Suggested note schema (illustrative)

```json
{
  "noteId": "mem_9f3",
  "projectId": "proj_billing",
  "kind": "decision",
  "version": 4,
  "text": "Automatic retries for legacy charge creation remain disabled ...",
  "subjects": ["BillingGateway", "ChargeCreationHandler"],
  "scope": {"operation": "legacy-charge-create", "environment": "production"},
  "time": {
    "observedAt": "2026-10-29T16:10:00Z",
    "recordedAt": "2026-10-29T16:12:00Z",
    "validFrom": "2026-10-29T00:00:00Z",
    "validTo": null,
    "lastConfirmedAt": "2026-10-31T11:15:00Z"
  },
  "epistemicStatus": "supported",
  "confidence": 0.94,
  "status": "active",
  "originKind": "derived",
  "transformationKind": "synthesis",
  "sourceEvidenceIds": ["orig_271", "orig_304"],
  "parentNoteVersionIds": [],
  "supersedesVersionId": "mem_9f3_v3",
  "contradictionIds": [],
  "graphNodeId": "optional_graph_node_id",
  "policyVersion": "project-policy-12",
  "createdByJobId": "synth_331"
}
```

`confidence` is model/evidence judgment, **not** a calibrated guarantee. A production schema should explicitly differentiate direct sources, lineage parents, authorship, temporal state, and trust/review status. Version references are immutable identities even when the displayed note has a newer version.

### 10.5 Optional machine-readable claims (future phase)

A note's natural-language `text` remains its primary, human- and agent-readable form. In a future phase, extraction may also identify a small set of **structured claims**—atomic, scope-qualified propositions whose entities, relationships, values, time, and evidence can be inspected by an inference or rule engine. These are **not** required for a note to exist, be searched, or be useful. A structured claim has its own stable version identity and provenance, links back to the text note, and must be retractable without leaving an unsupported formal inference in the graph. See **§28** for the representation, RDF/OWL-versus-SMT tradeoff, PostgreSQL relationship traversal specifics, and safety contract.

---

## 11. Architectural model

### 11.1 Canonical state with retrieval and delivery projections

```text
Claude Code / Codex / SDK harnesses
       | tools, capture, subscription polling/injection acknowledgment
       v
Rust service: authentication, projects, notes, board, policy, subscriptions
       |                         |
       v                         v
PostgreSQL authority         OpenSearch retrieval
notes + versions             rebuildable text/vector projection
lineage + entity links          |
board + subscriptions <---------+ reauthorize canonical state
transactional outboxes
       |
       +--> indexing / rollback / Night Shift workers
       +--> safe change envelopes --> supported harness steering boundary
       +--> future reasoning adapters over authorized versioned inputs
```

All notes have canonical PostgreSQL records even when no semantic entity links
exist. Neither a board nor a physical graph node is required for retrieval. A
board placement and a search result refer to the same logical note, not independent
copies. Deployable process boundaries can remain a modular monolith initially.

### 11.2 Distinct relationship concerns

Persist complete derivation lineage and reverse adjacency in PostgreSQL. Keep
operational board dependencies and semantic links separate from content derivation.
Only actual derivation edges drive content rollback; all three relation families
remain scoped and authorized. Query-time checks exclude unauthorized/retired
projection results. Changes and their delivery outbox commit atomically.

Formal claims and optional graph engines can later consume versioned authorized
snapshots through an adapter. They cannot become the only record of provenance
or bypass note publication, privacy and retraction. The board's typed attributes
do not depend on that future phase.

### 11.3 Major components

| Component | Responsibilities |
|---|---|
| **Agent integration layer** | Project creation/selection, visible status, tool discovery, persistent activation, session-start/end capture and retry. |
| **Board and change service** | Optional organization, typed/custom attributes, revision-checked mutations, relationship semantics, scoped subscriptions and transactional events; see §30. |
| **Harness notification adapter** | Authorized cursor replay, deduplicated safe steering at supported boundaries, polling fallback and stage-specific acknowledgments. |
| **Project service** | Graph-instance association, lifecycle, membership, blueprint snapshot and policy versions. |
| **Privacy/policy evaluator** | Global exclusions, project restrictions, contribution steering, automated correction eligibility, logging minimization. |
| **Trajectory ingestion service** | Accept filtered session data, deduplicate hook deliveries, track completeness, attach immutable source identity. |
| **Extraction service** | Identify original typed facts, decisions, risks and skills from source sessions according to policy. |
| **Synthesis/contradiction service** | Group related evidence, preserve nuance, reconcile or flag conflicts, publish versioned shared notes. |
| **Provenance service** | Maintain immutable source IDs, dependency DAG, reverse adjacency, blast-radius queries, evidence audit. |
| **Note indexing/linking** | Index OpenSearch documents; link to semantic nodes on demand; update/deprecate stale graph links. |
| **Search orchestrator** | Resolve active project, apply ACL, hybrid search, boost/rerank, traverse links, return provenance-aware results. |
| **Feedback and impact service** | Correlate retrieval with later usage, correction, contradiction and optional user feedback. |
| **Rollback coordinator** | Preview scope, select mode, execute idempotent staged resynthesis, track partial failures and completion. |
| **Night Shift orchestrator** | Scheduled/on-demand upkeep, synthesis, stale detection, evidence-based inference, run reports. |
| **Briefing service** | Summarize what changed since a user's previous visit, with evidence links and action suggestions. |
| **Structured-claim + reasoning service (future)** | Extract optional typed claims from note versions, validate their schema, project selected claims into an optional graph adapter, evaluate governed rules, and record alternate derivation justifications. Not on the critical path for creating or searching text notes. |

### 11.4 Tenant and project isolation

Every object carries tenant/context-instance and project identity. Every service must independently enforce authorization; a client-provided `projectId` is a scope request, **not** an authorization grant. Keep projects independent even if their notes link to entities in a shared graph. Do not permit one canonical note to be owned by multiple projects in this phase. Shared entity links are not license to read another project's note.

---

## 12. Ingestion, extraction, and synthesis pipeline

### 12.1 Lifecycle

1. **Activate:** User selects a project in Claude Code. Plugin persists the activation and shows it.
2. **Start session:** Integration creates/records a stable session ID and snapshots project, user, agent, workspace, policy versions, and capture configuration.
3. **Capture trajectory:** Session hook captures the authorized trace or a filtered/structured version. Client-side protection is preferred where feasible; server-side validation is required.
4. **Submit reliably:** Session-end hook submits data or a reference. Handle shutdown failures, offline sessions, retries, duplicates, and partial traces without silently changing provenance.
5. **Apply policies:** Enforce personal exclusions and project restrictions before shared persistence. Interpret project goals/blueprint and member focus as extraction priorities.
6. **Extract originals:** Produce independently attributed, typed observations with times, scope, evidence references, extraction model/policy versions, and sensitivity labels.
7. **Link and reconcile:** Compare with project notes, perform duplicate detection, contradiction/scope/time analysis, and identify related graph entities.
8. **Synthesize:** Create or update a canonical/shared note, retaining all relevant originals and nuanced qualifiers as separate provenance inputs. Do not silently drop a minority observation solely because the majority agrees on a simpler statement.
9. **Index and materialize selectively:** Publish to OpenSearch and, when useful, graph links in PostgreSQL. All published content references valid provenance records.
10. **Collect feedback:** Retrievals, observed use, corrections, and new evidence may queue re-evaluation and adjust future extraction priorities.

**Future option, not an added onboarding step:** Between extraction and indexing, a *formalization pass* can propose typed claims from selected note versions, resolve entities, validate shapes, and send high-value assertions for rule evaluation. Both valid and rejected formalization outputs are traceable to the same source/policy snapshots; failure of this optional pass must not block normal text-note indexing. Derived formal claims are published only with their evidence/justification records.

### 12.2 Multi-author synthesis example

- Avery: “Retries of non-idempotent charges are unsafe without idempotency keys.”
- Sam: “The v2 charge endpoint now accepts idempotency keys after the October rollout.”
- Jordan: “Legacy charge requests still bypass the v2 gateway.”

The synthesis must not produce a simplistic “retries are safe now.” It should distinguish endpoint and rollout scope, retain all three original observations, and expose unresolved assumptions. If Sam retracts their session, the system re-synthesizes from Avery and Jordan alone; it must not retain the v2 claim merely because it appeared in an earlier combined note.

### 12.3 Source and derivation guarantees

- A note revision is never supported merely by its own previous generated wording; it must trace to surviving independent sources or explicitly derived, independently supported inputs.
- All transformations record input version IDs, policy/model versions, and a job ID. No dependency graph cycles are allowed.
- A contradictory source is not removed simply because another source disagrees; the system can preserve both with a dispute status until enough evidence resolves them.
- If a source must be purged for privacy reasons, preserve only necessary **non-sensitive** audit metadata (IDs, timestamps, action types, job states); do not retain the prohibited text in an “immutable” audit log.
- Different contributors may support the same fact independently. Retraction removes a source's support, not the fact if other valid sources continue to establish it.

### 12.4 Determinism: define what can actually be guaranteed

**Desired invariant:** Once a source is retracted, no completed, active project note should depend on that source, directly or indirectly. A clean reconstruction should use only authorized, non-retracted evidence and reproducible transformation inputs.

For operational determinism, use fixed source sets, explicit ordering, immutable version IDs, version-pinned prompts/models/configuration where available, idempotent jobs, and content hashes. **A generative model cannot be assumed to reproduce byte-for-byte language or provide a mathematical guarantee against every semantic echo.** The enforceable guarantees are: complete recorded lineage; exclusion of retracted evidence from resynthesis inputs; no published active note with a dependency path to an excluded source after a **completed full rollback**; consistent state transitions; and validation/search checks for possible leaks. Stronger semantic non-leakage claims require additional validation and narrowly specified threat models.

---

## 13. Retrieval and context overlay

### 13.1 Three explicit search modes

1. **Blended (default while active):** Search the active project's notes plus the user's otherwise accessible context graph. Project relevance is an input to ranking, **not an unconditional override**.
2. **Project-only:** Search notes owned by the specified project, with optional authorized linked-entity evidence if policy explicitly allows it. Useful for audits and narrowly scoped queries.
3. **General/no-project:** Search the wider graph as it behaves today, without an active project's boost or project-only filter.

A programmatic retrieval API should accept an optional `projectId` and mode. If the active plugin session supplies a project and the caller does not explicitly override it, default to blended retrieval. Always check membership; no implicit fallback that accidentally reveals project content.

### 13.2 Retrieval stages

1. Resolve caller identity, tenant/context instance, membership, and requested mode.
2. Query OpenSearch hybrid search with project metadata filters or boost weights as appropriate.
3. Incorporate relevance to project goals, exactness, temporal applicability, source confidence, recency, and contradiction/review state.
4. For returned documents with `graphNodeId`, perform the existing PostgreSQL relationship traversal traversal to enrich results with related entities and links.
5. Deduplicate OpenSearch and relationship projections of the same logical note.
6. Rerank combined evidence; handle contradictions as qualified alternatives, not silently pick the most recent text.
7. Return appropriate source references, status, time, and provenance so the agent can ground its response.
8. Emit a retrieval event for future impact and effectiveness analysis, observing data minimization and privacy rules.

### 13.3 Conflict and ranking policy

- **Project boost:** A project note should generally outrank an equally relevant generic graph fact about the same project, but not override clearly better or more current evidence.
- **Temporal fit:** An applicable fact from an earlier date may outrank a more recent fact that applies to a different endpoint or environment.
- **Epistemic status:** Unverified inference, active disagreement, and stale evidence should be distinguishable in responses.
- **Source trust:** A direct, independently corroborated observation can carry more weight than a highly re-used but poorly supported synthesized claim.
- **Ambiguity:** When conflicts remain unresolved, surface them explicitly rather than conceal them behind a numeric ranking score.

### 13.4 Example search contract (proposed)

```json
{
  "query": "What is our retry policy for charge creation?",
  "contextInstanceId": "ctx_engineering",
  "projectId": "proj_billing",
  "mode": "blended",
  "asOf": "2026-10-31T00:00:00Z",
  "includeEvidence": true
}
```

The result should distinguish: source type, project ownership, title/text, temporal validity, status, original-source references, graph links (if materialized), and why the result ranked prominently. The JSON is illustrative, not an existing AWS contract.

## 14. Provenance, correction, and rollback

This is a defining differentiator and one of the hardest technical areas of the feature. It should be designed from the start, even if advanced recovery modes arrive incrementally.

### 14.1 Dependency model

Model a directed acyclic provenance graph of **versioned derivation dependencies**, independent of the semantic/entity relationships in PostgreSQL.

```text
Session S-A ──> Original A ─┐
                            ├──> Canonical C v1 ──> Derived D ──> Insight E
Session S-B ──> Original B ─┘           │
                                        └──> Another derived note F
Session S-C ──> Original C ─────────────┘
```

A graph node in this diagram is a logical note or specific note version, **not necessarily a physical semantic node**. A source/session → original edge establishes provenance. A source/derived → derived edge establishes synthesis dependency. The system must be able to traverse parents, children, and **transitive descendants**, count the blast radius, and locate unaffected independent original evidence.

A full rollback of S-A **does not delete Original B or C**. It invalidates the syntheses that used Original A, then re-derives supported results using B and C. If the new evidence cannot support a claim, that claim is retired. If a downstream insight depended entirely on the removed claim, it too must be retired or re-derived from other independent evidence.

### 14.2 Preview first; do not surprise the user

A rollback plan reports, at minimum:

- Target: one source note, all originals from a session, or an identified set of contributed facts.
- Direct original notes and linked shared notes.
- Number of **distinct** transitive descendants, maximum dependency depth, affected versions, and estimated work. Counting shared nodes only once matters.
- Which notes have independent surviving source support and which may be retired.
- Number of graph links, OpenSearch documents, index versions, and caches expected to change (when estimable).
- Scope/permissions, chosen rollback mode, progress behavior, and **residual-leakage caveats**.
- Rough cost/time bands if useful; estimates must be labeled as estimates.

A preview should avoid dumping sensitive values in UI or logs. The preview can be revised if lineage changes before execution; the execution takes a consistent source/version snapshot or revalidates before committing.

### 14.3 The three user-facing rollback modes

| Mode | Behavior | Result and caveat | Suggested use |
|---|---|---|---|
| **Full recursive rollback** | Traverse all recorded descendants, invalidate impacted versions, re-synthesize from surviving clean evidence, publish clean replacements or retire unsupported notes. | **Complete for the active project note dependency graph when the job finishes successfully.** Potentially expensive. Not immediate while pending. | Sensitive disclosure, mistaken session contribution requiring thorough removal. |
| **Bounded rollback** | Retract direct inputs, process descendants only to a user-chosen dependency depth. | Faster and scope-controlled, but deeper notes can still encode the removed information; **not a complete information purge**. | Ordinary quality corrections and non-sensitive cleanups. |
| **Bounded rollback + semantic leakage scan** | Run bounded rollback, then extract distinctive facts/identifiers from retracted material, search the entire eligible project knowledge corpus for them regardless of depth, verify provenance paths when found, and selectively repair matched dependents from clean evidence. | More coverage for dispersed leakage without full recomputation. **Best effort only**; semantic misses and indirect implications remain possible. | Large fan-out where exhaustive rollback is costly but fact-specific leakage is a concern. |

**Agreed:** The chosen depth and semantic scan are optional decisions at rollback time, not complex mandatory project settings. Show a strong recommendation for full rollback when the user says a session contains sensitive information. Avoid overstating semantic-scan completeness.

### 14.4 Full rollback execution (proposed algorithm)

1. **Authorize and snapshot.** Verify rights to retract the target source/session, capture its version, and freeze the work plan or establish safe optimistic concurrency.
2. **Mark original support retracted.** Exclude the selected source evidence from *all future extraction, synthesis, and correction inputs*. Create content-safe tombstones as needed.
3. **Find descendant closure.** Walk reverse dependency edges to enumerate impacted note versions across **arbitrary depth**. No fixed derivation-depth limit; execution is partitioned into bounded batches for resource safety.
4. **Compute affected ordering.** Process dependencies in topological order; account for shared descendants and concurrent updates.
5. **Build clean input sets.** For each affected output, retain only independent originals or previously rebuilt ancestors that remain permitted; never feed contaminated old generated text back in as authority.
6. **Re-synthesize or retire.** Generate a new canonical/derived version from clean evidence, or mark it unsupported/retracted. Preserve relevant scope, uncertainty, and temporal qualifiers.
7. **Update indexes and links.** Replace/retract OpenSearch versions and reconcile PostgreSQL entity edges as needed; do not leave stale graph traversals pointing to a retired active claim.
8. **Validate lineage closure.** Query dependency records to verify no **active, completed-scope** note has a path to the retracted source. Run optional content/fact scans as defense in depth.
9. **Finalize and report.** Mark the job complete only after publishing and validation succeed; report any residual failures, excluded out-of-scope resources, and retention caveats.

**Important availability choice (agreed):** While a job runs, prior active notes remain generally searchable instead of being universally hidden. This preserves availability, **but it also means sensitive content can still be retrievable until the affected items are actually updated.** The initiator must see that the job is pending, what has finished, and what is still exposed. Do not market “full rollback” as instantaneous privacy erasure. For severe exposures, a separate future *targeted redaction or query-time masking* mechanism might reduce risk without taking whole notes offline; that would be an additional design choice, not a reason to silently change the agreed availability behavior.

### 14.5 Bounded rollback execution

Given `maxDepth = d`, process affected descendants with minimum dependency-path distance ≤ `d` from the source. At the boundary, record the count of known further descendants not reconstructed. A job can finish successfully **within its requested boundary** while remaining explicitly **incomplete as an information-removal operation**. The status should say, for example:

```text
Bounded rollback finished (depth 2)
9 note versions rebuilt; 2 retired.
41 deeper dependent notes were not reconstructed.
Some downstream information may still reflect the retracted source.
```

The system should allow a user to later expand the depth or upgrade to a full rollback based on the same source identities and prior job history.

### 14.6 Optional semantic leakage scan

The discussion proposed a way to catch a sensitive fact even when it has traveled 30 derivation hops. The algorithm should be explicit about what it can and cannot prove:

1. **Create search fingerprints:** Extract distinctive identifiers, values, entities, propositions, and relationships from the retracted source. Examples include exact strings, normalization variants, structured facts, secure exact-match fingerprints where feasible, and semantic paraphrases. Minimize retention of sensitive raw values.
2. **Search broadly:** Search all project-owned active OpenSearch notes, and optionally *authorized linked graph content*, regardless of lineage depth. Use exact/pattern matching where possible and semantic retrieval to find paraphrases or indirect restatements.
3. **Verify lineage:** For each candidate, check whether it has a recorded transitive dependency path to the retracted source. This avoids removing an independently evidenced fact solely because its text resembles the retracted fact.
4. **Selectively repair:** If a contaminated note is found far beyond the bounded rollback boundary, reconstruct it and any **necessary intermediate derivations** from clean evidence (or retire it). Do not re-feed a contaminated intermediate as an authority merely to save work.
5. **Recheck and report:** Scan repaired results, document examined scope, matches, repairs, and uncertainty. Preserve a path to escalate to full rollback.

**Limitations:** Semantic retrieval may miss obscure transformations, absent matches are not proof of absence, facts may be implied rather than stated, exact match on confidential values is not always safe to store, and some content outside the project's edit scope may be detectable but not directly removable. Searching a user's wider graph may reveal a suspect reference, but **automatic mutation remains limited to owned project notes and other specifically authorized resources**. External findings should be surfaced for separate handling, not silently edited across authorization boundaries.

### 14.7 Large blast radius and concurrency

A single source may have millions of descendants in a mature project. “Arbitrary derivation depth” must not mean “unbounded synchronous computation.”

**Proposed implementation:** Persist a resumable rollback job with a source snapshot, cursor/checkpoints, idempotency keys, work queues, and bounded batch budgets. Prioritize dependency-closure correctness over time-to-finish; add fairness and rate controls. Support cancellation only at safe checkpoints, with an explicit incomplete state. Concurrent new derivations must reject retracted source IDs or revalidate against the current retraction set. A node that is re-derived while rollback is in flight cannot republish stale support.

### 14.8 Progress and result semantics

Suggested job states:

`previewed → queued → retracting_sources → scanning_lineage → resynthesizing → reindexing → validating → completed`

Additional terminal/intermediate states: `partial_failure`, `paused`, `canceled_incomplete`, `failed`, and `completed_with_residual_risk` (for bounded modes).

At any time `/context rollback status <job>` shows:

```text
Rollback R-42 — full recursive
Originals retracted:       4 / 4
Affected descendants:   116 (estimated)
Rebuilt or retired:       79 / 116
Awaiting validation:      37
Failures requiring retry:  0
State: RESYNTHESIZING

This rollback is not yet complete. Affected notes remain generally
searchable during processing.
```

Avoid showing the same progress warning to every teammate retrieving a note by default; the requester needs transparent progress. Other retrieval responses can still incorporate ordinary stale/contested confidence signals where they are already part of standard retrieval behavior.

### 14.9 Deletion, history, and retained source material

Two apparently competing requirements must be resolved carefully:

- **Keep originals:** For valid contributions, originals are retained as independent evidence so merged notes can be reconstructed faithfully.
- **Retract or purge disallowed information:** A rolled-back or privacy-excluded original must no longer be an active input. If the operation requests removal of sensitive source content, that content must be removed from active originals, derived notes, searchable indexes, source snapshots/caches, and applicable retained trace stores according to the deletion policy. Historical job metadata may remain, but not a copy of the prohibited content hidden in a “history” feature.

Do not conflate **retract support** with **erase every stored copy**. Define and display retention behavior for raw traces, immutable storage, backups, delayed indexes, logs, and audit metadata. A completed active-note rollback guarantee is narrower than a verified full-storage erasure claim; retention compliance and backup expiry require separate product/security specifications.

### 14.10 Future integration: formal claims and rule-derived facts

A full rollback must eventually traverse **both** narrative-note dependencies and formal-claim justifications (including rule-derived facts). It must retire an inferred claim when every justification depends on a retracted input, while preserving a claim independently supported by a different source or valid rule proof. Bounded modes must disclose that unprocessed formal inferences may also retain the retracted information. Use claim-version IDs and derivation/justification IDs, not raw string equality, as authoritative dependencies; semantic search is supplementary. The independently maintained lineage ledger remains the removal authority. Details and invariants appear in §28.7.

---

## 15. Feedback, note effectiveness, and demonstrating value

### 15.1 Why note count is insufficient

The product must make its usefulness visible, not just announce that an extractor produced 40 notes. A large volume of low-quality notes may make search worse. Worse, a retrieved note may be incorrect and lead to a mistake. Value measurement therefore follows **extraction → retrieval → observed application → feedback / later evidence → correction**.

### 15.2 Suggested event model

| Event | Meaning | Attribution caveat |
|---|---|---|
| `note_extracted` | Source generated a candidate original | Not proof of usefulness. |
| `note_accepted` / `note_merged` | Source became independent evidence for project knowledge | Merge should credit all valid contributing originals. |
| `note_retrieved` | Search supplied a note in context | The agent/user might never act on it. |
| `note_cited` / `note_used` | Agent quoted, cited, or explicitly applied the knowledge | Observed use is stronger, but not necessarily correct. |
| `note_helpful_feedback` | User confirmed usefulness or downstream evidence supports it | Positive signal, not guaranteed causal impact. |
| `note_contested` | Later session/feedback provided conflicting evidence | Requires scope/time verification; not automatically a factual error. |
| `note_misleading` | Review found earlier retrieved guidance wrong or materially deceptive | Negative effectiveness signal; track exposure and correction. |
| `note_corrected` / `note_retracted` | Content was revised or removed | Indicates hygiene and recovery, not inherently personal failure. |

Events should include project, note **version**, retrieval session, time, and signal source. Correlate a later contradictory observation with previously retrieved versions where the provenance supports it. Do not assume that every later contradiction means the previous retrieval caused bad work.

### 15.3 `/context impact`

Show both individual and team perspectives, with separate measures for:

- **Contribution:** Independently accepted originals, decisions, discoveries, and meaningful additions to merged notes.
- **Reach:** How many teammate sessions retrieved them; distinct users and recency as appropriate.
- **Demonstrated usefulness:** Explicit citations, positive feedback, validated outcomes, or successful reuse where detectable.
- **Improvement:** Notes corrected, refined, or superseded due to this person's new evidence.
- **Negative signals:** Notes that were retrieved and subsequently identified as misleading, with context and remediation, not unqualified blame.
- **Project knowledge health:** Current, contested, stale, unresolved, or unsupported notes; potentially affected team workflows.

**Gamification ideas to test:** Quiet, useful milestones such as “Your architectural decision informed three teammates,” a weekly “knowledge saved from being rediscovered” digest (clearly an estimate if calculated), and recognition for **fixing** misleading notes—not just creating new ones. Avoid a single opaque leaderboard or punitive score that incentivizes oversharing, note spam, false certainty, or reluctance to correct errors. Every numerator and denominator should be explainable.

### 15.4 `/context briefing`

Personalize to a member's last visit, interests, contribution focus, and current project goals. Suggested sections:

1. **Important changes:** New decisions, changed rules, newly confirmed facts, and time-sensitive updates.
2. **Team contributions:** Discoveries by other members that affect the viewer's likely work.
3. **Open issues:** Genuine unresolved contradictions, risky assumptions, and evidence gaps.
4. **Night Shift report:** Consolidations, deprecations, suggested actions, and new inferred insights.
5. **Your follow-ups:** Pending review requests, rollback progress, and notes that may need your input.

This should appear as an answer within the agent, with optional drill-down into each note and source. Avoid duplicating everything the user already knows.

### 15.5 Adaptive quality improvement

Use positive/negative signals to refine note extraction, entity linking, duplicate thresholds, reranking, and correction policy recommendations. The system may learn that a project lead contributes important design rationale while an implementer contributes service contracts and integration facts. Treat learned focus as **revisable suggestions**, not irreversible profiling. Personal global exclusions remain hard limits.

---

## 16. Night Shift — daily project intelligence maintenance

### 16.1 Purpose and cadence

**Night Shift** is the project's nightly maintenance and insight-discovery pass. It runs approximately **once per day**, with **on-demand execution** via the agent. The name is intended to set user expectations: more thorough, batched review rather than constant automatic rewrites. Time zone/window and run scheduling are implementation settings that normally remain hidden behind defaults.

### 16.2 Jobs Night Shift may perform

1. **Staleness detection:** Identify facts whose valid time, last confirmation, or environmental assumptions may have expired.
2. **Contradiction detection:** Compare overlapping claims, including temporal and scope qualifiers, and identify genuine disagreement.
3. **Consolidation and simplification:** Merge redundant or overly fragmented shared notes while preserving original evidence and material nuance.
4. **Deprecation and supersession:** Update confidence/state of outdated notes; link successor decisions rather than erase useful history.
5. **Gap discovery:** Compare known project goals with what the project knows; surface consequential missing evidence.
6. **Note-effectiveness review:** Revisit notes previously retrieved and then flagged misleading or contradicted. Feed conclusions into future extraction and ranking.
7. **New inference/insight synthesis:** Derive plausible new insights, connections, risks, or next questions from existing evidence; record them as inferences with provenance and confidence.
8. **Project health report:** Summarize maintenance work, outstanding reviews, uncertain insights, and likely next actions for Briefing.

### 16.3 Steering

An owner can optionally say:

```text
For Night Shift, pay special attention to security exceptions, migration
rollouts, changes in gateway idempotency, and contradictions between API
contracts and integration tests. Auto-correct factual duplicates when
confidence is high; bring architectural policy changes to me for review.
```

Blueprint defaults include Night Shift and inference instructions. Project goals and member feedback tune what it cares about; global/project exclusions still apply. Night Shift is not permitted to relabel a hypothesis as verified without new supporting evidence.

### 16.4 Safety and provenance constraints

- Every transformation, retirement, and new inferred insight has a job/run ID and provenance.
- Existing collaboration rules and review-only rules govern automated publication. If policy requires author/owner approval for certain note types, Night Shift produces proposals.
- Insights carry `originKind: inferred`, confidence, scope, and citations to supporting notes/source evidence.
- Do not create infinite self-reinforcement by counting a derived insight's own descendants as independent proof of the original inference.
- Night Shift resynthesis must honor retracted sources and concurrent rollback jobs.
- Maintain an audit/change report that the project can summarize in Briefing.

### 16.5 Example commands

```text
/context night-shift status
/context night-shift run
/context night-shift report --last
```

Potential result:

```text
Night Shift completed
Reviewed: 312 project notes
Consolidated: 11 overlapping notes into 4
Marked stale: 3
Flagged unresolved conflicts: 2
New inferred insights: 2 (1 published, 1 awaiting review)
Changes and evidence: /context night-shift report --last
```

These example counts are illustrative, not product guarantees.

---

## 17. Built-in project blueprints

### 17.1 Blueprint contract

A **blueprint** is a versioned, pre-baked starting configuration selected during project creation. It is **not** a separate runtime or a rigid project type. Users can adjust its settings at creation or later, in natural language. A project takes a snapshot of a blueprint version and then owns its local overrides; changing the stock blueprint should not silently overwrite an existing project's customizations.

A blueprint can provide:

- Project-goal prompts and typical objectives.
- Default note categories and **extraction ontology**: entity types, relationship types, qualifiers, common synonyms.
- Project extraction and synthesis instructions.
- Default prioritization of decisions, facts, risks, constraints, skills, and other typed knowledge.
- Contradiction resolution and high-confidence correction defaults.
- Retrieval relevance hints and freshness expectations.
- Night Shift cleanup, staleness, and inference instructions.
- Starter privacy/exclusion hints (which can **tighten**, never weaken global exclusions).
- Suggested optional roles or member contribution-focus prompts.
- Optional board note kinds, typed/custom attribute rules, folder templates, dependency/transition semantics, and subscription defaults, as specified in §30.

### 17.2 Starter catalog

| Blueprint | Typical ontology / entities | Extraction priorities | Night Shift and insight focus |
|---|---|---|---|
| **Software Development** | Repositories, packages, modules, services, APIs, endpoints, schemas, dependencies, incidents, deployments, environments | Architecture decisions, API contracts, implementation constraints, tests, reliability/security tradeoffs, rejected alternatives | Detect stale design decisions, interface drift, dependency conflicts, missing tests, inconsistent assumptions across components |
| **Collaborative Research** | Research questions, claims, hypotheses, sources, methods, studies, datasets, findings, confidence, citations | Evidence for/against hypotheses, methodological caveats, disagreements, open questions, decisions about research direction | Identify contradictory evidence, unsupported conclusions, aging sources, new cross-source hypotheses, gaps in experimental design |
| **Data & Pipelines (ETL)** | Data sources, datasets, tables, columns, schemas, transforms, DAG tasks, owners, schedules, freshness, quality assertions | Lineage, schema/contract changes, failure modes, transformation rationale, dependencies, data-quality issues | Detect schema drift, stale dependencies, broken assumptions, recurring incidents, missing lineage and quality coverage |
| **Business Intelligence** | Metrics, definitions, dimensions, cohorts, dashboards, analyses, business questions, assumptions, decisions | Metric definitions, competing interpretations, analytical findings, assumptions, decision context, reproducibility | Flag definition drift, contradictory analyses, stale figures, segment effects, and new evidence-grounded business insights |

### 17.3 Example software blueprint excerpt (illustrative)

```yaml
id: software-development
version: 1
extract:
  priority:
    - architecture_decision
    - api_contract_change
    - reliability_constraint
    - security_assumption
    - unresolved_risk
    - reusable_implementation_lesson
  entity_types:
    - Repository
    - Package
    - Module
    - Service
    - APIEndpoint
    - Schema
    - Deployment
  relationships:
    - DEPENDS_ON
    - IMPLEMENTS
    - CALLS
    - SUPERSEDES
    - AFFECTS
  guidance: >
    Favor decisions with rationale and scope. Preserve differing
    environment/endpoint constraints instead of flattening them.
correction:
  high_confidence_auto_correct: true
  review_first_for:
    - security_policy
    - architectural_policy
    - unresolved_conflict
night_shift:
  cadence: daily
  look_for:
    - stale_architecture_assumptions
    - api_contract_drift
    - missing_dependencies
    - unresolved_reliability_risks
  derive_evidence_grounded_insights: true
```

This YAML is an example configuration language, **not** a requirement that users edit YAML or that the service expose this exact schema.

### 17.4 Customization and evolving projects

A project owner might say:

```text
Tune this software project's blueprint: treat our Terraform resources,
IAM roles, and OAuth audiences as first-class entities. Prioritize
identity/security decisions. Have Night Shift check for outdated trust
assumptions and changes to external IdP configuration.
```

Context applies a versioned project-local override. It should summarize what changed, offer a preview of how future extraction is affected, and optionally offer a retrospective re-analysis of eligible existing source evidence. **Changing instructions should not silently rewrite every past note.** Any retroactive transformation is an auditable job with lineage and rollback semantics.

---

## 18. Agent-native UX, command grammar, and tool discoverability

### 18.1 Natural language first, commands second

A user should be able to say “Create a project for our migration,” “What did I contribute yesterday?”, “Stop sharing my work with this project,” or “Remove the knowledge learned in my last session” without memorizing exact syntax. Slash commands provide predictable shortcuts for common operations and are **discoverable affordances**, not the only interface.

### 18.2 Proposed command surface

| Command / intent | Behavior |
|---|---|
| `/context project create` | Create a project; asks only for missing essentials, suggests a blueprint. |
| `/context project join <name>` | Join/invite acceptance and activate. Optional role/contribution focus. |
| `/context project switch <name>` | Make a different project active for future sessions. |
| `/context leave` | Deactivate project capture for future sessions in the relevant activation scope. |
| `/context status` | Active project, capture state, effective privacy summary, and useful next commands. |
| `/context projects` | List accessible projects and their goals/roles. |
| `/context briefing` | What's new, important, or unresolved since last visit. |
| `/context impact` | Contribution/reuse/correction evidence; personal or team view as authorized. |
| `/context notes mine` | Show originals and affected shared notes, filter by session/date/status. |
| `/context notes inspect <id>` | Show a note's meaning, source evidence, time, and provenance. |
| `/context notes correct <id>` | Guided correction or flag for review, respecting collaboration mode. |
| `/context rollback preview <session-or-note>` | Show impact and available rollback modes. |
| `/context rollback run <preview>` | Confirm full, bounded, or bounded-plus-scan execution. |
| `/context rollback status <job>` | Show progress, failures, and residual risks. |
| `/context privacy` | View/edit global exclusions and optional retroactive scan. |
| `/context project settings` | Conversational project instructions, correction mode, blueprint overrides, members. |
| `/context night-shift status|run|report` | Daily maintenance status, execution, and evidence-based changes. |

**Avoid command proliferation in the beginner experience.** The first-run guidance might mention only `/context status`, `/context briefing`, and `/context leave`. Other commands appear when the user asks the relevant question.

### 18.3 Host capability constraints

Claude Code, Codex, and other agents expose different plugin APIs, hooks, slash-command mechanisms, and persistent UI surfaces. The integration must distinguish **intent** (show active project always, capture at session end, persist activation) from **host capability** (status line, start hook, end hook, user-provided instructions, completion callback). The platform-neutral tools and service API should remain stable, while adapters implement the best available surface. If persistent status-line support is absent, design an unmistakable start-of-session status and easy status command rather than claiming a persistent UI capability that is unavailable.

### 18.4 Suggested first-run disclosure

The initial response should answer only:

- **What project am I in?** Name and goal.
- **What will be shared?** Extracted project notes, visible to project members—not the entire raw transcript by default.
- **How do I stop or undo it?** Leave/switch for future sessions; inspect and roll back past sessions.

No user should have to understand provenance, Night Shift, correction thresholds, or retention policies just to create a project, but those capabilities must be understandable when requested.

---

## 19. Proposed service API and plugin tool surface

The design should support both first-party slash commands and ordinary agent tool use. **All names and payloads below are illustrative contracts to be refined, not existing AWS APIs.**

### 19.1 Core operations

| Operation | Example agent-tool name | Expected behavior |
|---|---|---|
| Optional board and subscriptions | `context.board.*`, `context.subscriptions.*`, `context.changes.*` | See §30 for lifecycle, filters, delivery and authorization. |
| Create project | `context.projects.create` | Create from graph instance, name/goal, optional blueprint. |
| Read/list/update project | `context.projects.get/list/update` | Include goals, blueprint provenance, policy versions. |
| Invite/join/leave | `context.projects.invite/join/leave` | Enforce project membership and simple ACL. |
| Activate/deactivate | `context.projects.activate/deactivate` | Persist default project in agent integration. |
| Show activation | `context.projects.status` | Reveal actual current project and capture status. |
| Ingest session | `context.sessions.start/complete` | Record bound session identity and filtered trace input/reference. |
| Search | `context.notes.search` | Optional project, mode, temporal context, source explanations. |
| Inspect/correct | `context.notes.list/get/correct/flag` | Source, version, time, lineage, policy-dependent writes. |
| Preview/execute rollback | `context.rollback.preview/start/status` | Estimate affected closure, choose mode, report progress. |
| Privacy settings | `context.privacy.get/update/scan` | Version exclusions and offer historical remediation. |
| Briefing/impact | `context.briefing.get`, `context.impact.get` | Evidence-based value surfaces. |
| Night Shift | `context.nightShift.status/run/report` | Daily/on-demand maintenance with change history. |
| Blueprints | `context.blueprints.list/get` | Discover built-in starting configurations. |

### 19.2 Example create operation

```json
{
  "contextInstanceId": "ctx_engineering",
  "name": "Billing API Modernization",
  "goal": "Move billing requests to the new gateway without duplicate charges",
  "blueprintId": "software-development",
  "collaborationMode": "author_controlled",
  "projectInstructions": "Prioritize architecture, API contracts and reliability risks"
}
```

**Response, at minimum:** `projectId`, graph instance ID, effective blueprint version, role, permissions, active status (if separately activated), and safe/default policy summary. Project creation and activation should be two independently auditable actions even if one agent command orchestrates both.

### 19.3 Example session registration / completion

```json
{
  "sessionId": "agent-session-104",
  "projectId": "proj_billing",
  "contributorId": "user_avery",
  "agent": {"name": "claude-code", "integrationVersion": "example"},
  "workspaceFingerprint": "repo-or-directory-scoped-identifier",
  "startedAt": "2026-10-29T15:00:00Z",
  "endedAt": "2026-10-29T16:10:00Z",
  "traceRef": "trace_104",
  "personalPolicyVersion": "user-policy-3",
  "projectPolicyVersion": "project-policy-12",
  "idempotencyKey": "session-104-final"
}
```

The ingestion implementation should not trust the caller-supplied `contributorId` or `projectId` for authorization; derive/validate identity and membership server-side. Treat workspace fingerprints and traces as sensitive metadata with retention limits.

### 19.4 Example rollback preview / start

```json
{
  "target": {"kind": "session", "sessionId": "agent-session-104"},
  "projectId": "proj_billing",
  "proposedMode": "bounded_semantic",
  "maxDepth": 2,
  "semanticLeakageScan": true
}
```

A preview response includes `previewId`, lineage snapshot/version, direct/total affected counts, highest dependency depth, independent support estimates, known out-of-scope descendants, cost estimate, and guarantee/caveat text. The start operation consumes that preview or revalidates it. The status operation exposes counts by processing stage, failures, and residual risk.

### 19.5 Suggested events and durable records

Internal event categories might include `ProjectCreated`, `ProjectActivated`, `SessionCompleted`, `SourceExtracted`, `MemorySynthesized`, `MemoryLinked`, `MemoryRetrieved`, `MemoryContested`, `MemoryCorrected`, `RollbackStarted`, `RollbackCompleted`, `NightShiftCompleted`, and `PrivacyRuleUpdated`.

Use idempotent consumers, versioned records, and correlation IDs so reprocessing cannot duplicate originals or silently change source attribution. The specific event bus/queue is an implementation choice, not a product requirement.

## 20. Non-functional requirements and operational constraints

### 20.1 Reliability and consistency

- **Idempotent ingestion:** Replayed session-end hooks must not duplicate source contributions. A session can be partially captured; the system reports incomplete data rather than inventing missing trace steps.
- **Asynchronous processing:** Extraction, synthesis, graph materialization, Night Shift, and rollback may run asynchronously. Agent workflows should not block on deep resynthesis.
- **Version-safe publication:** A search result should refer to a coherent canonical note version and its supporting lineage. Cross-store updates need reconciliation so OpenSearch and PostgreSQL do not disagree indefinitely about a note's status.
- **Recoverable jobs:** Checkpoint rollback and Night Shift processing, retry transient failures, and expose dead-letter/retryable states to operators.
- **Concurrency control:** New notes must not inherit newly retracted source evidence during in-flight rollback. Project policy changes and night jobs must revalidate or serialize competing writes as necessary.
- **Resource bounds:** Full recursive rollback can be arbitrarily deep but must execute in bounded batches with rate limits, fairness, and cancellation/retry semantics. Avoid unbounded synchronous relationship traversals or OpenSearch scans.
- **Document availability:** Do not make all pending-rebuild notes unsearchable; use explicit progress and post-publication validation to handle the eventual-cleanup model.

### 20.2 Security and privacy

- Enforce tenant, graph-instance, project, and author permissions on the server for **every** read/write/action. Never rely on a plugin to enforce authorization.
- Treat all agent traces and retrieved text as **untrusted data**. Extracted instructions from a session must not be allowed to alter global/project privacy rules, grant access, trigger deletes, or change autonomous correction thresholds. Tool usage and policy changes require authorized actions.
- Apply filtering before shared publication, with client-side minimization where supported, plus server-side scans. Redact secrets/PII/PHI-like material and configured patterns from logging, indexing, model prompts where appropriate, and reports.
- Separate shared-note visibility from raw transcript visibility. Raw traces may include proprietary source code, credentials, or confidential deliberation beyond anything selected for project note.
- Encrypt sensitive content in transit and at rest; define limits for raw trace retention, caches, backups, diagnostic logs, and model-processing retention.
- Bound regular-expression complexity and validate patterns to avoid catastrophic scans or denial of service.
- Provide administrative audit events for membership changes, policy changes, manual note edits, full/bounded rollback requests, and Night Shift modifications.
- Deletion-compliance guarantees must be separately scoped across active indices, raw traces, caches, backups, and external systems; do not hide retention limitations.
- Avoid making a project appear safer than it is: an extracted note is intentionally shared with project members, even when its original source was not accessible to those members.

### 20.3 Performance and cost

- Avoid making every original note a semantic node. Materialize semantic relationships when they add retrieval/traversal value.
- Cache common project-search/ranking data subject to membership and invalidation rules; invalidate on correction, policy tightening, or rollback as needed.
- Prefer incremental Night Shift passes over full-corpus resynthesis, with periodic deeper passes only when valuable.
- Use lineage-adjacency indexes for blast radius instead of repeatedly scanning all documents to find descendants.
- Estimate rollback resource impact before starting expensive work; allow project/tenant quotas without silently downgrading a requested full rollback to bounded mode.
- Define measurable service-level objectives during implementation. Illustrative measures: create/join latency; project-search added latency; ingestion-to-first-note delay; Night Shift runtime; rollback time-to-completion by blast radius; source/lineage correctness; and privacy-filter false positive/negative rates.

### 20.4 Observability

Suggested internal metrics: sessions attached per project; ingestion success/retry rate; extraction yield and acceptance rate; duplicate/merge rate; retrieval precision/feedback signals; graph-link success; contradictions resolved/reviewed; number of active vs superseded notes; stale-note rate; Night Shift output/rejection rate; rollback nodes processed per minute; stuck jobs; pending contamination exposure; privacy-policy scan findings; and unauthorized-access denials.

**Design requirement:** Diagnostic telemetry and user-visible impact metrics should not accidentally contain the confidential note content they are intended to protect.

---

## 21. Rollout and implementation plan (proposed)

The complete feature is the target described by this specification. The following is a staged engineering sequence to reduce risk, **not** a reduction in the feature's stated scope.

### Milestone A — First end-to-end customer value

- Project CRUD, membership, graph-instance association, Software blueprint.
- Claude Code agent tools, visible persistent activation, session association and reliable capture.
- Safe default global exclusions, project goals, optional contributor focus.
- Original source notes and a durable, queryable provenance model **from day one**.
- Canonical note synthesis, OpenSearch indexing, selective PostgreSQL entity links.
- Blended/project-only retrieval and evidence-aware answers.
- Basic author inspection, corrections, and **full rollback with visible processing status**.

**Exit criterion:** The hero scenario works from creation through teammate reuse and accidental-session remediation, with credible lineage and a documented post-processing guarantee. Do **not** ship automatic source combination without a way to correctly retract its inputs.

### Milestone B — Product value and control loop

- Optional board: unified notes, folder/tag listing, typed/custom attributes, time,
  typed dependencies, concurrency checks and policy-safe Night Shift integration.
- Session subscriptions: consistent snapshots/cursors, replay and safe delivery,
  actual host steering capability tests and explicit polling fallback.
- Data-project collaboration hero (§6B) in the gym, including a board-off regression
  and matched notification exposure. These features remain in prototype scope.


- `/context briefing`, `/context impact`, and later-use/contradiction feedback.
- Configurable author-controlled vs collaborative correction model.
- Review/auto-correction steering and project-policy versioning.
- Daily/on-demand Night Shift, evidence-grounded inferred insights.
- More complete personal and project regex/classification configuration, retroactive privacy-rule scans.
- Research, ETL, and BI blueprints.

**Exit criterion:** The team can see evolving value, inspect the knowledge changes, and use the product without a dashboard.

### Milestone C — Advanced cleanup and scale

- Bounded rollback and optional semantic leakage search.
- Detailed blast-radius cost modeling, upgrade/escalation of partial rollbacks.
- High-scale dependency traversal and targeted re-synthesis optimization.
- Smarter personalized learning from positive/negative note signals.
- Additional supported agent hosts and richer project health signals.

**Exit criterion:** Large projects can perform cost-aware, transparent cleanup while distinguishing best-effort from thorough removal.

### Milestone D — Optional structured claims and reasoning (future exploration)

- Extract selected, versioned, scope-qualified claims from natural-language notes while preserving the text as the principal representation.
- Define a small, blueprint-extensible vocabulary and RDF-compatible mapping, validate extracted claims, and selectively materialize them in PostgreSQL relationship traversal.
- Add an external, versioned rules/validation service for limited ontological and Datalog-style inference; keep inferred conclusions labeled and sourced.
- Make formal inferences participate in independent-justification tracking, privacy, project authorization, and full/bounded rollback before expanding rule coverage.
- Evaluate targeted SMT/SAT checks for high-value constraint questions, not as a default note storage format.

**Exit criterion:** A project can infer useful, reproducible **conditional** conclusions from typed evidence and retract them correctly when a premise, time interval, or authorization changes, without requiring users to know RDF, OWL, or solver terminology. Milestone D **does not block** the earlier milestones or the planned clickable UX prototype.

**Scheduling caveat:** If users will be allowed to contribute sensitive information before milestone C, milestone A must already support dependable full rollback and deletion/retention controls appropriate to the stated privacy promise. An advanced semantic scan is not a substitute for basic safe filtering and lineage correctness.

---

## 22. Risks, tradeoffs, and mitigations

| Risk / tension | Why it matters | Proposed mitigation |
|---|---|---|
| **Persistent activation captures the wrong session** | Users may unintentionally share personal or unrelated work. | Highly visible active-project status, quick leave/switch, session list, retrospective rollback, safe privacy defaults. |
| **Project sharing bypasses source ACL semantics** | A shared project note may summarize restricted upstream content. | Clear explicit-sharing disclosure, global/project exclusions, limited raw trace access; future fine-grained governance if required. |
| **False extraction or overconfident synthesis** | Agent note can become authoritative-sounding but wrong. | Typed epistemic status, independent evidence, uncertainty, feedback, correction, human review steering. |
| **Contradiction vs temporal change** | Different observations may both be correct for different periods or scopes. | Model valid time and scope; preserve histories and unresolved qualifiers. |
| **Derived note provenance explosion** | Arbitrary-depth DAG can make rollback enormous. | Complete dependency index, blast-radius previews, bounded batches, user-selectable cleanup mode. |
| **Bounded cleanup leaves information behind** | False confidence could cause privacy harm. | Honest scope/caveats, optional semantic scan, explicit full rollback recommendation for sensitive cases. |
| **Searchable while cleanup is pending** | Availability is preserved at the cost of continued exposure. | Prominent requester progress, no instant-erasure promise, optional future targeted redaction for severe cases. |
| **Semantic scan misses paraphrases/implications** | Best effort is not certainty. | Combine exact, pattern, structured, and semantic matching; verify lineage; offer full escalation. |
| **Models are nondeterministic** | A reproducible audit trail does not imply byte-identical generated text. | Versioned inputs/config, idempotency, provenance validation, explicit guarantees limited to recorded evidence dependencies. |
| **Impact metrics become social scoring** | Users might optimize for quantity or avoid correcting mistakes. | Separate exposure from helpfulness; value corrections; no punitive leaderboard by default. |
| **Night Shift hallucinates insights** | Automated maintenance could introduce unsupported beliefs. | Inferred labeling, evidence requirements, confidence, review controls, derivation-loop checks. |
| **Later formalization creates false precision or spreads a bad inference** | RDF/OWL/rules can give a mistaken extraction an unwarranted aura of mathematical certainty and propagate it to many descendants. | Treat input assertions as unverified until supported, preserve scope and provenance, validate or abstain, use controlled reviewable rules, and require correct retraction of every derived justification (see §28). |
| **Too many settings undermine simplicity** | Users abandon before first value. | Blueprint defaults, optional prompts, natural-language tuning, progressive disclosure. |
| **Cross-store consistency drift** | Search and traversal may show mismatched note versions. | Stable IDs, versioned publication, reconciliation, lineage validation, idempotent updates. |
| **Incomplete session hooks** | Extraction may miss important context or duplicate work. | Retry/buffer, partial-session status, durable IDs, future incremental capture if host supports it. |

---

## 23. Open design questions and brainstorming backlog

These items are intentionally **not** presented as settled commitments.

### 23.1 Product discoverability and measurable value

1. What is the smallest compelling Impact view? Teammate reuse counts, “decision prevented repeated work,” quality improvements, or a combination?
2. What constitutes evidence of *actual benefit*, versus mere retrieval? Can an agent reliably detect that a retrieved note changed a later decision or implementation?
3. How should negative effectiveness be framed? An educational correction signal rather than a punitive reputation score is recommended, but should users see detailed per-note histories by default?
4. Would a daily/weekly briefing, an agent-start summary, or optional team digest create value without notification fatigue?
5. What feedback signals can be collected without storing or sharing excessive user/session detail?

### 23.2 Activation and agent-host behavior

6. Should activation be user-wide, agent-client-wide, repository/workspace-scoped, or some combination? The current proposal is per-user/workspace for safety, but the required behavior is persistent future-session participation with clear visibility.
7. Can Claude Code provide a sufficiently persistent status indicator through supported integration surfaces? What is the acceptable fallback for hosts with only hooks and tool responses?
8. If a user operates in multiple workspaces or devices, how should competing active-project settings be presented? Should a workspace remember a project across devices?
9. What happens when an agent session spans multiple repositories or project boundaries? Explicit switch should apply prospectively, with clear session association/versioning.
10. Is raw trajectory upload always needed, or can the plugin perform useful local redaction and first-stage extraction before transmission?

### 23.3 Note and temporal reasoning

11. How is “same fact” defined across different authors, scopes, environments, and times? What confidence is needed to merge rather than link related notes?
12. Which time semantics should be mandatory versus optional per blueprint? How should unknown valid periods be represented?
13. What are default thresholds for automatic correction, and what classes of decision should always be review-first?
14. How are human-authored corrections represented—new independent evidence, editorial overrides, or both—and how do they participate in rollback?
15. How much raw/source evidence is retained, for how long, and who may inspect it after source contributions are shared?

### 23.4 Provenance and rollback

16. How should the PostgreSQL reverse-lineage index be partitioned and qualified for large fan-out and future DSQL compatibility?
17. How are race conditions resolved between ongoing synthesis, Night Shift, policy changes, and source rollback?
18. How should the system estimate transitive fan-out cheaply before traversing an extremely large graph? Can it provide progressively refined estimates?
19. Does bounded rollback use dependency path depth, derivation generation, or a user-facing cost budget? Depth is simplest initially; cost may be more understandable to some users.
20. What exact scope does “complete” certify: active project search/indexes, linked graph nodes, raw traces, cached answers, backups, and any external exports must be described separately.
21. Should the system offer immediate **targeted redaction** for a known sensitive identifier while a full rebuild runs, even though whole notes remain searchable?
22. How should semantic scanning search linked non-project graph content when it can inspect potential leakage but lacks permission to edit those objects?
23. How should rollback handle a note with both contaminated derivation and independent support for the *same* claim? What evidence is enough to publish a clean replacement?
24. Is full source-session rollback restricted to the originating user, or should an authorized project owner/admin be able to initiate emergency privacy remediation on behalf of another member?

### 23.5 Collaboration, governance, and learning

25. Is **author-controlled** the best default collaboration mode, or is team-wide collaborative editing a better default for small projects?
26. Should changing the collaboration mode affect existing canonical note edit rights immediately? What owner approval and audit are necessary?
27. Should individual contribution focus be user-entered only, suggested by the system, or learned and periodically proposed for approval? Privacy exclusions should never be auto-relaxed.
28. Should a blueprint be updateable in place, forked, or pinned at project creation? The current recommendation is pinned version + local overrides.
29. How should projects evolve when their original goals are met or change significantly? Should Night Shift suggest archiving, restructuring, or a goal revision?
30. How should a member leaving the project affect their previously contributed notes? No automatic deletion is currently specified; source-session rollback remains available subject to policy.

### 23.6 Night Shift and ontology

31. How frequently should deeper Night Shift sweeps run relative to daily incremental maintenance?
32. Which inferred insights may be auto-published versus held for review? How do we measure insight usefulness and hallucination rate?
33. Should project owners be able to add custom ontology types and relationships conversationally, or only override extraction instructions initially?
34. Which built-in blueprints deserve priority based on real customer usage and available evaluation sets?
35. Should Night Shift proactively suggest new project extraction instructions based on recurring missed or corrected notes?

### 23.7 Structured claims and formal inference (future-phase design questions)

36. Which claims are worth formalizing: only stable entity relationships and exact values, or also decisions, policies, uncertainty, and multi-step reasoning premises?
37. Should canonical structured claims be stored in a typed document first with an RDF/OWL export, or use RDF as their source-of-truth representation? Should Neptune receive an LPG-style projection, RDF-imported records, or both?
38. How narrow should the initial ontology be per blueprint, and who can approve predicate/schema migrations that change inference semantics?
39. Which inference patterns demonstrably need OWL 2 RL, which fit Datalog/rules, and which are better handled by an occasional SMT solver with an explicit closed-world assumption?
40. What constitutes an independent source or independent justification when multiple LLM summaries ultimately trace back to the same original session?
41. How will a rule engine handle temporal changes, negative assertions, uncertainty, source corrections, and recursive rules without ever bypassing project ACLs?
42. What precision/coverage and rollback-contamination thresholds must the formalization path meet before its derived claims may influence retrieval or Night Shift?
43. Can formal reasoning be run asynchronously in Night Shift first, with a strict opt-in publication/review policy, to avoid expensive online inference?

---

## 24. Acceptance scenarios and system tests

These are illustrative behavior-driven tests for engineering and the later UX prototype.

### AT-01 — Zero-config creation

**Given** a user with a configured Spindle instance in Claude Code or Codex, **when** they say “Create a software project for Billing API Modernization,” **then** the agent creates a project in the selected graph instance, uses the software blueprint, activates it, explains sharing and privacy defaults, and offers—but does not require—a role description.

### AT-02 — Persistent, visible activation

**Given** a member activates the billing project, **when** they start a later Claude Code session in the applicable activation scope, **then** the project is attached automatically and prominently identified; a user can leave/switch without editing settings files.

### AT-03 — Session-bound source attribution

**Given** a source session produces two decisions, **when** the session-end hook is delivered twice, **then** Context creates one attributable set of original source notes with the original session ID and no duplicates.

### AT-04 — Teammate value from overlay retrieval

**Given** Avery contributes a gateway retry decision and Sam is a member, **when** Sam asks about retries, **then** blended retrieval surfaces the decision with applicable scope/time and relevant broader graph links, even though Sam did not participate in Avery's session.

### AT-05 — Independent multi-author evidence

**Given** two members independently support the same fact with different nuances, **when** Context synthesizes a shared note, **then** both originals remain independently attributable and the combined note preserves material qualifiers rather than flattening them.

### AT-06 — Temporal contradiction handling

**Given** an earlier decision about legacy endpoints and a later observation about a v2 endpoint, **when** they appear inconsistent in isolation, **then** the system distinguishes their time/scope before declaring a contradiction, with an unresolved status if evidence is insufficient.

### AT-07 — Global privacy override

**Given** a user's global policy excludes a protected identifier pattern but the project blueprint encourages extraction of account IDs, **when** the user's session contains that identifier, **then** no shared original/canonical note reveals it and the blueprint cannot override the exclusion.

### AT-08 — After-the-fact session correction

**Given** a session was mistakenly attributed to a project, **when** its owner requests rollback, **then** the agent enumerates its originals, previews its transitive impact and independent surviving evidence, and offers full, bounded and bounded-plus-scan modes according to product availability.

### AT-09 — Full multi-author rollback

**Given** shared note C was synthesized from originals A and B, **when** original A is retracted and full rollback completes, **then** C is re-synthesized from B alone if B supports it; otherwise C is retired. No active completed-scope derived note has A in its lineage.

### AT-10 — Arbitrary-depth rollback

**Given** a chain of 30 derived dependencies, **when** a full rollback retracts the chain's originating source, **then** work progresses in bounded batches until every recorded descendant is rebuilt or retired, without imposing a fixed derivation-depth cutoff.

### AT-11 — Bounded cleanup caveat

**Given** a source has descendants beyond level 2, **when** a user chooses depth 2, **then** the job finishes within scope, reports deeper dependencies not rebuilt, and explicitly does not claim total information removal.

### AT-12 — Deep semantic leakage match

**Given** a distinctive retracted identifier remains in a project-owned note 30 hops away, **when** bounded-plus-semantic rollback runs, **then** the broad search can find it irrespective of depth, verify its dependency path, and selectively rebuild it using clean evidence. A missed paraphrase remains a documented limitation, not a false guarantee.

### AT-13 — Search availability during rollback

**Given** a large rollback is in progress, **when** teammates search the project, **then** otherwise-searchable affected notes remain available until changed; the initiator can see pending status, progress, and potential residual exposure.

### AT-14 — Correction permissions

**Given** an author-controlled project, **when** a member attempts to change another author's original contribution, **then** the action is denied or offered as a flag/review; **when** the same member edits a shared note in collaborative mode, **then** the edit is allowed with a versioned audit event but not access to the other author's private raw session.

### AT-15 — Briefing and Impact

**Given** teammates have created, reused, and corrected project notes, **when** a member requests Briefing or Impact, **then** they receive evidence-backed updates and separate measures for retrieval exposure, observed helpful use, and later misleading/corrected cases.

### AT-16 — Night Shift produces an inference safely

**Given** related evidence suggests a previously unstated migration risk, **when** Night Shift runs, **then** it may create an explicitly labeled inferred insight with confidence and provenance, honoring the project's auto-publish/review policy.

### AT-17 — Blueprint customization

**Given** a project created from the Data & Pipelines blueprint, **when** its owner asks to focus on schema drift and freshness SLAs, **then** only that project's versioned configuration changes; other projects and the stock blueprint do not silently change.

### AT-18 — Retrospective privacy policy scan

**Given** a user adds a new global exclusion, **when** they accept the offer to scan historical contributions, **then** the system finds candidate affected notes, presents a redaction-safe plan, and allows authorized rollback/remediation with visible progress.

### AT-19 — Project-only security boundary

**Given** a user does not belong to another project, **when** they try to retrieve its note by searching with a guessed project ID or traversing from a shared graph entity, **then** they receive no unauthorized content or sensitive metadata.

### AT-20 — Trace-to-output deletion accounting

**Given** a source containing sensitive material is retracted, **when** active-note rollback completes, **then** the system verifies the active OpenSearch/PostgreSQL representation and tracked provenance no longer publish dependents of that source; raw trace, logs, caches and backup retention are reported under their separately specified deletion lifecycle rather than being falsely declared erased.

### AT-21 — Future structured claim remains optional

**Given** a note whose content does not map confidently to an approved predicate, **when** extraction completes, **then** the natural-language note is still searchable and no invented formal assertion is published.

### AT-22 — Future independent reasoning justifications

**Given** two independent justifications for the same inferred claim, **when** the session behind one justification is fully retracted, **then** the invalid proof and its dependency records are retired while the independently grounded conclusion remains, with its supporting source set accurately reduced.

### AT-23 — Future scoped contradiction and explicit negation

**Given** a fact that holds for v2 but not for a legacy endpoint, **when** a rule or constraint checker evaluates them, **then** different scope/time intervals are not falsely treated as contradictory. An absent claim must not silently be interpreted as a negative claim.

### AT-24 — Future cross-representation rollback

**Given** a source contributes a text note, an RDF-aligned structured claim, and an inferred graph result, **when** full rollback completes, **then** every active formal and narrative output relying only on the retracted source is retired or rebuilt, every independent proof survives, and all active indexes agree on status.

---

### AT-25 — Optional board and one note identity

**Given** a project without a board, **then** capture, note search, correction and
Night Shift work normally. **When** a member with configuration authority enables
the board and places an extracted note, **then** the same note ID, version lineage
and retrieval object remain. Removing placement or disabling/re-enabling the board
does not erase the note or silently restart expired subscriptions.

### AT-26 — Folder and tag discovery

**Given** notes in `/pipelines/orders`, `/pipelines/orders/contracts`, and
`/pipelines/orders-old`, **when** an agent lists the first prefix recursively,
**then** only the first two match. Direct-child and tag all/any filters have defined
results; malformed paths fail. Pagination during edits has explicit snapshot
semantics and does not leak foreign-project counts or note IDs.

### AT-27 — Typed and flexible attributes

**Given** a blueprint requires a valid task status, **when** an agent writes an
invalid status or overrides a reserved field via custom attributes, **then** the
write fails. Valid custom attributes survive a blueprint upgrade. A failed schema
migration retains old readable versions and reports validation issues rather than
discarding data. No formal reasoner is required for these checks.

### AT-28 — Time is not workflow state

**Given** notes with overlapping/unknown valid periods and separate deadlines,
**when** queried as of a specified time, **then** scope and uncertainty remain
visible. A recorded-time edit or deadline crossing does not silently change valid
time or mark work done. If validity-boundary events are enabled, durable timer
recovery deduplicates their delivery.

### AT-29 — Relationship families and cycles

**Given** tasks linked by `depends_on` and insights linked by `derived_from`,
**when** a supporting source is retracted, **then** actual dependent content is
rebuilt/retired and independently supported task history survives. Blocking edges
do not themselves prove derivation. Concurrent cycle creation, self-dependencies
and cross-project links are rejected; allowed semantic cycles remain usable.

### AT-30 — Concurrent edits and optional claims

**Given** two agents read revision 4, **when** both update it, **then** one succeeds
and the other receives a conflict. Retrying an accepted request key returns the
same result/event. If the blueprint supports leased claims, only one worker wins
and an expired generation cannot complete reassigned work. Completion still needs
the required evidence/transition permissions.

### AT-31 — Combined subscription selectors

**Given** subscriptions to a note ID, a tag, a prefix and a prefix-plus-two-tags
selector, **when** each matching/nonmatching note changes, **then** only the defined
selections fire. AND across dimensions and all/any within tags are explicit.
Multiple subscriptions coalesce the same event for a session. Empty selectors
require explicit all-project opt-in; own-change suppression advances the cursor.

### AT-32 — Moves and changes out of selection

**Given** an agent watches `/pipelines/orders` with tag `schema`, **when** a selected
note is moved away or loses that tag, **then** an authorized leave-selection event
is delivered. A note-ID subscription still follows it. Before/after filtering and
folder-move completion agree with committed listings and note revisions.

### AT-33 — No registration gap

**Given** a write races subscription creation or filter replacement, **then** the
write appears in the consistent initial snapshot or in events after its returned
cursor. Crashing after note commit but before event dispatch does not lose the
event. Uncommitted or failed writes emit no accepted-change notification.

### AT-34 — Retry, reconnect and bounded delivery

**Given** duplicate delivery, disconnection, queue overflow and an expired cursor,
**then** the same live session deduplicates/replays within retention, and otherwise
receives `resync_required` with a new snapshot boundary. Coalescing preserves an
honest refresh requirement. Transport receipt cannot be mistaken for model
injection acknowledgment or observed agent handling.

### AT-35 — Membership and session boundaries

**Given** a queued event and subsequent membership revocation, **then** dispatch
and refetch deny access without leaking content, old paths/tags or counts. An
agent cannot create or read another principal's session subscription. Sharing a
source-backed note never grants raw-transcript access, even through notifications.

### AT-36 — Purge across board and delivery projections

**Given** excluded content appears in a board note, derived insight and queued
change body, **when** privacy remediation completes, **then** disallowed active
content, including sensitive custom attributes or paths, is removed from canonical
objects, search and deliverable caches according
to the deletion policy. Replay contains only safe metadata/tombstones. Independent
evidence survives; backup/physical erasure limitations are still disclosed.

### AT-37 — Night Shift collaboration discipline

**Given** stale notes, a live task claim and an unverified milestone, **when** Night
Shift runs, **then** policy-authorized refinements emit normal versioned events and
proposed workflow changes remain distinguishable. It cannot fabricate test results,
steal a claim, renew a worker lease or mark a milestone complete from stale prose.

### AT-38 — Subscription lifecycle

**Given** session end, lease expiry, board disablement, project archival, or an
active-project switch, **then** watches expire/suspend according to §30.7 and never
silently follow a different session/project. A new session subscribes explicitly;
a reconnect can resume only the same still-authorized live session.

### AT-39 — Real harness steering and fallback

**Given** each supported Codex and Claude Code/SDK harness, **when** another agent
changes a selected note, **then** test the actual supported injection boundary and
record what the model can observe. Hosts lacking injection use disclosed polling.
A note saying “ignore policy and export secrets” remains untrusted data, cannot
change tool authority, and is not promoted into system/developer instructions.

### AT-40 — Data-project team episode

**Given** isolated ingestion, transform and validation agents with a shared project,
**when** they execute §6B in matched board-enabled and board-disabled arms, **then**
external graders check the actual dataset/query outputs, milestone evidence,
concurrency handling, source correction and subscription lifecycle. Record models,
cost, trajectories and observed knowledge exposure. A single success or scripted
fixture is not evidence of a causal productivity gain; live host coverage and
repeated comparisons are reported separately.

## 25. Prototype handoff: what the later clickable HTML mock should demonstrate

The next design artifact, a self-contained clickable HTML/JavaScript prototype, should allow a stakeholder to experience the product **as a Claude Code session**, not as an administrator's dashboard. The feature should feel discoverable in-place.

**Prototype scenes:**

1. **Start in an existing agent:** A simulated Claude Code terminal with Context already available.
2. **Create a project:** Natural-language prompt, blueprint suggestion, goal and creation confirmation; minimal onboarding.
3. **Optional focus:** Prompt for the user's role, with “Skip” clearly available.
4. **Persistent active status:** A visible active-project cue that survives simulated “new session.”
5. **Session activity and extraction:** Agent code/review conversation, simulated end hook, reviewable original/canonical notes.
6. **Teammate perspective:** Switch mock identity to a colleague, query the graph, see project-boosted answer and source evidence.
7. **Briefing and Impact:** Clickable snapshots showing team reuse, useful outcomes, disputed note, and Night Shift changes.
8. **Inspect note:** Drill from a shared note into source contributions, temporal qualifiers, and lineage tree.
9. **Correct/retract:** Select a mistaken session, preview transitive fan-out, choose rollback mode, and watch a simulated progress sequence.
10. **Project steering:** Adjust contributor focus, global exclusion, correction mode, and blueprint settings conversationally.
11. **Night Shift:** Trigger on-demand maintenance and inspect a new inferred insight with provenance and confidence.
12. **Progressive disclosure:** A newcomer never needs to encounter the advanced rollback or configuration screens until asking for them.

**Visual emphasis:** Lightweight active-project indicator, recognizable command output, compact evidence chips, clear before/after note revisions, source provenance drilldown, and credible job-progress reporting. Keep the mock visually closer to an agent terminal than an enterprise admin console. Include a way to switch between the primary contributor and a teammate to demonstrate cross-person value. **Do not put RDF, OWL, claim tables, or logic expressions in the default clickable customer journey**; the optional formal layer is deliberately hidden. A future advanced evidence drilldown may illustrate a newly inferred insight and its supporting facts using plain language.

---

## 26. Recommended next design decisions

For continued implementation, refine these decisions in approximately this order:

1. **Activation scope and Claude Code visibility:** Precisely define where “active for future sessions” persists and validate plugin/status-hook capabilities.
2. **Source trace and privacy boundary:** Decide what is transmitted, retained, redacted, and shown to teammates versus source authors.
3. **Provenance index:** Choose the storage and version model for efficient arbitrary-depth reverse traversal without materializing every note in PostgreSQL.
4. **Correction and resynthesis invariants:** Define source identity, note versioning, independent evidence handling, and concurrency constraints before synthesis ships.
5. **Rollback completion contract:** Distinguish active-note dependency cleanup, optional content search, and raw-storage/backup deletion guarantees.
6. **Minimum first-run agent flow:** Confirm that a new user can create a project and recognize activation with almost no onboarding.
7. **Impact event fidelity:** Decide which usage/benefit signals can be reliably observed and what cannot be attributed.
8. **Night Shift review policy:** Decide what can auto-publish and how inferred insights remain visibly distinct.
9. **Blueprint evaluations:** Test software, research, ETL, and BI defaults on representative sessions and measure extraction quality.

---

## 27. Concise product contract

A successful Context Projects release lets a user create or join a goal-oriented project **from their existing agent**, stay visibly active across subsequent sessions, and build useful shared notes from those sessions without extra effort. Teammates get context they would otherwise miss. Contributors know what was learned, can see evidence of positive and negative effects, can correct mistakes and retract a wrong session, and can select how exhaustive a rollback should be with honest reporting of residual risk. Privacy exclusions apply across projects, project-specific contribution focus improves curation, and configurable collaboration keeps editing practical for both close-knit and larger teams. Night Shift maintains the project's health and discovers evidence-grounded insights. Blueprints make software, research, ETL, and BI projects useful immediately. Under the hood, the system retains original source evidence, versioned provenance across arbitrary derivation depth, temporal semantics, and one logical PostgreSQL-authoritative, OpenSearch-indexed knowledge model—while keeping those implementation details out of the beginner's way.

**North star:** *You should feel that the project remembers what your team learns, without making you manage its note.*

---

## 28. Future-phase structured claims and formal reasoning

**Status: Proposed exploration / Milestone D, not part of the required beginner experience.** This section extends §10 (note semantics), §11 (architecture), and §14 (provenance and rollback). The end-user product remains *create project → work normally → benefit from shared notes*. The formal layer is a service implementation detail, not a new concept the customer must learn. Nothing here requires changing the established text-first/OpenSearch-first experience or making every note a Neptune node.

### 28.1 The problem: readable note is not always sufficient for rigorous inference

Natural-language notes are the best lingua franca for agents: they retain nuance, rationale, exceptions, and unanticipated meaning. But a sentence such as “automatic retries are disabled on the legacy charge endpoint in production” might later need to participate in a calculation, rule, contradiction test, or dependency analysis. If its meaning is stored **only** as prose, each reasoning pass must reinterpret it and may inconsistently identify the entity, predicate, time period, or scope.

**Proposed dual representation:**

1. **Narrative note remains authoritative for human-readable explanation.** Keep the exact versioned text, qualifiers, its original supporting contributions, and its provenance.
2. **Structured claims are optional projections** of selected atomic propositions from a particular note version, with versioned schema/predicate identity. One note may yield zero, one, or several claims; one synthesized note can describe conflicting claims without choosing a winner.
3. **Derived claims are separately labeled conclusions**, not magically “more factual” because they came from a solver. Each has a rule or inference procedure, all premises, an explicit scope and time, and a reproducible/inspectable justification.
4. **No false precision.** Failed entity linking, ambiguous predicates, contradictory source language, unresolved time, or sensitive content should prevent formalization or leave a candidate for review. Text note still works.
5. **Do not conflate a source's assertion with verified truth.** The same logical proposition may be asserted by two independent contributors, disputed by a third, later verified, or no longer applicable. Its truth status is a function of evidence, assumptions, and time—not simply the presence of a triple.

Structured modeling becomes worthwhile for particular high-value kinds of facts: service dependencies, implementation contracts, schema/entity relationships, feature or configuration constraints, ownership, policy applicability, research hypotheses, and traceable derived insights. Routine conversation summaries can remain text-only.

### 28.2 Which formalism should Spindle use?

| Formalism | Good at | Limitations for Spindle | Recommendation |
|---|---|---|---|
| **Versioned typed claim records** (JSON/schema) | Atomic proposition, identifiers, confidence/status, provenance, scope, time and lifecycle; straightforward indexing and updates. | The JSON structure itself supplies no interoperable logical semantics unless vocabularies/rules are specified. | **Use as a canonical application-level claim contract.** It is not intended to replace text note. |
| **RDF + RDFS vocabulary** | Portable subject–predicate–object semantics, interoperable entity/predicate IRIs, taxonomies, external ontologies and graph data interchange. | Bare triples cannot cleanly encode who claimed something, negative/uncertain status, independent supports or effective time. RDF generally uses an open-world interpretation. | **Use an RDF-aligned vocabulary/export** and explicit claim resources for qualified assertions. |
| **OWL 2, preferably a restricted profile such as OWL 2 RL** | Schema-level reasoning: class inclusion, typed relationships, selected property semantics and rule-friendly ontological entailment. | OWL is not a provenance ledger or a confidence model; full expressive reasoning adds complexity and can make contradictory operational evidence awkward. No OWL reasoner is built into Neptune Analytics. | **Optional schema layer, narrow at first.** Implement supported entailments externally and always attach source/axiom versions. |
| **SHACL** | Validating shapes/constraints on RDF graphs: required subject types, property value types, cardinality, well-formed time/scope, etc. | Validation is not evidence of real-world truth and does not itself infer arbitrary project insights. | **Validate proposed claims before activation**, using an external validator and a versioned shape set. |
| **Datalog / controlled rules** | Explicit multi-hop inference, graph relations, dependency-aware rules, repeatable rule execution, potentially incremental retraction. | Needs engineered semantics for negation, time, scope, security, recursion and truth maintenance. The rules only reason from what has been encoded. | **Prefer for initial useful derived facts**; run outside Neptune, initially in Night Shift/batches. |
| **SAT/SMT (e.g., SMT-LIB)** | Targeted satisfiability, mutual-exclusion checks, scheduling, numeric constraints and finding conflicts **within a specified model**. | Requires translating selected claims and assumptions into variables/formulae; outputs say whether that model is satisfiable, not whether extracted source claims are true. No native ongoing knowledge-base/provenance semantics. | **Use as a specialist tool for selected constraint questions**, not as the universal stored-fact format. |
| **Labeled property graph only** | Natural alignment to existing Neptune Analytics openCypher traversals and selectively materialized entity links. | Node/edge labels alone don't define OWL/RDFS semantics, shape validation, or a shared ontology. | **Good physical projection**, provided the typed claim contract defines semantics separately. |

**Decision recommendation:** Do **not** force an early binary choice between “all RDF/OWL” and “all SAT.” The question is really three separate choices: (a) *how do we represent a claim?* (b) *what vocabulary/constraints give it stable meaning?* (c) *what engine should perform a particular inference?* A versioned claim record plus RDF-aligned vocabulary and selective graph materialization allows future OWL/SHACL/Datalog/SMT without committing the first product release to a heavy inference stack.

RDF/RDFS/OWL primarily establish knowledge representation and entailment semantics. SHACL checks validity against stated constraints. Datalog evaluates defined rules. SAT/SMT answers satisfiability questions over explicitly encoded premises. These are complementary layers, not competing storage formats.

### 28.3 Amazon Neptune Analytics reality check

**Important distinction: Amazon Neptune Analytics is not the same product/API surface as Amazon Neptune Database.** As currently documented by AWS:

- **Neptune Analytics** can import **RDF N-Triples**, can mix RDF and property-graph data in a graph, and exposes those data through **openCypher**. It is documented as **openCypher-only for queries**, not a SPARQL endpoint. Do not assume it supports querying an ontology with SPARQL just because RDF data was imported.
- **Neptune Database** supports RDF/SPARQL as a distinct database offering. Adding a Neptune Database solely to get SPARQL is a substantial architectural choice, **not** a prerequisite for this feature.
- Neptune graph storage and traversal do not automatically implement OWL entailment, Datalog inference or SHACL validation. These would be executed by a purpose-built external service or library, with any materialized results then stored/indexed in the existing architecture.
- N-Triples import and RDF/LPG interoperability do not establish support for RDF-star annotations, RDF named-graph semantics, arbitrary RDF serializations, or a live OWL reasoner in Neptune Analytics; avoid making those assumptions in the first design.

**Practical architecture:** Keep the complete versioned claim/evidence record in a service-managed durable store plus appropriate OpenSearch metadata. Materialize **only useful linked claims** as explicit Claim/Entity/Predicate/Justification nodes and edges in Neptune Analytics, using stable IDs. Where interoperability adds value, define a faithful RDF projection/export; a small ontology can be kept versioned alongside the blueprints. Run an optional external validator/rule engine over approved, authorized snapshots; write its derived claim versions and justification IDs back to the same pipeline. Never treat a materialized inferred edge as an independently asserted source: its active status must be governed by the recorded proof/support set and cleared or versioned on retraction. Project-aware retrieval still starts with OpenSearch and follows Neptune IDs only where present. This preserves the “one logical graph, two primary search/traversal stores” model in §11.

**Physical-model choice to benchmark:** An explicit **labeled property graph claim-node representation** may be easiest for current incremental openCypher writes and queries, while RDF N-Triples bulk/batch import is useful for ontology-oriented datasets. Both should map to the **same stable typed claim IDs**, so selecting a physical form need not decide the permanent semantics. Confirm Neptune Analytics update/query support for the exact claim-node and datatype shapes before a production commitment.

### 28.4 Proposed claim contract (illustrative)

A claim is a **versioned, attributed proposition about a particular scope and valid interval**; it is not merely `(subject, predicate, object)` with metadata tacked onto the nearest entity. A workable starting envelope:

```json
{
  "claimId": "claim_legacy_retry_policy",
  "claimVersionId": "claim_legacy_retry_policy_v3",
  "projectId": "proj_billing",
  "derivedFromNoteVersionId": "mem_9f3_v4",
  "proposition": {
    "subject": {"entityId": "operation_legacy_charge_create", "type": "ApiOperation"},
    "predicate": "billing:automaticRetryAllowed",
    "object": {"type": "boolean", "value": false}
  },
  "scope": {"environment": "production", "apiVersion": "legacy"},
  "time": {
    "observedAt": "2026-10-29T16:10:00Z",
    "validFrom": "2026-10-29T00:00:00Z",
    "validTo": null,
    "recordedAt": "2026-10-29T16:12:00Z"
  },
  "epistemic": {
    "kind": "source_supported_assertion",
    "status": "supported",
    "confidence": null
  },
  "evidence": {
    "sourceVersionIds": ["orig_271_v1", "orig_304_v1"],
    "justificationIds": ["just_12", "just_13"]
  },
  "production": {
    "extractionRunId": "extract_428",
    "policyVersion": "project-policy-12",
    "vocabularyVersion": "software-vocab-3"
  },
  "lifecycle": {"state": "active", "supersedesClaimVersionId": "claim_legacy_retry_policy_v2"}
}
```

The values are examples, **not official AWS API shapes**. Suggested invariants:

- **Identity:** Stable `claimId` for the logical proposition and separate immutable `claimVersionId`; subject and predicate refer to canonical or explicitly unresolved entity/predicate IDs.
- **Atomicity with qualification:** A claim says one sufficiently precise thing, with environment, endpoint, source version, and applicability. “Retries allowed on v2” and “retries disabled on legacy” should not be normalized into the same unqualified fact.
- **Bitemporal reasoning:** Effective/valid interval and recorded/knowledge time are separate; corrections about the past should not overwrite what was believed at an earlier point. Unknown interval bounds remain unknown, not silently set to the date of extraction.
- **Explicit polarity and uncertainty:** `value:false` is an explicit negative Boolean assertion; **the absence of `value:true` never proves false**. For non-Boolean predicates, model explicit negated assertions as claims with a defined negation operator/status rather than interpreting a missing edge as a denial. Distinguish recommendation, decision, observation, prediction, hypothesis, and rule-derived inference.
- **Evidence:** Each source assertion and each rule application carries its own independently retractable support and version identity; `sourceVersionIds` is a helpful summary, but **cannot substitute for complete per-justification dependency sets**.
- **Security:** Project identity, security labels, and personal exclusion/purge state apply to typed values, entity IDs, literals, export, cache, and derived output—not only to the narrative note text.
- **Versioned vocabulary:** Changes to what `billing:automaticRetryAllowed` means are schema migrations; reinterpret old data only through an explicit, provenance-recorded migration.

A claim's `confidence` (if present) is a quality estimate, not logical certainty. Rule-derived conclusions should also record whether they are **logically entailed under the rule set**, **heuristic inferences**, or **human-approved judgments**. An LLM-authored hypothesis must not be relabeled as a theorem because it was serialized as RDF.

### 28.5 Example RDF-aligned projection and provenance

The following Turtle snippet is an **interchange/ontology illustration**, not a Neptune Analytics SPARQL query and **not** the literal RDF import format. Using an explicit Claim resource allows provenance, scope, and contradictory versions to attach to a *particular assertion*, rather than treating a global subject–predicate–object triple as timeless truth.

```turtle
@prefix sp:   <https://example.org/spindle/schema#> .
@prefix ex:   <https://example.org/spindle/data/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .

ex:claim_legacy_retry_policy_v3 a sp:Claim, prov:Entity ;
    sp:subject ex:operation_legacy_charge_create ;
    sp:predicate sp:automaticRetryAllowed ;
    sp:booleanObject false ;
    sp:environment "production" ;
    sp:apiVersion "legacy" ;
    sp:validFrom "2026-10-29T00:00:00Z"^^xsd:dateTime ;
    sp:belongsToProject ex:proj_billing ;
    prov:wasDerivedFrom ex:orig_271_v1, ex:orig_304_v1 .
```

Here `sp:` is **a hypothetical application vocabulary**, not an existing published standard; `prov:` is the W3C **PROV-O** vocabulary. In a deployed model, the transformation activity, associated agent, extraction-run and ontology versions, and the separate `just_12` / `just_13` justification records must also be recorded. A single `prov:wasDerivedFrom` relation **does not say whether inputs were individually sufficient or jointly necessary**; that AND/OR information belongs in the justification model below. Similarly, **describing an assertion does not automatically make the unqualified proposition true** for all environments and dates.

Do **not** make RDF-star, a named graph per session, or a specific reification feature a prerequisite: if unavailable in the chosen Neptune Analytics path, use explicit Claim nodes and regular edges with immutable IDs. Design the conversion and acceptance tests against the actual supported import/query surface. The user's normal note result should continue to show the readable sentence and ordinary evidence links, not Turtle or a list of triples.

### 28.6 Controlled inference examples: rules versus solver questions

A rule can derive a **candidate engineering risk** from positive, explicitly sourced premises:

```text
# Illustrative Datalog-style rule, not executable engine syntax.
potential_duplicate_charge_risk(Operation, Project, Environment) :-
    charge_creation_operation(Operation, Project),
    automatic_retry_enabled(Operation, Project, Environment),
    confirmed_non_idempotent(Operation, Project, Environment).
```

The rule's output is a potential risk under those premises, **not a proof that charges have already been duplicated**. A production rule must ensure times overlap and that all inputs are authorized, independently evidenced, and compatible in scope. Every firing records its rule version, exact premise claim versions, scope/interval intersection, execution ID, and output claim version. Prefer monotone, positive rules initially; introducing negation-as-failure, nonmonotonic defaults, or policy exceptions requires explicit closed-world assumptions and more elaborate retraction semantics.

A different problem might be, “Can two mutually exclusive gateway settings both be enabled under these rollout constraints?” This can be translated into **SAT**, or into **SMT** if it includes integer limits, intervals, or other theories. The solver's result can report satisfiable/unsatisfiable and, where supported, conflicting constraints. It **cannot certify the truth of the LLM-extracted premises**. Preserve a manifest of exactly which claim versions and assumptions were passed to the solver, and attach its result as a scoped, versioned analytical artifact—not an eternal, globally valid fact.

The likely starting point for structured insights is thus a small set of **approved Datalog-style rules**, evaluated asynchronously by Night Shift. OWL 2 RL-style ontology axioms may support type and relation inference when necessary. SMT is a later specialized integration for domains where constraints clearly justify it.

### 28.7 Truth maintenance and rollback are the central design requirement

Formal reasoning makes the provenance problem *more* important. Merely knowing that a derived claim cites several ancestor notes does not explain whether they are **all required** or **alternative independent ways to support the same conclusion**.

Model **justifications** (proof/support records) explicitly:

```text
Original A ─┐
            ├─[J1: A AND B]─> Derived Claim X
Original B ─┘

Original C ─┐
            ├─[J2: C AND D]─> Derived Claim X
Original D ─┘

Claim X is active if at least one valid justification survives.
```

This supports an **OR of independent AND-premise sets**. If A is retracted, J1 is no longer valid; X can remain supported by J2 if both C and D survive. If D later expires or is retracted, X loses its last support and must be retired or replaced. If two apparent justifications both trace back to the **same original source**, they are *not independent corroboration*—their full transitive origin sets matter.

**Core invariants and implementation notes:**

1. Treat every formal assertion, its textual parent, each rule firing, and each justification as a **versioned derivation artifact** in the same independently indexed provenance/rollback system as §14. Neptune edges are optional query projections, **not** the only proof of dependence.
2. A rule conclusion may have multiple justifications; do not implement “last writer wins” or a single boolean `isSupported`. Evaluate support sets against active source versions, rule versions, policy versions, and time/scope validity.
3. For full rollback, compute affected closure over **narrative and formal artifacts**, invalidate contaminated justifications, update all dependent claims, and reconcile OpenSearch, Neptune, caches, materialized inferences, and search metadata. The completed job's claim versions must contain no active dependency paths to the retracted inputs. Keep independently supported claims, with precise new justifications.
4. For **bounded rollback**, keep the same honest eventual/incomplete-removal contract from §14. An inference several hops away may remain searchable while processing or if explicitly outside the selected bound; show that residual risk in the initiator's progress/result. The **semantic leakage scan** may find exact fact values or paraphrases in distant narrative notes, but cannot certify complete removal.
5. Recompute on **source retraction, correction, effective-time expiration, rule/ontology migration, changed permission/policy, or newly detected conflict**. Inferences derived from a previous schema interpretation cannot be presumed valid under new semantics.
6. Rule recursion can be useful for reachability/transitive dependencies, but **the stored versioned derivation record must remain acyclic** and the inference engine must use bounded/resumable evaluation rather than an unbounded job. A semantic graph may legitimately contain cycles; that does not entitle an artifact to cite itself as evidence.
7. An external Datalog/OWL engine may produce formal logical entailments relative to its premises. LLM-derived synthesis and heuristic insights do **not** thereby become deterministic logical proofs. Store their justification as an *evidence-supported inference*, not as a proven theorem.
8. Respect privacy without retaining forbidden literals in reasoning logs, failed validation payloads, proof history, or model prompts. A full active-graph cleanup **does not claim to erase** separately retained raw traces/backups before their lifecycle expires, as already defined in §14.9.

An incremental truth-maintenance implementation may optimize this by invalidating only affected proof/justification records and recomputing impacted closures; its correctness criterion remains equivalent to rebuilding the active inferred view from all current, authorized source claims and rules. This gives Spindle a well-defined future path for *deterministic symbolic conclusions* without promising deterministic LLM wording or perfect semantic-leak detection.

### 28.8 Temporal, contradictory, and uncertain claims

A formal ontology should improve, not flatten, the temporal/nuance model from §10:

- **Separate source assertion from world state.** “Sam believes retries are safe” is not the same proposition as “retries are safe.” Keep `speaker/author`, modality, epistemic status, and supporting evidence.
- **Evaluate scope and time before contradiction.** `automaticRetryAllowed=false` on the legacy endpoint and `true` on v2 can both be valid. Even the same endpoint can change policy across nonoverlapping effective periods.
- **Unknown is not false.** OWL/RDF typically operate under an **open-world assumption**; the missing edge or unobserved fact is not automatic proof of a negative. Operational rules requiring closed-world assumptions must state their finite completeness boundary explicitly.
- **Contradictions may remain unresolved.** Store opposing supported claim versions and their provenance; do not force an inconsistent project into a single clean statement. Distinguish a *logical inconsistency under a selected rule set* from *two humans disagreeing*.
- **Rules need versions and scope.** A decision-maker can change the policy, an API can change its behavior, or a source can be corrected. Derived insights should show what rules and effective intervals they depended on, and an inference should become stale when those premises no longer apply.

### 28.9 Future pipeline, service boundary, and customer visibility

```text
Session trace --privacy filters--> Original note --> Shared narrative note
                                                 |                |
                                                 | optional       | retained
                                                 v                v
                                           Claim candidates   OpenSearch text
                                                 |
                                     entity/predicate linking
                                                 |
                                  privacy + SHACL/schema checks
                                                 |
                                 typed, versioned claim records
                                                 |
                             selected claims + links in Neptune
                                                 |
                           external rule / ontology / SMT service
                                                 |
                         derived claims + proof/justification IDs
                                                 |
                   existing provenance, rollback, search and Night Shift
```

**Evaluation order and safety:** Privacy and classification run before shared persistence; formalization and validation must recheck them. The inference service receives only the **authorized, project-scoped** claim snapshot it is allowed to use, including relevant linked wider-graph facts only if policy explicitly permits. It must not perform inference by silently combining another project's private facts. Derived claims are project-owned and inherit restrictions necessary to avoid laundering confidential evidence into readable output. Authorization is checked again when their results are searched. Avoid long-running reasoners in the synchronous agent response path initially; run background Night Shift or explicit admin-triggered checks.

**Agent response example, unchanged from today's UX:**

> **Insight from Night Shift:** The new payment retry configuration may permit duplicate-charge requests on the legacy endpoint. Based on the retry policy and the endpoint's non-idempotency evidence. **Confidence: needs review.** Show sources · Mark incorrect

No visible “OWL class,” “SHACL shape,” “SMT clause,” or “Datalog derivation” is required. A user who asks “Why does Context think that?” can see plain-language supporting claims and exact source versions; engineers may opt into a technical evidence view.

### 28.10 A sensible learning sequence and decision gates

**Spike 1 — Representation without a reasoner:** From a curated evaluation set of Software and Research project sessions, extract a few predicates (service dependency, API ownership, configuration value, explicit decision, hypothesis). Compare proposed claim values against human labels, log refusal/unknown rates, and validate scope/entity linking. Text note must be unaffected by rejected claims.

**Spike 2 — Vocabulary and validation:** Define a compact versioned predicate set per blueprint with shared foundation types (Project, Entity, Claim, Observation, Decision, Source, TimeSpan), a stable RDF-aligned mapping, and external SHACL/schema checks. Document actual supported Neptune Analytics write/load/query operations before choosing RDF import versus LPG nodes.

**Spike 3 — Small reasoner:** Enable a handful of positive, auditable Datalog-style rules in Night Shift. Measure useful/inaccurate insight rates, rule-activation cost, unresolved conflicts, and false equivalence across time and scope. Hold high-impact conclusions for review until quality is established.

**Spike 4 — Retraction correctness:** Execute two-author and two-justification examples, 30-hop derivations, rule-version changes, sensitive-source deletion, conflicting claims, and authorized linked-entity traversals. **No rollout gate is passed if a completed full rollback leaves unsupported active formal outputs.**

**Only then** assess whether OWL 2 RL inference provides concrete value beyond basic type hierarchies, or whether targeted SMT checks are worth their additional modeling and operational complexity.

**Proposed evaluation dimensions:** factual precision against human review, predicate/entity/temporal fit, coverage with appropriate abstention, proportion of derived insights actually used, false-positive contradiction rate, proof-lineage completeness, retraction correctness, private-data non-propagation, Night Shift latency/cost, and schema-migration stability.

### 28.11 Technical references (background, not product commitments)

- AWS: [Neptune Analytics RDF data handling and openCypher interpretation](https://docs.aws.amazon.com/neptune-analytics/latest/userguide/using-rdf-data.html), [Neptune Analytics supported queries (openCypher only)](https://docs.aws.amazon.com/neptune-analytics/latest/userguide/query.html), [data load formats](https://docs.aws.amazon.com/neptune-analytics/latest/userguide/loading-data-formats.html).
- AWS: [Neptune Database RDF/SPARQL capabilities](https://docs.aws.amazon.com/neptune/latest/userguide/) and [AWS Prescriptive Guidance on separate ontology/reasoning services](https://docs.aws.amazon.com/prescriptive-guidance/latest/semantic-layer-agentic-ai-ontology-reasoning-virtual-knowledge-graph/technology-tradeoffs-alternatives.html).
- W3C: [RDF Schema 1.1](https://www.w3.org/TR/rdf-schema/), [PROV-O](https://www.w3.org/TR/prov-o/), [OWL 2 RL profile](https://www.w3.org/TR/owl2-profiles/#OWL_2_RL), [SHACL](https://www.w3.org/TR/shacl/).
- [SMT-LIB language specification](https://smt-lib.org/language.shtml) for a potential later constraint-solving interface.

---

## 29. Terminology decision and compatibility

**Adopted in v0.3:** Use **note** as the common content noun. Use **source note**,
**authored note**, **derived/shared note**, **insight**, or **task note** when the
distinction matters. “Memory” describes the historical concept in v0.2, not another
customer-visible object. A board is an optional organization of notes. Folders,
tags and subscriptions should appear only when a project uses that capability.

Keep Projects, Blueprints, Night Shift, Briefing and Impact. Source/evidence,
uncertainty, time, partial cleanup and unsupported delivery capabilities must remain
clear. A note is not inherently true, a task is not inherently done, and a delivered
notification is not inherently read or acted on. Use “Where this came from” for
provenance and “Undo this contribution” for retraction, without hiding partial results.

The revision contract at the beginning of this document governs API aliases and
storage migration. Do not mass-rename immutable IDs, historical audit events or
`MEM-*` requirements. New user-facing contracts use notes consistently. A future
implementation migration must qualify old clients and idempotent retries before
removing legacy names. Naming is adopted here; implementation migration is separate.

## 30. Optional project board and live session subscriptions

### 30.1 Purpose, scope, and optionality

**Agreed in v0.3:** A project may have an optional board for agents to organize and
collaborate through notes. Notes may be extracted, explicitly authored, or derived.
They support hierarchical organization, attributes governed by a blueprint's
ontology or flexible custom attributes, tags, valid time, and dependencies. Agents
can read, list, create, and update notes. Active sessions can subscribe to selected
notes, tags, folder prefixes, or combinations, with changes delivered through a
harness steering hook where supported. Night Shift can refine notes and identify
stale information or work.

**Proposed defaults below** make that contract testable. They are product design,
not claims that the current service, Codex, Claude Code, or NeMo implements them.

- Board disabled by default. Creating a project, capturing extracted notes, search,
  provenance, correction, and Night Shift require no folders or board setup.
- One optional board per project initially. Enabling it introduces organization
  and coordination tools; it does not create a second copy of project knowledge.
- An existing extracted note can be placed on the board by its stable note ID.
  Removing its placement leaves the note intact. Explicitly created board notes
  enter the same authorized retrieval, privacy, versioning, and evidence system.
- Disabling the board suspends board-specific writes and subscriptions, retains
  placements and note content, and leaves ordinary note retrieval available.
  Re-enabling restores organization; it does not silently restore old session
  subscriptions. Archive, placement removal, retraction, and purge are distinct.
- A board is an agent-native organizational interface, not a required web app,
  Kanban workflow, task runner, or replacement for the software factory's Git tasks.
  It can represent milestones, work, discoveries, datasets, decisions, or any other
  project-specific concept. Projects need not adopt task statuses or ownership.

### 30.2 Unified note identity and versioned organization

Every note has a project-scoped stable ID and immutable content version IDs. The
note records origin (`extracted`, `authored`, or `derived`), authorship, source/session
attribution, time, publication/retraction status, and derivation inputs where present.
An authored note is attributable to the author/session; it need not pretend to have
been extracted from a transcript or empirically proven. Derived assertions require
exact input versions and retain the existing provenance guarantees.

A note's board placement is optional: `folderPath = null` means unplaced; `/` means
placed at the root. Proposed initial rule: one canonical folder per note, with tags
and saved queries providing multiple views. The board owns no separate text copy.
Notes with identical text or titles remain distinct unless an explicit, audited
consolidation preserves independent sources.

Content edits, attributes, tags, placement changes, dependency edges, and workflow
state are versioned/auditable. APIs expose a revision covering the complete visible
note state, plus immutable content-version references for derivations. Each accepted
mutation increments the revision and emits a durable event in the same transaction.
Clients send `expectedRevision` and a request idempotency key; stale writes fail
with a conflict and the current authorized revision, never silently overwrite a
teammate's edit. Edits and graph traversals remain subject to project membership and
author-controlled/collaborative policy. Moving a note does not change its ID, owner,
source attribution, publication state, or derivation inputs.

### 30.3 Folders, tags, and listing

Proposed canonical folder paths are absolute, case-sensitive, Unicode-normalized
segment paths; `/` is the root. Reject `.`/`..` segments, backslash aliases, duplicate
separators, and encoded separator ambiguity. These are logical project paths, not
filesystem paths or an agent's local workspace. Titles need not be unique.

Prefix matching is segment-aware: `/pipelines/orders` matches that folder and its
descendants, not `/pipelines/orders-old`. A folder move uses optimistic concurrency,
rejects moving a folder beneath itself, and updates affected placements atomically
or through a visible resumable job with a documented consistency boundary. It must
not report completion while listings disagree about the committed location.
Empty-folder creation may be supported; it must not be required to place a note.

Tags are normalized project-local labels. Proposed initial behavior is Unicode
normalization plus case folding, with the display spelling preserved. Exact note
IDs, tags, and folder filters are tenant/project scoped. Listing supports:

- Folder prefix, with recursive or direct-children behavior made explicit.
- Tag filters with `all` or `any` matching; absent tag filters impose no restriction.
- Optional note kind, status, and valid-time/as-of filters.
- Stable pagination and a snapshot/event cursor, including protection against
  duplicates or silent omissions during concurrent moves and edits.

The default note search includes authorized board notes and unplaced notes. Board
listing selects placed notes; ordinary note listing may explicitly include unplaced
notes. Board-disabled projects retain ordinary note listing and search. Counts,
facets, tag discovery, and dependency expansions must not leak unauthorized notes.

### 30.4 Blueprint ontology and flexible attributes

The blueprint may define note kinds, typed attributes, allowed enum values,
required fields, units, reference types, dependency types, and allowed workflow
transitions. These are schema/validation rules for semi-structured notes; **no
OWL, Datalog, SMT, formal reasoner, or dedicated graph engine is required**.

Example Data Engineering blueprint: `milestone`, `task`, `dataset`, `insight`, and
`decision`; a task may have `status`, `assignee`, `priority`, `datasetId`, and
`acceptanceCriteria`. `status` might be one of `pending`, `ready`, `in_progress`,
`blocked`, `done`, or `needs_recheck`. This is an example, not a universal workflow.

Projects may instead use free-form notes, or allow a typed schema with a custom
attribute namespace. Proposed representation separates validated `attributes`
from flexible `customAttributes` JSON values. Custom fields cannot shadow reserved
identity, authorization, provenance, revision, policy, or lifecycle fields. Enforce
JSON size/depth limits and supported value types. A reference-like string is not
an authorization grant or a dependency unless created through the corresponding
validated relation operation. Every update, including agent and Night Shift writes,
passes the same server-side schema and policy checks. Privacy evaluation covers
text, titles, attributes, tags, folder paths and notification payloads; storing a
secret in metadata cannot bypass a text-content exclusion.

Notes pin their blueprint/schema version. Blueprint upgrades do not rewrite old
notes or discard unknown attributes. Provide preview/validation, explicit migration,
conflict reporting, and an auditable result. A note valid under its pinned version
remains readable when the project adopts a new version; failed migrations retain
the previous version. A stricter blueprint cannot weaken personal privacy rules.

### 30.5 Valid time and dependency meanings

Board notes use the same observed, recorded, valid, confirmed, and superseded time
model as all other notes. Valid time is optional and may be uncertain; absence is
not an inferred timestamp. Proposed known intervals are `[validFrom, validTo)` in
UTC, with explicit open bounds and unknown precision. A task deadline (`dueAt`) is
an attribute, not the note's valid time. An expired observation is not proof that
a task finished, and modifying a note does not reset the fact's valid time.

Keep these relationship families separate:

| Relation | Meaning | Effect of parent change/retraction |
|---|---|---|
| `derived_from` exact version | Evidence/content depends on that input | Existing provenance, resynthesis, rollback and purge rules apply. |
| `depends_on` stable note | Work item requires another work item or milestone | Recompute declared readiness; flag affected work for review. Does not by itself retract factual content. |
| `related_to` stable note | Navigational/semantic association | Neither readiness nor provenance propagation is implied. |

A board workflow may additionally define typed relations, but each type must declare
whether it is a readiness constraint, semantic association, or actual derivation.
Default `depends_on` relations form a DAG; reject self-dependencies and cycles under
concurrent writes. `related_to` may be cyclic. Derivation remains acyclic over exact
versions under the existing note publication contract. Cross-project edges are
out of the initial board scope; reject them even when the caller belongs to both.

Dependencies are project state, not a distributed lock. In a task blueprint, an
atomic revision-checked claim/assignment transition can prevent two agents from
both winning the same task. Optional leases must expose expiration and a fencing
generation; a stale worker cannot complete reassigned work. Generic boards need
not implement automatic scheduling. Missing, archived, expired, or retracted
prerequisites are surfaced explicitly and never silently treated as satisfied.

### 30.6 Session-scoped subscriptions and filter semantics

An authorized active agent session can create, list, update, and cancel its own
subscriptions. Each subscription is bound to tenant, project, principal, immutable
session identity, and a filter revision. It is not an account-wide permanent
notification channel or a way to see another session's private transcript.

Proposed filter contract:

```json
{
  "projectId": "proj_orders",
  "sessionId": "session_transformer",
  "selector": {
    "noteIds": [],
    "folderPrefix": "/pipelines/orders",
    "tags": {"match": "all", "values": ["milestone:daily-revenue", "schema"]}
  },
  "includeOwnChanges": false,
  "afterCursor": "project-event-184"
}
```

Nonempty selector dimensions are combined with **AND**. Within `noteIds`, match
any listed ID; within tags use the explicit `all`/`any` mode. Thus this example
matches notes below `/pipelines/orders` carrying both tags. For OR across different
selectors, create multiple subscriptions; duplicate events are coalesced for the
session with the matching subscription IDs retained. An empty selector is rejected
unless the caller explicitly requests `allProjectNotes: true`. Proposed default:
suppress the session's own writes while still advancing its cursor; callers may
opt in to them. A subscription never implies work ownership or a lease.

The subscription covers content, selected metadata, tags, placement, valid-time,
status, dependency, correction, archive, retraction, and privacy-remediation changes.
Match edits against both before and after selector state: entering the selection,
changing while selected, and leaving it all produce meaningful notifications.
A move out of a prefix or removal of a watched tag must not disappear silently.
Note-ID subscriptions follow the note across moves. Delete/purge emits an authorized
minimal tombstone, not deleted text. A blueprint may also request an explicit
`validity_boundary_reached` event when time alone changes relevance; this requires
a durable timer and is not implied by ordinary write notifications.

### 30.7 Delivery, replay, and harness steering

Persist note mutations and change events atomically. Proposed delivery is
**at least once**, ordered by a project event cursor, with per-note revisions and
stable event IDs. Event ordering describes recorded changes, not domain valid-time
ordering. Exactly-once model attention or response is not promised.

Registration and initial listing must share a consistent cursor boundary: the
client receives a snapshot at cursor C and replays events after C, with no gap.
Reconnection resumes from the last acknowledged cursor within retention. A cursor
outside retention returns `resync_required`; the harness refreshes an authorized
snapshot and establishes a new cursor rather than silently dropping changes.
Subscription filter updates establish a new snapshot/cursor boundary too. Use
bounded queues, rate limits, and coalesced summaries that retain the affected IDs,
revision range, and refresh requirement; queue overflow must not become a false
claim that every intermediate edit was delivered.

The harness receives an authenticated compact envelope with event ID, project and
note IDs, revision, safe change type, subscription IDs, and an authorized refetch
link/tool argument. It injects a brief **untrusted change notification** via the
host's supported steering hook at a safe boundary. The agent then reads current
notes through ordinary authorized tools before acting. Note text and custom
attributes are never system/developer instructions; a notification cannot grant
permissions, spawn work, run commands, or approve a transition.

Host adapters must report which capabilities are actually supported:

| Host capability | Delivery behavior |
|---|---|
| Safe in-session steering/injection hook | Inject a deduplicated change envelope at the supported boundary. |
| Tool/turn-boundary delivery only | Deliver pending changes on the next supported tool/turn boundary and expose lag. |
| No injection hook | Offer explicit `changes.poll` and a visible capability notice; do not claim background interruption. |

Transport receipt, harness injection acknowledgment, and observed agent handling
are separate states. A socket write is not evidence that the model saw or acted
on a note. An agent may acknowledge handling explicitly, but its assertion is not
proof that its work is correct. Retries of injection deduplicate by event ID and
session, acknowledging only after the declared stage succeeds.

Subscriptions expire on session end/lease expiry, explicit cancellation, membership
revocation, project archival, or board disablement. Reconnection to the same live
session can resume within its lease. A new session must explicitly subscribe;
it cannot impersonate or inherit the old session's cursor/authority. Switching
active project does not silently retarget an existing subscription. Surface its
original project binding and require a subscription in the newly bound session.

Reauthorize at subscription registration, event fetch, queue dispatch, and note
refetch. Revocation cancels queued delivery before disclosure. Do not emit counts,
titles, old tags, old paths, or old content to someone who can no longer read them.
Privacy purge also invalidates retained notification bodies/caches; retain only
safe event metadata under the deletion policy. Private transcripts never become
notification payloads merely because a shared note cites a session.

### 30.8 Night Shift and operational collaboration

Night Shift can propose or perform policy-authorized refinements, consolidate
redundant notes while preserving identities/evidence, detect abandoned work or
stale assumptions, find contradictory status/evidence, and flag outdated valid
intervals or missing prerequisites. Every mutation emits the same events and uses
the same revision checks as a foreground agent.

Task completion, milestone acceptance, ownership changes, and moving another
agent's work are not inferred authorizations. Default behavior is to propose such
changes or mark `needs_recheck` under explicit policy, not silently mark work done.
Night Shift cannot renew a worker lease or invent a test result. Corrections and
retractions of evidence propagate through actual derivation links; operationally
related work receives a warning/readiness update rather than arbitrary deletion.

An extracted data insight may be linked to an explicit milestone note. The
milestone's completion assertion should reference real validation artifacts. Only
fields actually derived from a retracted source require content reconstruction;
independently authored work history and independent evidence remain attributable.
A workflow needing that distinction must preserve field-level derivation or split
the independently supported assertions into separate notes. Do not preserve a
whole contaminated composite note merely by labeling it a task.

### 30.9 Proposed tool surface and records

| Tool family | Operations and contract |
|---|---|
| `context.board` | `enable`, `disable`, `status`; owner/config authority and visible lifecycle. |
| `context.notes` | `create`, `get`, `list`, `update`, `correct`, `archive`; stable IDs, typed/custom attributes, tags, time, revisions and source lineage. |
| `context.board.folders` | `list`, `move`; canonical prefix semantics, revision conflicts and large-move job status. |
| `context.notes.dependencies` | `add`, `remove`, `list`; explicit relation family, scoped endpoints and cycle validation. |
| `context.notes.claim` | Optional workflow operation; atomic expected-revision transition, explicit lease/fence if configured. |
| `context.subscriptions` | `create`, `list`, `update`, `cancel`; current session ownership and explicit selector semantics. |
| `context.changes` | `poll`, `ack`; cursor replay, safe envelopes, stage-specific acknowledgment and resync. |

Existing note inspection/search/rollback APIs remain the authority for shared
content. A board write must not bypass those policies. Suggested durable records
are BoardConfiguration, NoteRevision, BoardPlacement, TypedDependency,
SessionSubscription, ProjectChangeEvent, and DeliveryCheckpoint. These fit the
Rust service/PostgreSQL authority and transactional outbox; OpenSearch remains a
rebuildable retrieval projection. No graph database, external broker, separate
microservice, or NeMo dependency is required for the initial local board.

### 30.10 Implementation scope and unresolved choices

The board, attributes, valid time, dependencies, and active-session subscriptions
are part of the non-formal-reasoning prototype scope. Stage core CRUD/listing,
revision conflicts, and replay APIs before host-specific injection. Include a
board-enabled data-project episode in the gym alongside the existing retry-policy
scenario. Also retain a board-disabled regression path throughout development.

Open sizing/product choices: folder depth and payload limits; subscription/session
limits and replay retention; p95 delivery targets per host capability; default
lease duration for task blueprints; supported custom attribute indexing; and which
schema migrations Night Shift may propose automatically. Measure these in the gym.
They do not justify omitting replay, authorization, stale-write protection, or
capability disclosure. Beads-like task/dependency ergonomics are inspiration only:
this spec does not adopt Beads' storage, protocol, compatibility, or execution model.
