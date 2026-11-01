# AWS Context Projects — Product and Technical Specification

**Status:** Working draft, v0.2 — terminology review and future formal reasoning added  
**Date:** October 31, 2026  
**Working feature name:** Context Projects  
**Internal codename:** Project Spindle (not a customer-facing product name)  
**Product context:** An existing agent context-management service referred to in the design discussion as *AWS Context*  
**Audience:** Product, design, engineering, security, and developer-experience teams  
**Revision note:** v0.2 retains the original v0.1 product decisions and adds an optional structured-claim/reasoning architecture, technical caveats for Neptune Analytics, and a nonbinding terminology exploration.  
**Source:** The product-design conversation from which this document was requested. This is a proposed feature specification, **not** a statement of released AWS functionality or official AWS API contracts.

> **Product promise:** Start a project in your coding agent, keep working normally, and let useful team knowledge accumulate—with clear visibility, personal privacy controls, evidence, and the ability to correct or retract what was learned.

**Quick navigation:** [Customer storyboard](#6-golden-path-customer-experience-a-storyboard) · [User stories](#7-user-stories) · [Functional requirements](#8-functional-requirements) · [Architecture](#11-architectural-model) · [Rollback and provenance](#14-provenance-correction-and-rollback) · [Briefing and Impact](#15-feedback-memory-effectiveness-and-demonstrating-value) · [Night Shift](#16-night-shift--daily-project-intelligence-maintenance) · [Blueprints](#17-built-in-project-blueprints) · [Agent interface](#18-agent-native-ux-command-grammar-and-tool-discoverability) · [Open questions](#23-open-design-questions-and-brainstorming-backlog) · [Acceptance tests](#24-acceptance-scenarios-and-system-tests) · [Prototype handoff](#25-prototype-handoff-what-the-later-clickable-html-mock-should-demonstrate) · [Formal claims and reasoning — future phase](#28-future-phase-structured-claims-and-formal-reasoning) · [Terminology and naming options](#29-terminology-review-and-naming-options)

---

## 1. Problem and opportunity

Developers and other knowledge workers make consequential decisions while working with agents, but the reasoning, constraints, discoveries, and corrections from those sessions are scattered across ephemeral conversations and individuals' context. A shared context graph can retrieve information, but it does not by itself define which observations matter to a particular collaborative effort, whose knowledge should be combined, how changes over time should be handled, or how an individual can inspect and retract something contributed accidentally. **Context Projects** adds a small, agent-native collaboration boundary to an existing Context graph: a project has a goal, members, and optional steering instructions; associated agent sessions contribute traceable project memories; retrieval blends those memories with the user's broader accessible graph; and maintenance, feedback, and correction continuously improve the shared knowledge. The experience should be almost effortless to begin and progressively reveal its advanced controls only when needed.

### 1.1 One-minute product description

A Context customer with an existing graph opens Claude Code (or later Codex or another supported agent), asks to create or join a project, and optionally selects a blueprint such as **Software Development**. Context activates that project for subsequent sessions, visibly confirms that activation, and ingests session trajectories through plugin hooks. It extracts relevant decisions, facts, constraints, lessons, and inferred insights into a shared project memory layer. Teammates get more useful answers and briefings because their agent can retrieve project knowledge alongside the existing graph. Users can inspect their contributions, correct mistakes, or roll back an entire session and its derived effects. A daily **Night Shift** maintains and improves the project knowledge. A **Briefing** explains what changed, and **Impact** shows when contributions actually helped or led others astray.

### 1.2 Existing system and new boundary

The following capabilities are **existing architecture as described in the discussion**, not deliverables to rebuild:

- An agent context-management service with a context graph, hybrid search, feedback, contradiction detection, linking, and extraction of reusable knowledge and skills.
- **Amazon Neptune Analytics** for graph nodes and traversable relationships.
- **Amazon OpenSearch Serverless** for hybrid textual/semantic retrieval.
- A search flow that retrieves OpenSearch documents first and then follows their referenced node IDs to perform Neptune Analytics traversals when needed.
- Agent integrations/plugins able to make Context tools available in coding-agent sessions.

Context Projects introduces project identity, membership, per-project extraction and retrieval behavior, traceable source/derived memories, retrospective controls, nightly maintenance, and agent-native developer workflows. It remains **one logical context graph**, not a disconnected knowledge silo.

---

## 2. Product tenets and design decisions

### 2.1 Non-negotiable experience principles

1. **Simple from the first minute.** A user should need to understand only *project*, *join/activate*, and *shared memories* to see value. Everything else is optional or discovered at the moment it is useful.
2. **Agent-native, conversational, and self-discovering.** The full workflow should be possible from Claude Code through natural language, tools, and a small number of memorable slash commands. A new web console is not required.
3. **Visible participation, no surprises.** A project remains active for future sessions after activation, but users must have an obvious indication of the currently active project and its contribution behavior.
4. **Value must be observable.** Show what changed, which memories were used, which teammates benefited, and where knowledge caused confusion or was corrected. Avoid claiming impact when only retrieval exposure was measured.
5. **Personal privacy takes priority.** Global personal exclusions apply to *every* project and cannot be relaxed by project or blueprint instructions. Projects may impose additional restrictions.
6. **Shared knowledge is editable and attributable.** Members can see project memories; editing rules are configurable. Contributors can audit their own sources and retract accidental contributions.
7. **Evidence survives synthesis.** Keep original extracted source observations separate from canonical/synthesized memories, with provenance sufficient to remove an input and re-synthesize its descendants.
8. **Temporal truth matters.** A newer observation need not invalidate an older one if their valid time, environment, or scope differs.
9. **Maintenance improves rather than invents authority.** Night Shift can infer new insights, but every inferred insight remains explicitly labeled and grounded in evidence.
10. **Advanced machinery stays in the background.** Blueprints, defaults, and progressive disclosure should hide the complexity of policies, ontologies, graph traversal, and rollback plans.
11. **Reasoning is additive, not a new burden.** Any later formal-claim layer supplements natural-language memory; inference must remain qualified, evidence-backed, reversible, and invisible by default.

### 2.2 Recorded decisions versus proposals

This document uses three labels:

- **Agreed:** Directly established in the conversation.
- **Proposed:** A concrete design recommendation to make the agreed behavior implementable.
- **Open:** A decision that requires validation, prioritization, or further brainstorming.

**Agreed:** Each project is associated with an existing Context graph instance. A project memory belongs to **one project only** for now. Members of a project can read its project memories, including those contributed by other members. Membership and collaboration controls are deliberately simple in the first release. A user activates a project once, rather than opting in for every session; later sessions remain associated by default until activation is changed. Original extracted contributions must be retained as individually traceable evidence for as long as they are valid inputs. Shared or derived memories must preserve their input lineage. The plugin must clearly show the active project. Rollback has full, bounded, and bounded-plus-best-effort-semantic variants. Pending rollback does not make all affected memories unsearchable; progress and incomplete-removal risks are visible to the initiator. Project and personal steering, Night Shift, and built-in blueprints are part of the desired feature.

**Proposed:** Store the active-project selection per user and agent workspace, with one selected project at a time for that workspace, so unrelated repositories are less likely to be captured accidentally. Preserve it across restarts and show it at every session start, plus in a status surface if the host supports one. The exact activation scope remains an open implementation choice; whatever scope is selected must meet the agreed persistent-and-visible behavior.

---

## 3. Goals, success criteria, and non-goals

### Goals

- Capture and refine useful *project-specific* knowledge from agent work with essentially no extra manual documentation burden.
- Give project members relevant answers that combine current project memory with permitted context from the existing graph.
- Preserve separate source contributions when multiple people reach overlapping or partially conflicting conclusions; synthesize the best nuanced shared memory without erasing evidence.
- Make project memory temporally aware, correctable, auditable, and reversible from a single session through arbitrary derivation depth.
- Demonstrate concrete individual and team value through briefing, impact, and feedback.
- Support different project types out of the box and allow optional ongoing steering without building a configuration-heavy product.

### Experience success criteria (targets to validate, not promised metrics)

- A user with an installed Context plugin can create and activate a project in a short conversational exchange; role and advanced settings can all be skipped.
- The active project is obvious on entry to every associated agent session and immediately discoverable later.
- The first meaningful session results in reviewable extracted memories without requiring a separate application.
- A teammate can retrieve a relevant project decision, its temporal qualifier, and its supporting evidence while working in a different session.
- An author can preview the impact of rolling back an accidentally associated session and follow the resulting job to completion.
- Briefing and Impact distinguish *memory produced* from *memory retrieved*, *memory actually used*, and *memory later found misleading*.

### Non-goals for the initial scope

- Replacing Neptune Analytics, OpenSearch Serverless, or the existing graph/retrieval architecture.
- Automatically sharing the same memory into multiple projects; one project owns one memory.
- Enterprise-wide cross-project governance, elaborate classification/approval hierarchies, or fine-grained inherited source-ACL enforcement on each shared project memory.
- Requiring users to manage sessions, extraction rules, or provenance graphs manually to get started.
- Presenting unverified inferred insights as established facts.
- Building a standalone web UI as the primary control surface. A later interactive HTML prototype is a design artifact, not a prerequisite for the product.

---

## 4. Core concepts and mental model

| Concept | What the user needs to know | Behind the scenes |
|---|---|---|
| **Context graph instance** | Their existing knowledge base | OpenSearch + Neptune Analytics, treated as one logical graph |
| **Project** | A shared space organized around a goal | Stable ID, graph-instance association, membership, policies, blueprint snapshot |
| **Active project** | The project their agent work is contributing to | Persistent user/agent activation binding; snapshotted on each session |
| **Project member** | Someone who can benefit from and contribute to shared memories | Membership, role, and collaboration mode |
| **Blueprint** | A sensible starting point, e.g. Software or Research | Default ontology, extraction, curation, insights, Night Shift, and retrieval settings |
| **Original memory** | Something Context learned directly from a source session | Immutable/versioned extracted evidence with source/session identity |
| **Shared/canonical memory** | The reconciled project understanding | Versioned synthesis supported by one or more originals or derived inputs |
| **Briefing** | What changed since I last worked here? | Personalized summary of new decisions, corrections, unresolved conflicts, and Night Shift |
| **Impact** | Where did project knowledge help or mislead? | Evidence-based attribution using retrieval, usage, feedback, and later contradictions |
| **Night Shift** | Daily knowledge maintenance and insight discovery | Scheduled, auditable, project-steered maintenance job |
| **Rollback** | Undo information learned from a session or memory | Impact preview, provenance traversal, targeted retirement/re-synthesis, progress tracking |

A typical user does **not** need to distinguish original from canonical memory or understand provenance DAGs until inspecting or correcting information. Those distinctions are essential in the system, but should not dominate the product language.

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

### Scene 0 — Already a Context customer

**Avery** has a Context graph connected to their engineering environment. The Context plugin is already installed in Claude Code; its search and graph tools work. Their team is modernizing a billing API. Today, the decisions are scattered across sessions, reviews, and engineers' memories.

**Customer sees:** Their normal Claude Code session. No new account or graph setup.

**Underlying behavior:** The existing Context graph is a parent resource in which a project can be created.

### Scene 1 — Create the project with one request

Avery says:

```text
Create a Context project called "Billing API Modernization" using the
Software Development blueprint. Our goal is to move billing requests to
the new gateway without introducing duplicate charges. Remember key
architecture decisions, constraints, API contracts, and unresolved risks.
```

Claude Code uses Context tools to create the project, associates it with Avery's existing graph instance, and activates it.

```text
✓ Created: Billing API Modernization
  Blueprint: Software Development
  Context: Your existing engineering graph
  Status: ACTIVE for future Claude Code sessions in this workspace

I'll capture relevant project knowledge from your sessions. Extracted
project memories will be visible to members you invite. Personal privacy
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
New sessions contribute project memories. /context status | /context leave
```

Ideally this appears in a host-supported status line or persistent agent UI. If the host cannot support that, the plugin must present an explicit session-start announcement and make the current project quickly retrievable. A transient announcement alone should be tested for sufficient visibility.

**Important:** Avery does not activate the project separately for each session. An explicit switch or leave command changes the future default. The session records which project was active *when that session started* so later switches cannot silently reassign old work.

### Scene 4 — Work normally; an architectural decision becomes memory

Avery and Claude Code review gateway retry behavior. They decide:

```text
Until the new gateway issues durable idempotency keys, do not retry
non-idempotent charge-creation requests automatically. Read-only calls
can use bounded exponential-backoff retries.
```

Avery continues working; there is no special save gesture. When the session ends, the plugin's session-end hook submits the trajectory or a configured, filtered representation. Context applies global personal exclusions, project policy, and Avery's contribution focus. It extracts original observations including the decision, rationale, scope, and time, then creates or updates an associated shared memory. It links the memory to existing service entities when those references exist in the graph.

Later, `/context memories mine --recent` might show:

```text
From your last session: 3 proposed memories, 2 accepted, 1 merged.
• Decision: No automatic retries for non-idempotent charge creation
  until durable idempotency keys are available. [source session S-104]
• Constraint: Read-only gateway requests may use bounded backoff. [S-104]
• Existing shared memory updated: Gateway reliability policy. [S-104]

Review or correct any item: /context memories mine
```

The actual CLI need not expose every stage by default; this is a discoverable audit view.

### Scene 5 — A teammate gets the answer at the right moment

Avery invites **Sam** and **Jordan** to the project. Sam joins from Claude Code with an optional contribution focus such as “gateway implementation and integration testing.” Project memories are shared with members even if their original source authors are different.

A few days later Sam asks Claude Code:

```text
Can we automatically retry failed charge-creation requests?
```

Context searches the active project's memories **and** the broader graph Sam is already allowed to access. It finds the project decision, prioritizes its relevance, and traverses graph links to related gateway components where available. Claude Code replies in substance:

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

Context records both original observations, checks their scope and valid times, and synthesizes a qualified project memory:

```text
Charge-creation retries remain disabled for legacy endpoints.
For gateway v2 create-charge requests with confirmed idempotency-key
support, bounded retries may be enabled after rollout verification.
```

If rollout status or evidence is insufficient, the combined memory explicitly says **unresolved / needs review** rather than manufacturing certainty. Both contributors' source evidence remains traceable.

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
- Two memories await review because new evidence partly conflicts.

Every claim can be opened to its project memory and evidence. The briefing is not merely a list of documents; it is an actionable orientation to the project's evolving state.

### Scene 8 — See value: `/context impact`

Avery asks:

```text
/context impact
```

An example response:

```text
Your contribution impact — last 30 days
• 12 original memories accepted; 7 informed shared canonical memories
• 19 teammate retrievals exposed those memories
• 6 cases include a later explicit citation or positive feedback
• 2 decisions were updated as new evidence arrived
• 1 retrieved memory was later flagged misleading and corrected

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

Context shows the session and resulting original/shared memories. Before changing anything, it presents a **rollback preview**:

```text
Session S-108: 4 original contributions
Directly supported: 3 shared memories
Transitive dependencies: 27 derived memories across 8 levels
Other independent sources support: 2 of the affected shared memories

Choose:
1. Full recursive rollback (complete active-memory cleanup; slower)
2. Bounded rollback (choose depth; deeper influence may remain)
3. Bounded + semantic leakage scan (best effort beyond depth)
```

Avery can inspect the implications and start the job. The system retracts S-108's original inputs, re-synthesizes memories with their **remaining clean evidence**, and retires unsupported ones. Avery watches progress and any failures; other members' unrelated evidence is not discarded. During processing, memories remain generally searchable, as agreed, but Avery sees that the rollback is **not yet complete** and that sensitive information may remain until processing finishes.

### Scene 10 — Night Shift maintains the shared understanding

Overnight, **Night Shift** reviews the project's goals, new memories, stale facts, temporal changes, contradictions, and retrieval feedback. It may consolidate overlapping memories, deprecate stale ones, correct well-supported errors when policy permits, and **propose or publish novel inferred insights** with explicit evidence and confidence.

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

## 7. User stories

| ID | Persona / user story | Acceptance intent |
|---|---|---|
| US-01 | As an existing Context customer, I can create a project in Claude Code without learning a new UI. | One conversational flow creates a project linked to an existing graph; advanced settings are optional. |
| US-02 | As a creator, I can select a built-in blueprint and provide a goal. | Useful extraction defaults exist before manual customization. |
| US-03 | As a contributor, I can join and stay active across future sessions. | No per-session opt-in; clear persistent status and explicit leave/switch. |
| US-04 | As a contributor, I can optionally describe my role and expected knowledge contribution. | My extraction is better targeted without affecting another member's focus. |
| US-05 | As a privacy-conscious person, I can define global never-share rules. | Rules apply in all projects and have higher priority than project defaults. |
| US-06 | As a member, I can read project memories contributed by teammates. | The project sharing boundary is clear, simple, and visible at join time. |
| US-07 | As an engineer, I get project-relevant knowledge during normal graph search. | Blended retrieval boosts relevant project context without ignoring the wider graph. |
| US-08 | As a researcher or analyst, I can search only this project's knowledge when desired. | An explicit project-only search mode is available. |
| US-09 | As a contributor, I can see which sessions and memories I contributed. | Each original memory has a source session and accountable author. |
| US-10 | As a member, I can flag misleading knowledge. | Flag creates feedback and can trigger correction/review. |
| US-11 | As a contributor, I can correct or retract my own information. | Shared syntheses are updated and source evidence is not silently overwritten. |
| US-12 | As a project owner, I can choose author-controlled or collaborative corrections. | Permissions are enforced and all changes auditable. |
| US-13 | As a contributor, I can retract an entire accidentally included session. | Preview shows transitive blast radius; all supported rollback modes are offered. |
| US-14 | As a contributor, I can choose how exhaustive rollback should be. | Full, bounded, and bounded + semantic scan state costs and guarantees accurately. |
| US-15 | As an auditor, I can follow source/derived memory lineage. | Original evidence and transformations remain available where authorized. |
| US-16 | As a team member, I can ask for a project briefing. | Includes relevant changes since my last interaction and new Night Shift insights. |
| US-17 | As a contributor, I can see how my memories affected my team. | Impact separates exposure, demonstrated usefulness, corrections, and uncertainty. |
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
| PRJ-04 | P0 | Invite and join teammates using simple project membership. | Members can search all shared project memories as agreed. |
| PRJ-05 | P0 | Support persistent project activation in the plugin. | Subsequent sessions attach automatically until explicit switch or leave. |
| PRJ-06 | P0 | Make the active project and capture status unmistakable. | Users can identify the project at session start and via an anytime command. |
| PRJ-07 | P0 | Snapshot project binding at session start. | Changing active project later does not rewrite an earlier session's provenance. |
| PRJ-08 | P1 | Provide an easy path to enumerate prior associated sessions. | Contributor can find an incorrect session after it ends. |
| PRJ-09 | P1 | Support project archival without erasing required retained provenance. | Archived projects stop accepting new sessions by default; retrieval/retention behavior is documented. |

### 8.2 Ingestion, extraction, synthesis, and time

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| MEM-01 | P0 | Ingest agent session traces via hooks/integration. | Session-end ingestion is idempotent; failure/retry paths exist. |
| MEM-02 | P0 | Apply personal privacy exclusions before storing or sharing extracted content. | Test cases for secrets, PHI/PII-like content, and custom patterns do not become shared memories. |
| MEM-03 | P0 | Extract typed original memories suited to project goals and blueprint ontology. | Decisions, facts, constraints, risks, discoveries, and skills can be distinguished. |
| MEM-04 | P0 | Preserve an individually attributable original for each source contribution. | Two agreeing contributors remain distinguishable even after a merge. |
| MEM-05 | P0 | Synthesize canonical memories from one or multiple originals. | Unified text preserves qualifiers, exceptions, and meaningful disagreement. |
| MEM-06 | P0 | Keep provenance for every derivation, including recursively derived memories. | Reverse dependency traversal finds every recorded descendant. |
| MEM-07 | P0 | Track effective time, observed time, and modification time separately. | Search and contradiction handling can distinguish changed reality from incompatible claims. |
| MEM-08 | P0 | Support existing link and contradiction detection pipelines. | Project memory can link to wider graph entities without losing project ownership. |
| MEM-09 | P1 | Use feedback to update or deprecate low-quality memories. | Later contradictory evidence can trigger review or configured correction. |
| MEM-10 | P1 | Preserve source/derived history while permitting content purging for retractions. | Clean evidence remains; deleted or excluded content does not survive as readable audit text. |

### 8.3 Search, sharing, and permissions

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| RET-01 | P0 | Search project memories together with the caller's permitted wider graph by default. | Project relevance is boosted but not treated as absolute authority. |
| RET-02 | P0 | Accept optional project context on retrieval. | Explicit modes support blended, project-only, and non-project search. |
| RET-03 | P0 | Enforce project membership and graph access at server boundaries. | Cross-project/unauthorized queries do not return protected material. |
| RET-04 | P0 | Return provenance, freshness, temporal scope, and confidence where meaningful. | Agent can qualify answers and provide evidence links. |
| RET-05 | P0 | Store an unlinked memory in OpenSearch without forcing a Neptune node. | Retrieval handles memories with or without Neptune node IDs. |
| RET-06 | P1 | Materialize graph nodes/edges when entity linkage/traversal adds value. | Linked project memories participate in existing graph exploration. |
| ACL-01 | P0 | Project members can see all project memories under the agreed simple sharing model. | A teammate can read contributed shared memories without raw-trace access. |
| ACL-02 | P0 | Support **author-controlled** and **collaborative** correction modes. | Unauthorized edits are denied; all accepted edits are auditable. |
| ACL-03 | P0 | Limit original session trace access and source-session rollback to authorized actors. | Shared-memory access does not imply read access to another member's raw agent transcript. |
| ACL-04 | P1 | Make membership and data-sharing implications clear at join. | The user is told extracted project knowledge is shared with members. |

### 8.4 Visibility, privacy, and rollback

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| PRI-01 | P0 | Offer global personal never-extract/never-share steering with safe defaults. | Project/blueprint rules cannot relax global exclusions. |
| PRI-02 | P0 | Support personal contribution steering per project. | Optional role instructions influence quality but are not mandatory. |
| PRI-03 | P1 | Support additional project restrictions and regex/pattern rules. | Clear configuration, test/preview, and versioning are available in-agent. |
| PRI-04 | P1 | On tighter global exclusions, offer a scan of existing contributions. | Findings can feed the same rollback/remediation process. |
| AUD-01 | P0 | List original and canonical memories contributed by user/session. | Each displayed item includes identity, status, and evidence/source linkage. |
| AUD-02 | P0 | Support individual-memory correction, retraction, and review flags. | Corrected or removed information updates its derived dependents. |
| ROL-01 | P0 | Preview direct and transitive rollback blast radius before execution. | Counts, depth, independent support, and approximate cost are shown. |
| ROL-02 | P0 | Support full transitive rollback with resynthesis from allowed inputs. | On completion, active project outputs do not rely on retracted evidence. |
| ROL-03 | P1 | Support user-selected bounded-depth rollback. | Result explicitly states that deeper influence may remain. |
| ROL-04 | P1 | Add optional best-effort semantic leakage scan to bounded rollback. | Matches are searched beyond chosen depth and checked against provenance. |
| ROL-05 | P0 | Display job progress, partial failures, and completion state. | User can identify what is complete, pending, or beyond chosen scope. |
| ROL-06 | P0 | Keep affected memories generally searchable during processing. | Availability is preserved; initiator sees pending exposure caveat. |
| ROL-07 | P0 | Remove retracted source support without removing independent evidence. | Multi-author synthesis retains valid surviving nuances after rollback. |

### 8.5 Feedback, Night Shift, and blueprints

| ID | Priority | Requirement | Verification / acceptance criterion |
|---|---|---|---|
| VAL-01 | P1 | Implement `/context briefing`. | New and changed knowledge since last visit is personalized and evidenced. |
| VAL-02 | P1 | Implement `/context impact`. | Exposure, observed usage, favorable signals, and misleading/corrected cases remain distinct. |
| VAL-03 | P1 | Track memory retrieval events and subsequent session feedback. | A later contradiction can be associated with a previously retrieved memory. |
| VAL-04 | P1 | Feed quality signals back into extraction and correction. | Accepted/rejected/incorrect memories inform improvement without weakening privacy. |
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
| RSN-01 | Future | Optionally extract typed, atomic **claims** alongside readable memories, each with stable ID, source evidence, temporal/scope qualifiers, and policy versions. | A memory remains valid without any structured claims; structured claims point to the exact supporting memory/source versions. |
| RSN-02 | Future | Define a versioned, blueprint-extensible predicate/entity vocabulary with an RDF-aligned export/mapping. | The same assertion has a stable meaning across extraction, graph materialization, and inference. |
| RSN-03 | Future | Validate claims against explicit schemas/shapes and classify uncertain or contradictory extractions before publishing. | Bad entity types, missing scopes, sensitive content, and unsupported claims are not silently promoted into trusted facts. |
| RSN-04 | Future | Run controlled external inference and retain **each independent justification** for every inferred claim. | Withdrawing one premise removes only conclusions whose remaining valid justifications are insufficient. |
| RSN-05 | Future | Apply project ACLs, time/scope compatibility, and source retraction consistently across text, formal claims, graph links, and inference results. | Rollback and temporal tests pass with both text and formal representations enabled. |
| RSN-06 | Future | Keep structured reasoning invisible in the default agent UX. | Project creation, normal search, Briefing, and Night Shift remain useful without ontology or logic terminology. |

---

## 9. Permissions, sharing, and trust boundary

### 9.1 V1 sharing rule (agreed)

A project's members can access **all active shared project memories**, irrespective of which member contributed them or whether that member's broader source graph contains material inaccessible to the reader. **Contributing extracted knowledge to the project is an explicit sharing action within that project boundary.** This deliberately simple rule must be explained during project creation/join and during invitation.

It is important to distinguish three surfaces:

1. **Shared project memory:** Visible to every member, subject to project retention and correction policies.
2. **Original extracted memory / audit view:** The shared claim and its evidence metadata can be surfaced appropriately; detailed source content is subject to the source owner's access rights and deletion policy.
3. **Raw session trace and unrelated graph resources:** **Not** automatically readable by project members. Existing access controls still apply to those underlying resources and to non-project graph search results.

An extracted memory may summarize information originating from otherwise restricted data. Since the project explicitly shares that derived knowledge, this is a trust choice with a real disclosure risk; project membership must not be presented as magically inheriting fine-grained source ACLs. Stronger governance and propagation controls are potential later work.

### 9.2 Project roles (proposed minimum)

| Role | Create/invite/configure | Read all project memories | Change shared memories | Retract source sessions |
|---|---|---|---|---|
| **Owner/admin** | Yes | Yes | Yes | Own sessions; others only through explicit authorized administrative/remediation workflow |
| **Member, author-controlled mode** | No, except permitted invitation tasks | Yes | Own contributions and resulting corrections; flag others | Own sessions |
| **Member, collaborative mode** | No, except permitted invitation tasks | Yes | Any shared project memory, with audit trail | Own sessions, not others' raw source sessions |

**Agreed:** The correction collaboration model is configurable at project creation and changeable later. **Proposed default:** Author-controlled, with an easy choice of collaborative for a small close-knit team. Collaborative changes do **not** grant access to or permission to rewrite someone else's private trace. An authorized correction to a canonical memory creates a new version with its own evidence and authorship, not a silent rewrite of the original source observation.

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

**Default privacy posture (proposed):** Filter likely credentials/secrets, highly identifying personal information, protected-health-information-like content, and common customer-sensitive patterns, with configurable false-positive handling. Combine rule-based patterns (including optional regex) with classifiers and contextual analysis. Do not claim automatic PHI/PII detection is perfect. Apply filtering as close to the collection point as feasible, then validate server-side before a shared memory is published. Avoid echoing excluded values in diagnostics or logs.

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
errors automatically unless the memory represents a policy decision,
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

## 10. Memory semantics and temporal model

### 10.1 Memory categories

Built-in types (extensible by blueprint) include:

- **Fact / observation:** A supported statement about the world or system.
- **Decision record:** Decision, rationale, alternatives, decision makers if known, scope, and date.
- **Constraint / policy:** A rule or condition affecting implementation or interpretation.
- **Discovery / lesson / skill:** A reusable finding, method, or operational practice.
- **Risk / issue / open question:** A concern that should not be represented as a resolved fact.
- **Hypothesis / inferred insight:** A derived interpretation whose confidence and evidence must be explicit.
- **Temporal change / supersession:** A fact that modifies the applicable period or applicability of another memory.

### 10.2 Original evidence versus shared synthesis

**Original extracted memory:** A durable source observation extracted from one input session (or, in future, another approved source). It has an author, session ID, relevant excerpt/reference, extraction version, and temporal metadata. Do not merge two people's originals into an indistinguishable source item.

**Canonical/shared memory:** A versioned, readable project-level synthesis. It may be supported by multiple independent original observations, and those originals may introduce complementary qualifiers or apparent contradictions. The synthesized memory should state the narrowest justified scope and explicitly preserve disagreement when reconciliation is impossible.

**Derived memory:** A synthesis or inferred insight whose parents may themselves be canonical or derived memories. The provenance dependency structure for derived content must be **acyclic**, even though separate semantic graph relationships (e.g. “related to”) can contain cycles.

### 10.3 Required time fields

At minimum, distinguish:

- `observed_at`: When the source session or supporting evidence observed the condition.
- `recorded_at`: When Context accepted that memory or revision.
- `valid_from` / `valid_to`: When the claim is believed to apply in the domain.
- `last_confirmed_at`: Most recent independent confirmation, if any.
- `superseded_at` and `superseded_by`: When a replacement applies, without necessarily deleting historical truth.
- `scope`: Environment, service, endpoint, customer segment, experiment, or other qualifiers as appropriate.

Do not infer `valid_from` solely from `recorded_at`; uncertainty should be representable. Contradiction detection should evaluate temporal overlap and scope compatibility before classifying claims as genuinely incompatible.

### 10.4 Suggested memory schema (illustrative)

```json
{
  "memoryId": "mem_9f3",
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
  "originKind": "synthesized",
  "sourceEvidenceIds": ["orig_271", "orig_304"],
  "parentMemoryVersionIds": [],
  "supersedesVersionId": "mem_9f3_v3",
  "contradictionIds": [],
  "neptuneNodeId": "optional_graph_node_id",
  "policyVersion": "project-policy-12",
  "createdByJobId": "synth_331"
}
```

`confidence` is model/evidence judgment, **not** a calibrated guarantee. A production schema should explicitly differentiate direct sources, lineage parents, authorship, temporal state, and trust/review status. Version references are immutable identities even when the displayed memory has a newer version.

### 10.5 Optional machine-readable claims (future phase)

A memory's natural-language `text` remains its primary, human- and agent-readable form. In a future phase, extraction may also identify a small set of **structured claims**—atomic, scope-qualified propositions whose entities, relationships, values, time, and evidence can be inspected by an inference or rule engine. These are **not** required for a memory to exist, be searched, or be useful. A structured claim has its own stable version identity and provenance, links back to the text memory, and must be retractable without leaving an unsupported formal inference in the graph. See **§28** for the representation, RDF/OWL-versus-SMT tradeoff, Neptune Analytics specifics, and safety contract.

---

## 11. Architectural model

### 11.1 One logical graph, selectively materialized

The existing retrieval pattern is retained:

```text
Claude Code / Codex / agent host
          |
          v
Context plugin + MCP/agent tools + session hooks
          |
          v
Context service API / authorization / project selection
          |
    +-----+-----------+------------------+
    |                 |                  |
    v                 v                  v
Project + policy   Session ingest    Search orchestrator
management         & extraction       (project-aware)
    |                 |                  |
    |                 v                  v
    |          Memory synthesis     OpenSearch Serverless
    |          & provenance         (hybrid retrieval)
    |                 |                  |
    |                 v                  v
    |          Source/canonical    IDs / graph links
    |          memory indexing           |
    |                 |                  v
    |                 +----------> Neptune Analytics
    |                              (entity/relationship
    |                               traversals when linked)
    |                                      |
    +--------------------+-----------------+
                         |
                Feedback / Impact / Briefing
                         |
                  Night Shift scheduler
                         |
                Rollback / resynthesis
```

**Agreed storage direction:** All project memories are logical members of the existing graph. **A memory with no useful semantic/entity links can live only in OpenSearch Serverless**; it should not require a Neptune node merely to be searchable. When entity relationships or graph traversals are valuable, materialize the relevant memory node and links in Neptune and retain the graph node ID in the OpenSearch document. Search results without a Neptune ID still work as ordinary results.

### 11.2 Provenance is a distinct indexing concern

A crucial design distinction: **having provenance does not mean every memory must become a Neptune entity node.** The system must retain parent/child dependency identity and support efficient reverse traversal, even for OpenSearch-only memories. Recommended options:

1. **Dedicated durable lineage index (proposed preference):** Store versioned parent → child dependency edges in a service-managed index, such as a DynamoDB adjacency representation or another durable store that supports both forward and reverse lookup. Keep IDs/summary metadata in OpenSearch; materialize entity edges in Neptune selectively.
2. **Neptune-backed lineage for every source relationship:** Simpler dependency traversal conceptually, but would defeat the desire to minimize graph materialization.
3. **Only arrays in OpenSearch documents:** Easy to prototype, but reverse transitive fan-out and large rollback work become expensive and operationally fragile without a secondary adjacency index.

**Recommendation:** Keep the **semantic/entity graph** selectively materialized in Neptune, while ensuring the **provenance dependency DAG** is separately complete and efficiently queryable. This introduces a logical indexing component but preserves the one-graph product model. The underlying storage technology is an implementation choice, not a customer-visible concept.

**Future-phase distinction:** The optional *formal claim graph* is an additional semantic projection, not a replacement for the independent complete provenance index. A claim is stored/materialized in Neptune only when structured traversal or inference justifies it; the full source-to-output dependency and rollback record must remain available even for OpenSearch-only claims. Neptune Analytics supports RDF N-Triples import, but its query interface is openCypher, not SPARQL; OWL/RDFS reasoning requires an external component. See §28 and its references.

### 11.3 Major components

| Component | Responsibilities |
|---|---|
| **Agent integration layer** | Project creation/selection, visible status, tool discovery, persistent activation, session-start/end capture and retry. |
| **Project service** | Graph-instance association, lifecycle, membership, blueprint snapshot and policy versions. |
| **Privacy/policy evaluator** | Global exclusions, project restrictions, contribution steering, automated correction eligibility, logging minimization. |
| **Trajectory ingestion service** | Accept filtered session data, deduplicate hook deliveries, track completeness, attach immutable source identity. |
| **Extraction service** | Identify original typed facts, decisions, risks and skills from source sessions according to policy. |
| **Synthesis/contradiction service** | Group related evidence, preserve nuance, reconcile or flag conflicts, publish versioned shared memories. |
| **Provenance service** | Maintain immutable source IDs, dependency DAG, reverse adjacency, blast-radius queries, evidence audit. |
| **Memory indexing/linking** | Index OpenSearch documents; link to Neptune nodes on demand; update/deprecate stale graph links. |
| **Search orchestrator** | Resolve active project, apply ACL, hybrid search, boost/rerank, traverse links, return provenance-aware results. |
| **Feedback and impact service** | Correlate retrieval with later usage, correction, contradiction and optional user feedback. |
| **Rollback coordinator** | Preview scope, select mode, execute idempotent staged resynthesis, track partial failures and completion. |
| **Night Shift orchestrator** | Scheduled/on-demand upkeep, synthesis, stale detection, evidence-based inference, run reports. |
| **Briefing service** | Summarize what changed since a user's previous visit, with evidence links and action suggestions. |
| **Structured-claim + reasoning service (future)** | Extract optional typed claims from memory versions, validate their schema, project selected claims into Neptune, evaluate governed rules, and record alternate derivation justifications. Not on the critical path for creating or searching text memories. |

### 11.4 Tenant and project isolation

Every object carries tenant/context-instance and project identity. Every service must independently enforce authorization; a client-provided `projectId` is a scope request, **not** an authorization grant. Keep projects independent even if their memories link to entities in a shared graph. Do not permit one canonical memory to be owned by multiple projects in this phase. Shared entity links are not license to read another project's memory.

---

## 12. Ingestion, extraction, and synthesis pipeline

### 12.1 Lifecycle

1. **Activate:** User selects a project in Claude Code. Plugin persists the activation and shows it.
2. **Start session:** Integration creates/records a stable session ID and snapshots project, user, agent, workspace, policy versions, and capture configuration.
3. **Capture trajectory:** Session hook captures the authorized trace or a filtered/structured version. Client-side protection is preferred where feasible; server-side validation is required.
4. **Submit reliably:** Session-end hook submits data or a reference. Handle shutdown failures, offline sessions, retries, duplicates, and partial traces without silently changing provenance.
5. **Apply policies:** Enforce personal exclusions and project restrictions before shared persistence. Interpret project goals/blueprint and member focus as extraction priorities.
6. **Extract originals:** Produce independently attributed, typed observations with times, scope, evidence references, extraction model/policy versions, and sensitivity labels.
7. **Link and reconcile:** Compare with project memories, perform duplicate detection, contradiction/scope/time analysis, and identify related graph entities.
8. **Synthesize:** Create or update a canonical/shared memory, retaining all relevant originals and nuanced qualifiers as separate provenance inputs. Do not silently drop a minority observation solely because the majority agrees on a simpler statement.
9. **Index and materialize selectively:** Publish to OpenSearch and, when useful, graph links in Neptune. All published content references valid provenance records.
10. **Collect feedback:** Retrievals, observed use, corrections, and new evidence may queue re-evaluation and adjust future extraction priorities.

**Future option, not an added onboarding step:** Between extraction and indexing, a *formalization pass* can propose typed claims from selected memory versions, resolve entities, validate shapes, and send high-value assertions for rule evaluation. Both valid and rejected formalization outputs are traceable to the same source/policy snapshots; failure of this optional pass must not block normal text-memory indexing. Derived formal claims are published only with their evidence/justification records.

### 12.2 Multi-author synthesis example

- Avery: “Retries of non-idempotent charges are unsafe without idempotency keys.”
- Sam: “The v2 charge endpoint now accepts idempotency keys after the October rollout.”
- Jordan: “Legacy charge requests still bypass the v2 gateway.”

The synthesis must not produce a simplistic “retries are safe now.” It should distinguish endpoint and rollout scope, retain all three original observations, and expose unresolved assumptions. If Sam retracts their session, the system re-synthesizes from Avery and Jordan alone; it must not retain the v2 claim merely because it appeared in an earlier combined memory.

### 12.3 Source and derivation guarantees

- A memory revision is never supported merely by its own previous generated wording; it must trace to surviving independent sources or explicitly derived, independently supported inputs.
- All transformations record input version IDs, policy/model versions, and a job ID. No dependency graph cycles are allowed.
- A contradictory source is not removed simply because another source disagrees; the system can preserve both with a dispute status until enough evidence resolves them.
- If a source must be purged for privacy reasons, preserve only necessary **non-sensitive** audit metadata (IDs, timestamps, action types, job states); do not retain the prohibited text in an “immutable” audit log.
- Different contributors may support the same fact independently. Retraction removes a source's support, not the fact if other valid sources continue to establish it.

### 12.4 Determinism: define what can actually be guaranteed

**Desired invariant:** Once a source is retracted, no completed, active project memory should depend on that source, directly or indirectly. A clean reconstruction should use only authorized, non-retracted evidence and reproducible transformation inputs.

For operational determinism, use fixed source sets, explicit ordering, immutable version IDs, version-pinned prompts/models/configuration where available, idempotent jobs, and content hashes. **A generative model cannot be assumed to reproduce byte-for-byte language or provide a mathematical guarantee against every semantic echo.** The enforceable guarantees are: complete recorded lineage; exclusion of retracted evidence from resynthesis inputs; no published active memory with a dependency path to an excluded source after a **completed full rollback**; consistent state transitions; and validation/search checks for possible leaks. Stronger semantic non-leakage claims require additional validation and narrowly specified threat models.

---

## 13. Retrieval and context overlay

### 13.1 Three explicit search modes

1. **Blended (default while active):** Search the active project's memories plus the user's otherwise accessible context graph. Project relevance is an input to ranking, **not an unconditional override**.
2. **Project-only:** Search memories owned by the specified project, with optional authorized linked-entity evidence if policy explicitly allows it. Useful for audits and narrowly scoped queries.
3. **General/no-project:** Search the wider graph as it behaves today, without an active project's boost or project-only filter.

A programmatic retrieval API should accept an optional `projectId` and mode. If the active plugin session supplies a project and the caller does not explicitly override it, default to blended retrieval. Always check membership; no implicit fallback that accidentally reveals project content.

### 13.2 Retrieval stages

1. Resolve caller identity, tenant/context instance, membership, and requested mode.
2. Query OpenSearch hybrid search with project metadata filters or boost weights as appropriate.
3. Incorporate relevance to project goals, exactness, temporal applicability, source confidence, recency, and contradiction/review state.
4. For returned documents with `neptuneNodeId`, perform the existing Neptune Analytics traversal to enrich results with related entities and links.
5. Deduplicate OpenSearch and Neptune representations of the same logical memory.
6. Rerank combined evidence; handle contradictions as qualified alternatives, not silently pick the most recent text.
7. Return appropriate source references, status, time, and provenance so the agent can ground its response.
8. Emit a retrieval event for future impact and effectiveness analysis, observing data minimization and privacy rules.

### 13.3 Conflict and ranking policy

- **Project boost:** A project memory should generally outrank an equally relevant generic graph fact about the same project, but not override clearly better or more current evidence.
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

Model a directed acyclic provenance graph of **versioned derivation dependencies**, independent of the semantic/entity relationships in Neptune.

```text
Session S-A ──> Original A ─┐
                            ├──> Canonical C v1 ──> Derived D ──> Insight E
Session S-B ──> Original B ─┘           │
                                        └──> Another derived memory F
Session S-C ──> Original C ─────────────┘
```

A graph node in this diagram is a logical memory or specific memory version, **not necessarily a physical Neptune node**. A source/session → original edge establishes provenance. A source/derived → derived edge establishes synthesis dependency. The system must be able to traverse parents, children, and **transitive descendants**, count the blast radius, and locate unaffected independent original evidence.

A full rollback of S-A **does not delete Original B or C**. It invalidates the syntheses that used Original A, then re-derives supported results using B and C. If the new evidence cannot support a claim, that claim is retired. If a downstream insight depended entirely on the removed claim, it too must be retired or re-derived from other independent evidence.

### 14.2 Preview first; do not surprise the user

A rollback plan reports, at minimum:

- Target: one source memory, all originals from a session, or an identified set of contributed facts.
- Direct original memories and linked shared memories.
- Number of **distinct** transitive descendants, maximum dependency depth, affected versions, and estimated work. Counting shared nodes only once matters.
- Which memories have independent surviving source support and which may be retired.
- Number of graph links, OpenSearch documents, index versions, and caches expected to change (when estimable).
- Scope/permissions, chosen rollback mode, progress behavior, and **residual-leakage caveats**.
- Rough cost/time bands if useful; estimates must be labeled as estimates.

A preview should avoid dumping sensitive values in UI or logs. The preview can be revised if lineage changes before execution; the execution takes a consistent source/version snapshot or revalidates before committing.

### 14.3 The three user-facing rollback modes

| Mode | Behavior | Result and caveat | Suggested use |
|---|---|---|---|
| **Full recursive rollback** | Traverse all recorded descendants, invalidate impacted versions, re-synthesize from surviving clean evidence, publish clean replacements or retire unsupported memories. | **Complete for the active project memory dependency graph when the job finishes successfully.** Potentially expensive. Not immediate while pending. | Sensitive disclosure, mistaken session contribution requiring thorough removal. |
| **Bounded rollback** | Retract direct inputs, process descendants only to a user-chosen dependency depth. | Faster and scope-controlled, but deeper memories can still encode the removed information; **not a complete information purge**. | Ordinary quality corrections and non-sensitive cleanups. |
| **Bounded rollback + semantic leakage scan** | Run bounded rollback, then extract distinctive facts/identifiers from retracted material, search the entire eligible project knowledge corpus for them regardless of depth, verify provenance paths when found, and selectively repair matched dependents from clean evidence. | More coverage for dispersed leakage without full recomputation. **Best effort only**; semantic misses and indirect implications remain possible. | Large fan-out where exhaustive rollback is costly but fact-specific leakage is a concern. |

**Agreed:** The chosen depth and semantic scan are optional decisions at rollback time, not complex mandatory project settings. Show a strong recommendation for full rollback when the user says a session contains sensitive information. Avoid overstating semantic-scan completeness.

### 14.4 Full rollback execution (proposed algorithm)

1. **Authorize and snapshot.** Verify rights to retract the target source/session, capture its version, and freeze the work plan or establish safe optimistic concurrency.
2. **Mark original support retracted.** Exclude the selected source evidence from *all future extraction, synthesis, and correction inputs*. Create content-safe tombstones as needed.
3. **Find descendant closure.** Walk reverse dependency edges to enumerate impacted memory versions across **arbitrary depth**. No fixed derivation-depth limit; execution is partitioned into bounded batches for resource safety.
4. **Compute affected ordering.** Process dependencies in topological order; account for shared descendants and concurrent updates.
5. **Build clean input sets.** For each affected output, retain only independent originals or previously rebuilt ancestors that remain permitted; never feed contaminated old generated text back in as authority.
6. **Re-synthesize or retire.** Generate a new canonical/derived version from clean evidence, or mark it unsupported/retracted. Preserve relevant scope, uncertainty, and temporal qualifiers.
7. **Update indexes and links.** Replace/retract OpenSearch versions and reconcile Neptune entity edges as needed; do not leave stale graph traversals pointing to a retired active claim.
8. **Validate lineage closure.** Query dependency records to verify no **active, completed-scope** memory has a path to the retracted source. Run optional content/fact scans as defense in depth.
9. **Finalize and report.** Mark the job complete only after publishing and validation succeed; report any residual failures, excluded out-of-scope resources, and retention caveats.

**Important availability choice (agreed):** While a job runs, prior active memories remain generally searchable instead of being universally hidden. This preserves availability, **but it also means sensitive content can still be retrievable until the affected items are actually updated.** The initiator must see that the job is pending, what has finished, and what is still exposed. Do not market “full rollback” as instantaneous privacy erasure. For severe exposures, a separate future *targeted redaction or query-time masking* mechanism might reduce risk without taking whole memories offline; that would be an additional design choice, not a reason to silently change the agreed availability behavior.

### 14.5 Bounded rollback execution

Given `maxDepth = d`, process affected descendants with minimum dependency-path distance ≤ `d` from the source. At the boundary, record the count of known further descendants not reconstructed. A job can finish successfully **within its requested boundary** while remaining explicitly **incomplete as an information-removal operation**. The status should say, for example:

```text
Bounded rollback finished (depth 2)
9 memory versions rebuilt; 2 retired.
41 deeper dependent memories were not reconstructed.
Some downstream information may still reflect the retracted source.
```

The system should allow a user to later expand the depth or upgrade to a full rollback based on the same source identities and prior job history.

### 14.6 Optional semantic leakage scan

The discussion proposed a way to catch a sensitive fact even when it has traveled 30 derivation hops. The algorithm should be explicit about what it can and cannot prove:

1. **Create search fingerprints:** Extract distinctive identifiers, values, entities, propositions, and relationships from the retracted source. Examples include exact strings, normalization variants, structured facts, secure exact-match fingerprints where feasible, and semantic paraphrases. Minimize retention of sensitive raw values.
2. **Search broadly:** Search all project-owned active OpenSearch memories, and optionally *authorized linked graph content*, regardless of lineage depth. Use exact/pattern matching where possible and semantic retrieval to find paraphrases or indirect restatements.
3. **Verify lineage:** For each candidate, check whether it has a recorded transitive dependency path to the retracted source. This avoids removing an independently evidenced fact solely because its text resembles the retracted fact.
4. **Selectively repair:** If a contaminated memory is found far beyond the bounded rollback boundary, reconstruct it and any **necessary intermediate derivations** from clean evidence (or retire it). Do not re-feed a contaminated intermediate as an authority merely to save work.
5. **Recheck and report:** Scan repaired results, document examined scope, matches, repairs, and uncertainty. Preserve a path to escalate to full rollback.

**Limitations:** Semantic retrieval may miss obscure transformations, absent matches are not proof of absence, facts may be implied rather than stated, exact match on confidential values is not always safe to store, and some content outside the project's edit scope may be detectable but not directly removable. Searching a user's wider graph may reveal a suspect reference, but **automatic mutation remains limited to owned project memories and other specifically authorized resources**. External findings should be surfaced for separate handling, not silently edited across authorization boundaries.

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

This rollback is not yet complete. Affected memories remain generally
searchable during processing.
```

Avoid showing the same progress warning to every teammate retrieving a memory by default; the requester needs transparent progress. Other retrieval responses can still incorporate ordinary stale/contested confidence signals where they are already part of standard retrieval behavior.

### 14.9 Deletion, history, and retained source material

Two apparently competing requirements must be resolved carefully:

- **Keep originals:** For valid contributions, originals are retained as independent evidence so merged memories can be reconstructed faithfully.
- **Retract or purge disallowed information:** A rolled-back or privacy-excluded original must no longer be an active input. If the operation requests removal of sensitive source content, that content must be removed from active originals, derived memories, searchable indexes, source snapshots/caches, and applicable retained trace stores according to the deletion policy. Historical job metadata may remain, but not a copy of the prohibited content hidden in a “history” feature.

Do not conflate **retract support** with **erase every stored copy**. Define and display retention behavior for raw traces, immutable storage, backups, delayed indexes, logs, and audit metadata. A completed active-memory rollback guarantee is narrower than a verified full-storage erasure claim; retention compliance and backup expiry require separate product/security specifications.

### 14.10 Future integration: formal claims and rule-derived facts

A full rollback must eventually traverse **both** narrative-memory dependencies and formal-claim justifications (including rule-derived facts). It must retire an inferred claim when every justification depends on a retracted input, while preserving a claim independently supported by a different source or valid rule proof. Bounded modes must disclose that unprocessed formal inferences may also retain the retracted information. Use claim-version IDs and derivation/justification IDs, not raw string equality, as authoritative dependencies; semantic search is supplementary. The independently maintained lineage ledger remains the removal authority. Details and invariants appear in §28.7.

---

## 15. Feedback, memory effectiveness, and demonstrating value

### 15.1 Why memory count is insufficient

The product must make its usefulness visible, not just announce that an extractor produced 40 memories. A large volume of low-quality memories may make search worse. Worse, a retrieved memory may be incorrect and lead to a mistake. Value measurement therefore follows **extraction → retrieval → observed application → feedback / later evidence → correction**.

### 15.2 Suggested event model

| Event | Meaning | Attribution caveat |
|---|---|---|
| `memory_extracted` | Source generated a candidate original | Not proof of usefulness. |
| `memory_accepted` / `memory_merged` | Source became independent evidence for project knowledge | Merge should credit all valid contributing originals. |
| `memory_retrieved` | Search supplied a memory in context | The agent/user might never act on it. |
| `memory_cited` / `memory_used` | Agent quoted, cited, or explicitly applied the knowledge | Observed use is stronger, but not necessarily correct. |
| `memory_helpful_feedback` | User confirmed usefulness or downstream evidence supports it | Positive signal, not guaranteed causal impact. |
| `memory_contested` | Later session/feedback provided conflicting evidence | Requires scope/time verification; not automatically a factual error. |
| `memory_misleading` | Review found earlier retrieved guidance wrong or materially deceptive | Negative effectiveness signal; track exposure and correction. |
| `memory_corrected` / `memory_retracted` | Content was revised or removed | Indicates hygiene and recovery, not inherently personal failure. |

Events should include project, memory **version**, retrieval session, time, and signal source. Correlate a later contradictory observation with previously retrieved versions where the provenance supports it. Do not assume that every later contradiction means the previous retrieval caused bad work.

### 15.3 `/context impact`

Show both individual and team perspectives, with separate measures for:

- **Contribution:** Independently accepted originals, decisions, discoveries, and meaningful additions to merged memories.
- **Reach:** How many teammate sessions retrieved them; distinct users and recency as appropriate.
- **Demonstrated usefulness:** Explicit citations, positive feedback, validated outcomes, or successful reuse where detectable.
- **Improvement:** Memories corrected, refined, or superseded due to this person's new evidence.
- **Negative signals:** Memories that were retrieved and subsequently identified as misleading, with context and remediation, not unqualified blame.
- **Project knowledge health:** Current, contested, stale, unresolved, or unsupported memories; potentially affected team workflows.

**Gamification ideas to test:** Quiet, useful milestones such as “Your architectural decision informed three teammates,” a weekly “knowledge saved from being rediscovered” digest (clearly an estimate if calculated), and recognition for **fixing** misleading memories—not just creating new ones. Avoid a single opaque leaderboard or punitive score that incentivizes oversharing, memory spam, false certainty, or reluctance to correct errors. Every numerator and denominator should be explainable.

### 15.4 `/context briefing`

Personalize to a member's last visit, interests, contribution focus, and current project goals. Suggested sections:

1. **Important changes:** New decisions, changed rules, newly confirmed facts, and time-sensitive updates.
2. **Team contributions:** Discoveries by other members that affect the viewer's likely work.
3. **Open issues:** Genuine unresolved contradictions, risky assumptions, and evidence gaps.
4. **Night Shift report:** Consolidations, deprecations, suggested actions, and new inferred insights.
5. **Your follow-ups:** Pending review requests, rollback progress, and memories that may need your input.

This should appear as an answer within the agent, with optional drill-down into each memory and source. Avoid duplicating everything the user already knows.

### 15.5 Adaptive quality improvement

Use positive/negative signals to refine memory extraction, entity linking, duplicate thresholds, reranking, and correction policy recommendations. The system may learn that a project lead contributes important design rationale while an implementer contributes service contracts and integration facts. Treat learned focus as **revisable suggestions**, not irreversible profiling. Personal global exclusions remain hard limits.

---

## 16. Night Shift — daily project intelligence maintenance

### 16.1 Purpose and cadence

**Night Shift** is the project's nightly maintenance and insight-discovery pass. It runs approximately **once per day**, with **on-demand execution** via the agent. The name is intended to set user expectations: more thorough, batched review rather than constant automatic rewrites. Time zone/window and run scheduling are implementation settings that normally remain hidden behind defaults.

### 16.2 Jobs Night Shift may perform

1. **Staleness detection:** Identify facts whose valid time, last confirmation, or environmental assumptions may have expired.
2. **Contradiction detection:** Compare overlapping claims, including temporal and scope qualifiers, and identify genuine disagreement.
3. **Consolidation and simplification:** Merge redundant or overly fragmented shared memories while preserving original evidence and material nuance.
4. **Deprecation and supersession:** Update confidence/state of outdated memories; link successor decisions rather than erase useful history.
5. **Gap discovery:** Compare known project goals with what the project knows; surface consequential missing evidence.
6. **Memory-effectiveness review:** Revisit memories previously retrieved and then flagged misleading or contradicted. Feed conclusions into future extraction and ranking.
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
- Existing collaboration rules and review-only rules govern automated publication. If policy requires author/owner approval for certain memory types, Night Shift produces proposals.
- Insights carry `originKind: inferred`, confidence, scope, and citations to supporting memories/source evidence.
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
Reviewed: 312 project memories
Consolidated: 11 overlapping memories into 4
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
- Default memory categories and **extraction ontology**: entity types, relationship types, qualifiers, common synonyms.
- Project extraction and synthesis instructions.
- Default prioritization of decisions, facts, risks, constraints, skills, and other typed knowledge.
- Contradiction resolution and high-confidence correction defaults.
- Retrieval relevance hints and freshness expectations.
- Night Shift cleanup, staleness, and inference instructions.
- Starter privacy/exclusion hints (which can **tighten**, never weaken global exclusions).
- Suggested optional roles or member contribution-focus prompts.

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

Context applies a versioned project-local override. It should summarize what changed, offer a preview of how future extraction is affected, and optionally offer a retrospective re-analysis of eligible existing source evidence. **Changing instructions should not silently rewrite every past memory.** Any retroactive transformation is an auditable job with lineage and rollback semantics.

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
| `/context memories mine` | Show originals and affected shared memories, filter by session/date/status. |
| `/context memories inspect <id>` | Show a memory's meaning, source evidence, time, and provenance. |
| `/context memories correct <id>` | Guided correction or flag for review, respecting collaboration mode. |
| `/context rollback preview <session-or-memory>` | Show impact and available rollback modes. |
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
- **What will be shared?** Extracted project memories, visible to project members—not the entire raw transcript by default.
- **How do I stop or undo it?** Leave/switch for future sessions; inspect and roll back past sessions.

No user should have to understand provenance, Night Shift, correction thresholds, or retention policies just to create a project, but those capabilities must be understandable when requested.

---

## 19. Proposed service API and plugin tool surface

The design should support both first-party slash commands and ordinary agent tool use. **All names and payloads below are illustrative contracts to be refined, not existing AWS APIs.**

### 19.1 Core operations

| Operation | Example agent-tool name | Expected behavior |
|---|---|---|
| Create project | `context.projects.create` | Create from graph instance, name/goal, optional blueprint. |
| Read/list/update project | `context.projects.get/list/update` | Include goals, blueprint provenance, policy versions. |
| Invite/join/leave | `context.projects.invite/join/leave` | Enforce project membership and simple ACL. |
| Activate/deactivate | `context.projects.activate/deactivate` | Persist default project in agent integration. |
| Show activation | `context.projects.status` | Reveal actual current project and capture status. |
| Ingest session | `context.sessions.start/complete` | Record bound session identity and filtered trace input/reference. |
| Search | `context.memories.search` | Optional project, mode, temporal context, source explanations. |
| Inspect/correct | `context.memories.list/get/correct/flag` | Source, version, time, lineage, policy-dependent writes. |
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
- **Version-safe publication:** A search result should refer to a coherent canonical memory version and its supporting lineage. Cross-store updates need reconciliation so OpenSearch and Neptune do not disagree indefinitely about a memory's status.
- **Recoverable jobs:** Checkpoint rollback and Night Shift processing, retry transient failures, and expose dead-letter/retryable states to operators.
- **Concurrency control:** New memories must not inherit newly retracted source evidence during in-flight rollback. Project policy changes and night jobs must revalidate or serialize competing writes as necessary.
- **Resource bounds:** Full recursive rollback can be arbitrarily deep but must execute in bounded batches with rate limits, fairness, and cancellation/retry semantics. Avoid unbounded synchronous Neptune traversals or OpenSearch scans.
- **Document availability:** Do not make all pending-rebuild memories unsearchable; use explicit progress and post-publication validation to handle the eventual-cleanup model.

### 20.2 Security and privacy

- Enforce tenant, graph-instance, project, and author permissions on the server for **every** read/write/action. Never rely on a plugin to enforce authorization.
- Treat all agent traces and retrieved text as **untrusted data**. Extracted instructions from a session must not be allowed to alter global/project privacy rules, grant access, trigger deletes, or change autonomous correction thresholds. Tool usage and policy changes require authorized actions.
- Apply filtering before shared publication, with client-side minimization where supported, plus server-side scans. Redact secrets/PII/PHI-like material and configured patterns from logging, indexing, model prompts where appropriate, and reports.
- Separate shared-memory visibility from raw transcript visibility. Raw traces may include proprietary source code, credentials, or confidential deliberation beyond anything selected for project memory.
- Encrypt sensitive content in transit and at rest; define limits for raw trace retention, caches, backups, diagnostic logs, and model-processing retention.
- Bound regular-expression complexity and validate patterns to avoid catastrophic scans or denial of service.
- Provide administrative audit events for membership changes, policy changes, manual memory edits, full/bounded rollback requests, and Night Shift modifications.
- Deletion-compliance guarantees must be separately scoped across active indices, raw traces, caches, backups, and external systems; do not hide retention limitations.
- Avoid making a project appear safer than it is: extracted memory is intentionally shared with project members, even when its original source was not accessible to those members.

### 20.3 Performance and cost

- Avoid making every original memory a Neptune node. Materialize semantic relationships when they add retrieval/traversal value.
- Cache common project-search/ranking data subject to membership and invalidation rules; invalidate on correction, policy tightening, or rollback as needed.
- Prefer incremental Night Shift passes over full-corpus resynthesis, with periodic deeper passes only when valuable.
- Use lineage-adjacency indexes for blast radius instead of repeatedly scanning all documents to find descendants.
- Estimate rollback resource impact before starting expensive work; allow project/tenant quotas without silently downgrading a requested full rollback to bounded mode.
- Define measurable service-level objectives during implementation. Illustrative measures: create/join latency; project-search added latency; ingestion-to-first-memory delay; Night Shift runtime; rollback time-to-completion by blast radius; source/lineage correctness; and privacy-filter false positive/negative rates.

### 20.4 Observability

Suggested internal metrics: sessions attached per project; ingestion success/retry rate; extraction yield and acceptance rate; duplicate/merge rate; retrieval precision/feedback signals; graph-link success; contradictions resolved/reviewed; number of active vs superseded memories; stale-memory rate; Night Shift output/rejection rate; rollback nodes processed per minute; stuck jobs; pending contamination exposure; privacy-policy scan findings; and unauthorized-access denials.

**Design requirement:** Diagnostic telemetry and user-visible impact metrics should not accidentally contain the confidential memory content they are intended to protect.

---

## 21. Rollout and implementation plan (proposed)

The complete feature is the target described by this specification. The following is a staged engineering sequence to reduce risk, **not** a reduction in the feature's stated scope.

### Milestone A — First end-to-end customer value

- Project CRUD, membership, graph-instance association, Software blueprint.
- Claude Code agent tools, visible persistent activation, session association and reliable capture.
- Safe default global exclusions, project goals, optional contributor focus.
- Original source memories and a durable, queryable provenance model **from day one**.
- Canonical memory synthesis, OpenSearch indexing, selective Neptune links.
- Blended/project-only retrieval and evidence-aware answers.
- Basic author inspection, corrections, and **full rollback with visible processing status**.

**Exit criterion:** The hero scenario works from creation through teammate reuse and accidental-session remediation, with credible lineage and a documented post-processing guarantee. Do **not** ship automatic source combination without a way to correctly retract its inputs.

### Milestone B — Product value and control loop

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
- Smarter personalized learning from positive/negative memory signals.
- Additional supported agent hosts and richer project health signals.

**Exit criterion:** Large projects can perform cost-aware, transparent cleanup while distinguishing best-effort from thorough removal.

### Milestone D — Optional structured claims and reasoning (future exploration)

- Extract selected, versioned, scope-qualified claims from natural-language memories while preserving the text as the principal representation.
- Define a small, blueprint-extensible vocabulary and RDF-compatible mapping, validate extracted claims, and selectively materialize them in Neptune Analytics.
- Add an external, versioned rules/validation service for limited ontological and Datalog-style inference; keep inferred conclusions labeled and sourced.
- Make formal inferences participate in independent-justification tracking, privacy, project authorization, and full/bounded rollback before expanding rule coverage.
- Evaluate targeted SMT/SAT checks for high-value constraint questions, not as a default memory storage format.

**Exit criterion:** A project can infer useful, reproducible **conditional** conclusions from typed evidence and retract them correctly when a premise, time interval, or authorization changes, without requiring users to know RDF, OWL, or solver terminology. Milestone D **does not block** the earlier milestones or the planned clickable UX prototype.

**Scheduling caveat:** If users will be allowed to contribute sensitive information before milestone C, milestone A must already support dependable full rollback and deletion/retention controls appropriate to the stated privacy promise. An advanced semantic scan is not a substitute for basic safe filtering and lineage correctness.

---

## 22. Risks, tradeoffs, and mitigations

| Risk / tension | Why it matters | Proposed mitigation |
|---|---|---|
| **Persistent activation captures the wrong session** | Users may unintentionally share personal or unrelated work. | Highly visible active-project status, quick leave/switch, session list, retrospective rollback, safe privacy defaults. |
| **Project sharing bypasses source ACL semantics** | A shared project memory may summarize restricted upstream content. | Clear explicit-sharing disclosure, global/project exclusions, limited raw trace access; future fine-grained governance if required. |
| **False extraction or overconfident synthesis** | Agent memory can become authoritative-sounding but wrong. | Typed epistemic status, independent evidence, uncertainty, feedback, correction, human review steering. |
| **Contradiction vs temporal change** | Different observations may both be correct for different periods or scopes. | Model valid time and scope; preserve histories and unresolved qualifiers. |
| **Derived memory provenance explosion** | Arbitrary-depth DAG can make rollback enormous. | Complete dependency index, blast-radius previews, bounded batches, user-selectable cleanup mode. |
| **Bounded cleanup leaves information behind** | False confidence could cause privacy harm. | Honest scope/caveats, optional semantic scan, explicit full rollback recommendation for sensitive cases. |
| **Searchable while cleanup is pending** | Availability is preserved at the cost of continued exposure. | Prominent requester progress, no instant-erasure promise, optional future targeted redaction for severe cases. |
| **Semantic scan misses paraphrases/implications** | Best effort is not certainty. | Combine exact, pattern, structured, and semantic matching; verify lineage; offer full escalation. |
| **Models are nondeterministic** | A reproducible audit trail does not imply byte-identical generated text. | Versioned inputs/config, idempotency, provenance validation, explicit guarantees limited to recorded evidence dependencies. |
| **Impact metrics become social scoring** | Users might optimize for quantity or avoid correcting mistakes. | Separate exposure from helpfulness; value corrections; no punitive leaderboard by default. |
| **Night Shift hallucinates insights** | Automated maintenance could introduce unsupported beliefs. | Inferred labeling, evidence requirements, confidence, review controls, derivation-loop checks. |
| **Later formalization creates false precision or spreads a bad inference** | RDF/OWL/rules can give a mistaken extraction an unwarranted aura of mathematical certainty and propagate it to many descendants. | Treat input assertions as unverified until supported, preserve scope and provenance, validate or abstain, use controlled reviewable rules, and require correct retraction of every derived justification (see §28). |
| **Too many settings undermine simplicity** | Users abandon before first value. | Blueprint defaults, optional prompts, natural-language tuning, progressive disclosure. |
| **Cross-store consistency drift** | Search and traversal may show mismatched memory versions. | Stable IDs, versioned publication, reconciliation, lineage validation, idempotent updates. |
| **Incomplete session hooks** | Extraction may miss important context or duplicate work. | Retry/buffer, partial-session status, durable IDs, future incremental capture if host supports it. |

---

## 23. Open design questions and brainstorming backlog

These items are intentionally **not** presented as settled commitments.

### 23.1 Product discoverability and measurable value

1. What is the smallest compelling Impact view? Teammate reuse counts, “decision prevented repeated work,” quality improvements, or a combination?
2. What constitutes evidence of *actual benefit*, versus mere retrieval? Can an agent reliably detect that a retrieved memory changed a later decision or implementation?
3. How should negative effectiveness be framed? An educational correction signal rather than a punitive reputation score is recommended, but should users see detailed per-memory histories by default?
4. Would a daily/weekly briefing, an agent-start summary, or optional team digest create value without notification fatigue?
5. What feedback signals can be collected without storing or sharing excessive user/session detail?

### 23.2 Activation and agent-host behavior

6. Should activation be user-wide, agent-client-wide, repository/workspace-scoped, or some combination? The current proposal is per-user/workspace for safety, but the required behavior is persistent future-session participation with clear visibility.
7. Can Claude Code provide a sufficiently persistent status indicator through supported integration surfaces? What is the acceptable fallback for hosts with only hooks and tool responses?
8. If a user operates in multiple workspaces or devices, how should competing active-project settings be presented? Should a workspace remember a project across devices?
9. What happens when an agent session spans multiple repositories or project boundaries? Explicit switch should apply prospectively, with clear session association/versioning.
10. Is raw trajectory upload always needed, or can the plugin perform useful local redaction and first-stage extraction before transmission?

### 23.3 Memory and temporal reasoning

11. How is “same fact” defined across different authors, scopes, environments, and times? What confidence is needed to merge rather than link related memories?
12. Which time semantics should be mandatory versus optional per blueprint? How should unknown valid periods be represented?
13. What are default thresholds for automatic correction, and what classes of decision should always be review-first?
14. How are human-authored corrections represented—new independent evidence, editorial overrides, or both—and how do they participate in rollback?
15. How much raw/source evidence is retained, for how long, and who may inspect it after source contributions are shared?

### 23.4 Provenance and rollback

16. What precise storage engine should back the reverse lineage index while preserving sparse Neptune materialization?
17. How are race conditions resolved between ongoing synthesis, Night Shift, policy changes, and source rollback?
18. How should the system estimate transitive fan-out cheaply before traversing an extremely large graph? Can it provide progressively refined estimates?
19. Does bounded rollback use dependency path depth, derivation generation, or a user-facing cost budget? Depth is simplest initially; cost may be more understandable to some users.
20. What exact scope does “complete” certify: active project search/indexes, linked graph nodes, raw traces, cached answers, backups, and any external exports must be described separately.
21. Should the system offer immediate **targeted redaction** for a known sensitive identifier while a full rebuild runs, even though whole memories remain searchable?
22. How should semantic scanning search linked non-project graph content when it can inspect potential leakage but lacks permission to edit those objects?
23. How should rollback handle a memory with both contaminated derivation and independent support for the *same* claim? What evidence is enough to publish a clean replacement?
24. Is full source-session rollback restricted to the originating user, or should an authorized project owner/admin be able to initiate emergency privacy remediation on behalf of another member?

### 23.5 Collaboration, governance, and learning

25. Is **author-controlled** the best default collaboration mode, or is team-wide collaborative editing a better default for small projects?
26. Should changing the collaboration mode affect existing canonical memory edit rights immediately? What owner approval and audit are necessary?
27. Should individual contribution focus be user-entered only, suggested by the system, or learned and periodically proposed for approval? Privacy exclusions should never be auto-relaxed.
28. Should a blueprint be updateable in place, forked, or pinned at project creation? The current recommendation is pinned version + local overrides.
29. How should projects evolve when their original goals are met or change significantly? Should Night Shift suggest archiving, restructuring, or a goal revision?
30. How should a member leaving the project affect their previously contributed memories? No automatic deletion is currently specified; source-session rollback remains available subject to policy.

### 23.6 Night Shift and ontology

31. How frequently should deeper Night Shift sweeps run relative to daily incremental maintenance?
32. Which inferred insights may be auto-published versus held for review? How do we measure insight usefulness and hallucination rate?
33. Should project owners be able to add custom ontology types and relationships conversationally, or only override extraction instructions initially?
34. Which built-in blueprints deserve priority based on real customer usage and available evaluation sets?
35. Should Night Shift proactively suggest new project extraction instructions based on recurring missed or corrected memories?

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

**Given** an existing Context customer in Claude Code, **when** they say “Create a software project for Billing API Modernization,” **then** the agent creates a project in the selected graph instance, uses the software blueprint, activates it, explains sharing and privacy defaults, and offers—but does not require—a role description.

### AT-02 — Persistent, visible activation

**Given** a member activates the billing project, **when** they start a later Claude Code session in the applicable activation scope, **then** the project is attached automatically and prominently identified; a user can leave/switch without editing settings files.

### AT-03 — Session-bound source attribution

**Given** a source session produces two decisions, **when** the session-end hook is delivered twice, **then** Context creates one attributable set of original source memories with the original session ID and no duplicates.

### AT-04 — Teammate value from overlay retrieval

**Given** Avery contributes a gateway retry decision and Sam is a member, **when** Sam asks about retries, **then** blended retrieval surfaces the decision with applicable scope/time and relevant broader graph links, even though Sam did not participate in Avery's session.

### AT-05 — Independent multi-author evidence

**Given** two members independently support the same fact with different nuances, **when** Context synthesizes a shared memory, **then** both originals remain independently attributable and the combined memory preserves material qualifiers rather than flattening them.

### AT-06 — Temporal contradiction handling

**Given** an earlier decision about legacy endpoints and a later observation about a v2 endpoint, **when** they appear inconsistent in isolation, **then** the system distinguishes their time/scope before declaring a contradiction, with an unresolved status if evidence is insufficient.

### AT-07 — Global privacy override

**Given** a user's global policy excludes a protected identifier pattern but the project blueprint encourages extraction of account IDs, **when** the user's session contains that identifier, **then** no shared original/canonical memory reveals it and the blueprint cannot override the exclusion.

### AT-08 — After-the-fact session correction

**Given** a session was mistakenly attributed to a project, **when** its owner requests rollback, **then** the agent enumerates its originals, previews its transitive impact and independent surviving evidence, and offers full, bounded and bounded-plus-scan modes according to product availability.

### AT-09 — Full multi-author rollback

**Given** shared memory C was synthesized from originals A and B, **when** original A is retracted and full rollback completes, **then** C is re-synthesized from B alone if B supports it; otherwise C is retired. No active completed-scope derived memory has A in its lineage.

### AT-10 — Arbitrary-depth rollback

**Given** a chain of 30 derived dependencies, **when** a full rollback retracts the chain's originating source, **then** work progresses in bounded batches until every recorded descendant is rebuilt or retired, without imposing a fixed derivation-depth cutoff.

### AT-11 — Bounded cleanup caveat

**Given** a source has descendants beyond level 2, **when** a user chooses depth 2, **then** the job finishes within scope, reports deeper dependencies not rebuilt, and explicitly does not claim total information removal.

### AT-12 — Deep semantic leakage match

**Given** a distinctive retracted identifier remains in a project-owned memory 30 hops away, **when** bounded-plus-semantic rollback runs, **then** the broad search can find it irrespective of depth, verify its dependency path, and selectively rebuild it using clean evidence. A missed paraphrase remains a documented limitation, not a false guarantee.

### AT-13 — Search availability during rollback

**Given** a large rollback is in progress, **when** teammates search the project, **then** otherwise-searchable affected memories remain available until changed; the initiator can see pending status, progress, and potential residual exposure.

### AT-14 — Correction permissions

**Given** an author-controlled project, **when** a member attempts to change another author's original contribution, **then** the action is denied or offered as a flag/review; **when** the same member edits a shared memory in collaborative mode, **then** the edit is allowed with a versioned audit event but not access to the other author's private raw session.

### AT-15 — Briefing and Impact

**Given** teammates have created, reused, and corrected project memories, **when** a member requests Briefing or Impact, **then** they receive evidence-backed updates and separate measures for retrieval exposure, observed helpful use, and later misleading/corrected cases.

### AT-16 — Night Shift produces an inference safely

**Given** related evidence suggests a previously unstated migration risk, **when** Night Shift runs, **then** it may create an explicitly labeled inferred insight with confidence and provenance, honoring the project's auto-publish/review policy.

### AT-17 — Blueprint customization

**Given** a project created from the Data & Pipelines blueprint, **when** its owner asks to focus on schema drift and freshness SLAs, **then** only that project's versioned configuration changes; other projects and the stock blueprint do not silently change.

### AT-18 — Retrospective privacy policy scan

**Given** a user adds a new global exclusion, **when** they accept the offer to scan historical contributions, **then** the system finds candidate affected memories, presents a redaction-safe plan, and allows authorized rollback/remediation with visible progress.

### AT-19 — Project-only security boundary

**Given** a user does not belong to another project, **when** they try to retrieve its memory by searching with a guessed project ID or traversing from a shared graph entity, **then** they receive no unauthorized content or sensitive metadata.

### AT-20 — Trace-to-output deletion accounting

**Given** a source containing sensitive material is retracted, **when** active-memory rollback completes, **then** the system verifies the active OpenSearch/Neptune representation and tracked provenance no longer publish dependents of that source; raw trace, logs, caches and backup retention are reported under their separately specified deletion lifecycle rather than being falsely declared erased.

### AT-21 — Future structured claim remains optional

**Given** a memory whose content does not map confidently to an approved predicate, **when** extraction completes, **then** the natural-language memory is still searchable and no invented formal assertion is published.

### AT-22 — Future independent reasoning justifications

**Given** two independent justifications for the same inferred claim, **when** the session behind one justification is fully retracted, **then** the invalid proof and its dependency records are retired while the independently grounded conclusion remains, with its supporting source set accurately reduced.

### AT-23 — Future scoped contradiction and explicit negation

**Given** a fact that holds for v2 but not for a legacy endpoint, **when** a rule or constraint checker evaluates them, **then** different scope/time intervals are not falsely treated as contradictory. An absent claim must not silently be interpreted as a negative claim.

### AT-24 — Future cross-representation rollback

**Given** a source contributes a text memory, an RDF-aligned structured claim, and an inferred graph result, **when** full rollback completes, **then** every active formal and narrative output relying only on the retracted source is retired or rebuilt, every independent proof survives, and all active indexes agree on status.

---

## 25. Prototype handoff: what the later clickable HTML mock should demonstrate

The next design artifact, a self-contained clickable HTML/JavaScript prototype, should allow a stakeholder to experience the product **as a Claude Code session**, not as an administrator's dashboard. The feature should feel discoverable in-place.

**Prototype scenes:**

1. **Start in an existing agent:** A simulated Claude Code terminal with Context already available.
2. **Create a project:** Natural-language prompt, blueprint suggestion, goal and creation confirmation; minimal onboarding.
3. **Optional focus:** Prompt for the user's role, with “Skip” clearly available.
4. **Persistent active status:** A visible active-project cue that survives simulated “new session.”
5. **Session activity and extraction:** Agent code/review conversation, simulated end hook, reviewable original/canonical memories.
6. **Teammate perspective:** Switch mock identity to a colleague, query the graph, see project-boosted answer and source evidence.
7. **Briefing and Impact:** Clickable snapshots showing team reuse, useful outcomes, disputed memory, and Night Shift changes.
8. **Inspect memory:** Drill from a shared memory into source contributions, temporal qualifiers, and lineage tree.
9. **Correct/retract:** Select a mistaken session, preview transitive fan-out, choose rollback mode, and watch a simulated progress sequence.
10. **Project steering:** Adjust contributor focus, global exclusion, correction mode, and blueprint settings conversationally.
11. **Night Shift:** Trigger on-demand maintenance and inspect a new inferred insight with provenance and confidence.
12. **Progressive disclosure:** A newcomer never needs to encounter the advanced rollback or configuration screens until asking for them.

**Visual emphasis:** Lightweight active-project indicator, recognizable command output, compact evidence chips, clear before/after memory revisions, source provenance drilldown, and credible job-progress reporting. Keep the mock visually closer to an agent terminal than an enterprise admin console. Include a way to switch between the primary contributor and a teammate to demonstrate cross-person value. **Do not put RDF, OWL, claim tables, or logic expressions in the default clickable customer journey**; the optional formal layer is deliberately hidden. A future advanced evidence drilldown may illustrate a newly inferred insight and its supporting facts using plain language.

---

## 26. Recommended next design decisions

Before implementation begins, resolve these decisions in approximately this order:

1. **Activation scope and Claude Code visibility:** Precisely define where “active for future sessions” persists and validate plugin/status-hook capabilities.
2. **Source trace and privacy boundary:** Decide what is transmitted, retained, redacted, and shown to teammates versus source authors.
3. **Provenance index:** Choose the storage and version model for efficient arbitrary-depth reverse traversal without materializing every memory in Neptune.
4. **Correction and resynthesis invariants:** Define source identity, memory versioning, independent evidence handling, and concurrency constraints before synthesis ships.
5. **Rollback completion contract:** Distinguish active-memory dependency cleanup, optional content search, and raw-storage/backup deletion guarantees.
6. **Minimum first-run agent flow:** Confirm that a new user can create a project and recognize activation with almost no onboarding.
7. **Impact event fidelity:** Decide which usage/benefit signals can be reliably observed and what cannot be attributed.
8. **Night Shift review policy:** Decide what can auto-publish and how inferred insights remain visibly distinct.
9. **Blueprint evaluations:** Test software, research, ETL, and BI defaults on representative sessions and measure extraction quality.

---

## 27. Concise product contract

A successful Context Projects release lets a user create or join a goal-oriented project **from their existing agent**, stay visibly active across subsequent sessions, and build useful shared memories from those sessions without extra effort. Teammates get context they would otherwise miss. Contributors know what was learned, can see evidence of positive and negative effects, can correct mistakes and retract a wrong session, and can select how exhaustive a rollback should be with honest reporting of residual risk. Privacy exclusions apply across projects, project-specific contribution focus improves curation, and configurable collaboration keeps editing practical for both close-knit and larger teams. Night Shift maintains the project's health and discovers evidence-grounded insights. Blueprints make software, research, ETL, and BI projects useful immediately. Under the hood, the system retains original source evidence, versioned provenance across arbitrary derivation depth, temporal semantics, and one logical OpenSearch-first/Neptune-linked graph—while keeping those implementation details out of the beginner's way.

**North star:** *You should feel that the project remembers what your team learns, without making you manage its memory.*

---

## 28. Future-phase structured claims and formal reasoning

**Status: Proposed exploration / Milestone D, not part of the required beginner experience.** This section extends §10 (memory semantics), §11 (architecture), and §14 (provenance and rollback). The end-user product remains *create project → work normally → benefit from shared memories*. The formal layer is a service implementation detail, not a new concept the customer must learn. Nothing here requires changing the established text-first/OpenSearch-first experience or making every memory a Neptune node.

### 28.1 The problem: readable memory is not always sufficient for rigorous inference

Natural-language memories are the best lingua franca for agents: they retain nuance, rationale, exceptions, and unanticipated meaning. But a sentence such as “automatic retries are disabled on the legacy charge endpoint in production” might later need to participate in a calculation, rule, contradiction test, or dependency analysis. If its meaning is stored **only** as prose, each reasoning pass must reinterpret it and may inconsistently identify the entity, predicate, time period, or scope.

**Proposed dual representation:**

1. **Narrative memory remains authoritative for human-readable explanation.** Keep the exact versioned text, qualifiers, its original supporting contributions, and its provenance.
2. **Structured claims are optional projections** of selected atomic propositions from a particular memory version, with versioned schema/predicate identity. One memory may yield zero, one, or several claims; one synthesized memory can describe conflicting claims without choosing a winner.
3. **Derived claims are separately labeled conclusions**, not magically “more factual” because they came from a solver. Each has a rule or inference procedure, all premises, an explicit scope and time, and a reproducible/inspectable justification.
4. **No false precision.** Failed entity linking, ambiguous predicates, contradictory source language, unresolved time, or sensitive content should prevent formalization or leave a candidate for review. Text memory still works.
5. **Do not conflate a source's assertion with verified truth.** The same logical proposition may be asserted by two independent contributors, disputed by a third, later verified, or no longer applicable. Its truth status is a function of evidence, assumptions, and time—not simply the presence of a triple.

Structured modeling becomes worthwhile for particular high-value kinds of facts: service dependencies, implementation contracts, schema/entity relationships, feature or configuration constraints, ownership, policy applicability, research hypotheses, and traceable derived insights. Routine conversation summaries can remain text-only.

### 28.2 Which formalism should Spindle use?

| Formalism | Good at | Limitations for Spindle | Recommendation |
|---|---|---|---|
| **Versioned typed claim records** (JSON/schema) | Atomic proposition, identifiers, confidence/status, provenance, scope, time and lifecycle; straightforward indexing and updates. | The JSON structure itself supplies no interoperable logical semantics unless vocabularies/rules are specified. | **Use as a canonical application-level claim contract.** It is not intended to replace text memory. |
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
  "derivedFromMemoryVersionId": "mem_9f3_v4",
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
- **Security:** Project identity, security labels, and personal exclusion/purge state apply to typed values, entity IDs, literals, export, cache, and derived output—not only to the narrative memory text.
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

Do **not** make RDF-star, a named graph per session, or a specific reification feature a prerequisite: if unavailable in the chosen Neptune Analytics path, use explicit Claim nodes and regular edges with immutable IDs. Design the conversion and acceptance tests against the actual supported import/query surface. The user's normal memory result should continue to show the readable sentence and ordinary evidence links, not Turtle or a list of triples.

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

Formal reasoning makes the provenance problem *more* important. Merely knowing that a derived claim cites several ancestor memories does not explain whether they are **all required** or **alternative independent ways to support the same conclusion**.

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
4. For **bounded rollback**, keep the same honest eventual/incomplete-removal contract from §14. An inference several hops away may remain searchable while processing or if explicitly outside the selected bound; show that residual risk in the initiator's progress/result. The **semantic leakage scan** may find exact fact values or paraphrases in distant narrative memories, but cannot certify complete removal.
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
Session trace --privacy filters--> Original memory --> Shared narrative memory
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

**Spike 1 — Representation without a reasoner:** From a curated evaluation set of Software and Research project sessions, extract a few predicates (service dependency, API ownership, configuration value, explicit decision, hypothesis). Compare proposed claim values against human labels, log refusal/unknown rates, and validate scope/entity linking. Text memory must be unaffected by rejected claims.

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

## 29. Terminology review and naming options

**Status: Naming candidates only; no unilateral rename of the agreed product design.** The internal engineering codename is **Project Spindle**; the current working customer feature name remains **Context Projects**. This section is an editorial design pass, not a new list of user concepts. Its purpose is to make the product feel like a teammate quietly remembering things rather than a graph-management console. **Night Shift** is a strong example: memorable, self-explanatory, and friendly without hiding a surprising action.

### 29.1 Recommended language principles

1. **Name the outcome, not the implementation.** Say “what we learned,” “sources,” “undo a contribution,” and “what changed,” not “canonical synthesis,” “provenance closure,” or “incremental inferencing.” Keep technical words in APIs, logs, developer specs and advanced evidence views.
2. **Favor plain-English nouns with a touch of personality.** “Night Shift,” “Briefing,” “Blueprint” and “Impact” are already relatively good. Introduce quirkier names only where the user can guess what happens without a tooltip.
3. **Keep distinctions that protect trust.** Never name a *partial* rollback “fully removed”; never call weak retrieval correlation “impact”; never call an LLM hypothesis a “proven fact.” A delightful label must not oversell its guarantee.
4. **One visible term per action.** Avoid shipping “archive / retract / rollback / prune / invalidate / forget” as five different customer verbs when users really mean “remove my contribution.” Advanced technical depth/mode choices appear only when needed.
5. **Prefer everyday words in agent conversation.** The user can say “Catch me up on Billing API,” “What have I helped with?”, or “Please undo what you learned from Tuesday's session.” The agent uses exact tools internally.
6. **Let the product's personality emerge in moments of value.** “Your decision helped two teammates,” “Night Shift found a possible conflict,” or “Three loose ends still need a decision.” Avoid playful names for privacy-critical buttons if they reduce precision.

### 29.2 Terminology audit: what to keep and what could be friendlier

**Recommendation legend:** **Keep** = the present name is already good. **Consider** = a proposed customer-visible alias worth prototyping. **Internal** = precise systems term that need not appear in the default UX.

| Current concept / term | Recommendation | Friendly / playful candidates | Rationale and suggested UI wording |
|---|---|---|---|
| **Context Projects** | **Keep** | Projects, Team Projects | Self-explanatory feature name. In the agent, “Create a project” is enough when Context is already installed. |
| **Project Spindle** | **Keep internal** | — | Strong, memorable engineering codename. Avoid making it another customer-facing noun on top of Context Projects. |
| **Project blueprint** | **Keep** | Starter Kit, Recipe, Starting Point | “Blueprint” effectively implies defaults you can customize. **Starter Kit** is warmer; **Recipe** adds charm but could imply a fixed workflow. |
| **Active project / activation** | **Consider simplification** | Working in, Current Project | Display **“Working in: Billing API Modernization”** rather than “activation binding configured.” |
| **Project member** | **Keep, with conversational alias** | Teammate, Collaborator | “Invite teammates” is friendlier; “member” is fine for permission checks. |
| **Project goal / steering** | **Keep goal; hide steering** | What Matters, Project Focus, North Star | Let users say “What should this project remember?” rather than “configure extraction steering.” |
| **Personal contribution focus** | **Consider** | My Focus, My Lens, What I Bring | “My Focus” is immediately understandable. “My Lens” is evocative for architects/researchers but slightly abstract. |
| **Global personal privacy exclusions** | **Consider** | Never Share, Off Limits, Private by Default | **“Never Share”** communicates the highest-priority rule; “Off Limits” is friendly but less explicit about extraction versus sharing. |
| **Original extracted memory** | **Internal; simplify in review UI** | Source Note, Session Note, Original Note | **“Source Note”** implies the direct observation and contrasts well with “Shared Memory.” |
| **Canonical / synthesized memory** | **Internal; use existing shared memory** | Shared Memory, Team Memory, Combined Note | **“Shared Memory”** is clearer than “canonical,” which incorrectly suggests an indisputable truth. “Combined Note” describes the process. |
| **Memory synthesis / consolidation** | **Internal** | Combine Notes, Stitch Together, Weave | **“Combined from three source notes”** conveys provenance; “Weave” could be a playful advanced activity label, not a core concept. |
| **Inferred insight** | **Keep insight** | New Connection, Discovery, Hunch | **“New insight”** is clear. If weakly supported, **“Possible connection”** or **“Hunch”** helps avoid overconfidence. |
| **Contradiction detection** | **Consider** | Conflicts, Reality Check, Disagreements | Show **“Possible conflict”** when time/scope don't yet prove incompatibility. “Reality Check” is a nice name for an optional review action, not a claim of authoritative truth. |
| **Feedback / memory effectiveness** | **Consider** | Was This Helpful?, Memory Health, Pulse Check | **“Was this helpful?”** is better than an abstract effectiveness score; **“Memory Health”** fits a periodic Night Shift summary. |
| **Night Shift** | **Keep strongly** | Night Crew, After Hours | Already friendly and accurately signals daily asynchronous upkeep. Don't rename without a compelling reason. |
| **Briefing / `/context briefing`** | **Keep** | Catch-up, Morning Brief, What Changed? | “Briefing” is strong. **“Catch me up”** is the natural-language phrase; “Morning Brief” may wrongly imply it runs only in the morning. |
| **Impact / `/context impact`** | **Keep** | Ripples, Footprint, Contributions | “Impact” is clear and measurable if instrumented. **“Ripples”** is a distinctive playful alternative that suits cross-team reuse; still explain what was actually observed. |
| **Retrieval overlay / relevance boost** | **Internal** | Project-aware search, Results from your project | A teammate doesn't need to know the merger/reranking algorithm. Say **“Found in your project”** when relevant. |
| **Provenance / dependency DAG** | **Internal** | Evidence Trail, Thread Map, Where It Came From | **“Evidence Trail”** is the best clear customer alias; **“Thread Map”** fits Spindle's internal metaphor for an advanced graphical view. |
| **Blast radius / transitive fan-out** | **Internal; simplify** | Affected Memories, Ripple Preview, What Else Changes? | **“This undo affects 47 memories”** is clearer and less alarming than “blast radius 47.” |
| **Rollback / retraction** | **Consider** | Undo Contribution, Remove Session, Unwind | **“Undo what this session contributed”** is natural. **“Unwind”** is a fun shorthand but requires a caveat for partial cleanup. |
| **Full recursive rollback** | **Consider** | Thorough Undo, Full Cleanup, Complete Rebuild | **“Thorough undo (all known dependent memories)”** explains the scope without claiming instant erasure of logs or backups. |
| **Bounded rollback** | **Consider** | Limited Undo, Quick Cleanup, Selected Depth | **“Limited undo”** is accurate. “Quick Cleanup” alone risks implying completeness; retain a visible residual-risk statement. |
| **Bounded rollback + semantic leakage scan** | **Consider** | Deep Sweep, Echo Check, Extra Search | **“Limited undo + Deep Sweep”** is memorable; clearly label as a *best-effort search* rather than guaranteed removal. |
| **Project ontology / formal claim** | **Internal** | Facts & Relationships, Structured Knowledge | Users should normally only see clear claims and sources, not RDF/OWL terminology. **“Facts”** can overstate certainty; “structured knowledge” is safer internally. |
| **Project health / stale memories** | **Consider** | Pulse, Health Check, Needs Attention | **“Night Shift: 4 items need attention”** avoids creating another dashboard noun. “Pulse” is a possible later glanceable view. |
| **Open questions / unresolved risks** | **Keep** | Loose Ends, Things to Decide | **“Loose Ends”** is friendly and useful for a Night Shift or Briefing section, provided it doesn't trivialize important risks. |

### 29.3 Candidate coherent naming directions (options, not decisions)

**Option A — Clear and dependable (recommended baseline):** Projects, Blueprints, Shared Memories, **My Focus**, **Never Share**, **Briefing**, **Impact**, **Night Shift**, **Evidence Trail**, **Undo Contribution**. Almost all original names remain; just the clinical terms disappear from beginner UI.

**Option B — Subtle Spindle flavor:** Projects, Blueprints, Shared Memories, **My Lens**, **Off Limits**, **Catch-up**, **Ripples**, **Night Shift**, **Thread Map**, **Unwind**, **Loose Ends**. More memorable and a little quirky, with a metaphor of threads, connections and ripple effects. Best for expressive surfaces, not safety-critical permissions or deletion actions.

**Option C — Developer utilitarian:** Projects, Templates, Memory, Role, Privacy Rules, Changes, Usage, Daily Maintenance, Lineage, Rollback. Technically clear, but comparatively dry and more like a configuration console than an agent-native collaborator.

**Current preference:** Use **Option A** in the first UX prototype, sprinkle in selected Option B expressions as secondary labels or human copy (especially **Ripples**, **Loose Ends**, **Thread Map**), and retain existing API/command identifiers until tested. “Night Shift” remains fixed in all directions.

### 29.4 Small copy experiments for the clickable agent prototype

| Situation | Clinical wording to avoid | Suggested conversational wording |
|---|---|---|
| New project created | “Instantiation succeeded. Extraction policy initialized.” | “You're working in **Billing API Modernization**. I'll remember useful project decisions as you work.” |
| Invite/join | “Membership grants canonical memory visibility.” | “Your teammates can see memories shared with this project. Your private session transcript stays private.” |
| Optional role setup | “Set participant-scoped extraction steering.” | “What do you work on here, and what should your teammates learn from your sessions? (Optional)” |
| Active status | “Active project context binding: `proj_billing`.” | “**Working in: Billing API Modernization** · Sharing project memories” |
| Night Shift | “Contradiction and inference batch completed.” | “**Night Shift finished:** 2 decisions updated, 1 possible conflict, 1 new connection.” |
| Briefing | “Canonical artifact delta since last read cursor.” | “**Here's what changed since you were last here.**” |
| Impact | “Retrieval-event attribution indicates utilization.” | “Your architecture decision was used in 3 teammates' sessions. One use led to a correction.” |
| Evidence drilldown | “Source attribution DAG of synthesis version 4.” | “**Where this came from:** 2 session notes, combined on Tuesday.” |
| Partial undo | “Depth-limited rollback complete with unresolved descendants.” | “**Limited undo finished.** 9 memories updated; 41 deeper ones weren't checked. This may not remove all traces.” |
| Deep sweep | “Best-effort semantic leakage path analysis enabled.” | “Want me to do an **extra deep sweep** for facts that might have traveled further?” |

### 29.5 Terms to avoid in the default agent flow

Avoid putting **canonical**, **ontology**, **RDF**, **OWL**, **SHACL**, **Datalog**, **SAT/SMT**, **DAG**, **provenance materialization**, **epistemic status**, **transitive fan-out**, **blast radius**, or **context-instance association** in novice prompts. Those may be appropriate in an engineer's diagnostic view, API, specification, or advanced audit details. A few words—**provenance** and **rollback**, especially—are appropriate when an expert explicitly asks for them, but should not be prerequisites for using Projects.

**Editorial decision to review later:** Whether to expose one playful sub-brand such as **Ripples** (Impact) or **Deep Sweep** (optional semantic scan). Prefer to test those in the clickable prototype rather than rename commands now. Keep *Night Shift* and *Blueprint* unless user testing reveals actual confusion.
