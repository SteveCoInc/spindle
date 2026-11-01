# References and verification status

Product authority: [uploaded v0.2 spec](../specs/project-spindle-spec-v0.2.md),
amended by the user's confirmed scope in [manual setup](manual-setup.md). The
uploaded file is a design document, not a statement of released AWS functionality.
Design recommendations in this plan are our proposals rather than source claims.

## Official sources inspected during planning

- [OpenAI Codex TypeScript SDK README](https://github.com/openai/codex/blob/main/sdk/typescript/README.md): retrieved from raw GitHub. Documents CLI wrapping, streamed events, threads, working directories, structured outputs and configuration overrides. Supports the proposed runner design; does not prove full interactive plugin capture or a specific released package's compatibility.
- [Anthropic Claude Agent SDK Python README](https://github.com/anthropics/claude-agent-sdk-python/blob/main/README.md): retrieved from raw GitHub. Documents SDK tools, MCP, hooks, clients and working directories. It also distinguishes allowed-tool auto-approval from removing tool capabilities. This is capability evidence; implementation may use the TypeScript SDK after checking its exact API parity.

These are moving `main` documents, not pinned implementation dependencies. M0 must
record the chosen released SDK/CLI versions and run the host capability tests.
No live agent or storage conformance tests were run for this planning change.

## Official references to verify during implementation

- [Neptune Analytics queries](https://docs.aws.amazon.com/neptune-analytics/latest/userguide/query.html)
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
distinction; a fresh official check remains an M0/M5 decision gate. No product
roadmap claim is made about a future AWS reasoner.

## Engineering judgments to validate

The modular monolith, transactional outbox, separate lineage authority, isolated
reviewer, trusted evaluation boundary, exact-commit CI evidence, and matched gym
comparisons are recommended design patterns. Their value here must be demonstrated
by failure-injection tests, actual host compatibility and gym measurements. This
research does not establish a universal best stack or validate unimplemented code.
