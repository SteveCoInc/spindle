# References and verification status

Product authority: [uploaded v0.2 spec](../specs/project-spindle-spec-v0.2.md),
amended by the user's confirmed scope in [manual setup](manual-setup.md). The
uploaded file is a design document, not a statement of released AWS functionality.
Design recommendations in this plan are our proposals rather than source claims.

## Official sources inspected during planning

- [OpenAI Codex TypeScript SDK README](https://github.com/openai/codex/blob/main/sdk/typescript/README.md): inspected in the original proposal for streamed events, threads and working directories. Historical capability evidence; the revised Rust design uses CLI/protocol boundaries or an external NeMo harness, not a TypeScript application.
- [Anthropic Claude Agent SDK Python README](https://github.com/anthropics/claude-agent-sdk-python/blob/main/README.md): documents tools, MCP, hooks, clients and working directories. It distinguishes tool auto-approval from removing capabilities. This informs the external integration boundary; Spindle-owned code remains Rust.
- [NeMo Gym assessment](gym-platform-evaluation.md): pinned source links for the Codex/Claude harnesses, Docker/Daytona providers, sandbox API and environment lifecycle at commit `0c7ac5119c99e0aa0dfeda7be59157c589680ffd`. Documentation/source inspection only, with concrete qualification gaps recorded.
- [Daytona old public core README](https://github.com/daytonaio/daytona/blob/main/README.md): inspected maintenance notice, hosted/API capabilities and SDK guidance. It states the public core is no longer maintained and points to the current organization. This limits the proposed self-hosting route; it is not a claim that the hosted service is discontinued.
- [AWS Aurora DSQL samples](https://github.com/aws-samples/aurora-dsql-samples/blob/main/README.md) and [Rust SQLx example](https://github.com/aws-samples/aurora-dsql-samples/blob/main/rust/sqlx/README.md): inspected official sample documentation for the SQLx connector, pooled IAM authentication and OCC retries. This establishes a documented Rust client route, not full compatibility with Spindle's future schema.
- [AWS DSQL booking example](https://github.com/aws-samples/aurora-dsql-samples/blob/main/deno/booking-api/postgres-js/README.md): inspected discussion of OCC, unique indexes, application integrity and a snapshot-isolation overlap race. Supports requiring explicit concurrency tests rather than equating PostgreSQL wire compatibility with invariant preservation.

These are moving `main` documents, not pinned implementation dependencies. M0 must
record the chosen released SDK/CLI versions and run the host capability tests.
No live agent or storage conformance tests were run for this planning change.

## Official references to verify during implementation

- [Aurora DSQL compatibility/limitations](https://docs.aws.amazon.com/aurora-dsql/latest/userguide/known-issues.html)
- [Aurora DSQL migration guidance](https://docs.aws.amazon.com/aurora-dsql/latest/userguide/working-with-postgresql-compatibility-migration-guide.html)
- [Aurora DSQL Rust SQLx connector](https://github.com/awslabs/aurora-dsql-connectors/tree/main/rust/sqlx)
- [Neptune Analytics queries (deferred graph phase)](https://docs.aws.amazon.com/neptune-analytics/latest/userguide/query.html)
- [Neptune Analytics RDF handling](https://docs.aws.amazon.com/neptune-analytics/latest/userguide/using-rdf-data.html)
- [Neptune Database](https://docs.aws.amazon.com/neptune/latest/userguide/)
- [OpenSearch Serverless overview](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-overview.html)
- [OpenSearch hybrid search](https://docs.opensearch.org/latest/vector-search/ai-search/hybrid-search/index/)
- [Apache Jena Fuseki](https://jena.apache.org/documentation/fuseki2/)
- [Neo4j Cypher manual](https://neo4j.com/docs/cypher-manual/current/)
- [Claude Code hooks](https://code.claude.com/docs/en/hooks)
- [Codex MCP](https://developers.openai.com/codex/mcp/)
- [Codex AGENTS.md guidance](https://developers.openai.com/codex/guides/agents-md/)
- [GitHub merge queues](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue)
- [GitHub merge_group workflow event](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#merge_group)
- [OpenTelemetry concepts](https://opentelemetry.io/docs/concepts/)
- [W3C PROV-O](https://www.w3.org/TR/prov-o/), [SPARQL 1.1](https://www.w3.org/TR/sparql11-overview/), [SHACL](https://www.w3.org/TR/shacl/)

AWS and GitHub documentation fetches were denied by the environment network policy.
The other links in this second list are references for implementation, not pages
verified in this session. The source spec supplies the Neptune Analytics/SPARQL
distinction; a fresh official check belongs to the deferred graph decision. DSQL
requires its own current compatibility check before AWS adoption. No product
roadmap claim is made about a future AWS reasoner.

## Engineering judgments to validate

The modular monolith, transactional outbox, separate lineage authority, isolated
reviewer, trusted evaluation boundary, exact-commit CI evidence, and matched gym
comparisons are recommended design patterns. Their value here must be demonstrated
by failure-injection tests, actual host compatibility and gym measurements. This
research does not establish a universal best stack or validate unimplemented code.
