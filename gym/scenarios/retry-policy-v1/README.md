# Retry-policy transfer seed

This is a real, deliberately defective Rust payment-client policy library for the
first multi-persona episode. Only `seed/` goes into each participant workspace.
The controller owns `scenario.toml`, persona-private context, and `grader/`.
Sam must not receive Avery's private facts through a shared file or grader mount.
An identical seed/publication schedule is required in enabled and disabled arms.

The behavioral oracle is three independent tests. The seed must fail the
legacy-without-key test while passing the other two. This baseline failure is
expected and is not a failure of Spindle's ordinary CI. A live run is successful
only when the controller checks the actual agent-produced source with the external
oracle, retains observable trajectories, and confirms cleanup and budget status.

The scenario is development data, not a secret held-out evaluation. Factory agents
can read it in this repository. A protected holdout needs the separate trusted
storage and execution boundary described in the factory plan.

Framework contract tests and local sandbox isolation have run, but this scenario
has not yet run through actual Codex personas or production capture hooks. The
TOML records cost/time ceilings; it does not implement budget enforcement.

Run `make gym-baseline` to verify the seed's expected two passes and one behavioral
failure. The Rust test runs only this trusted repository fixture in a temporary
checkout. It must not be reused to execute untrusted agent build scripts on the
controller host; candidate grading needs an isolated, credential-free sandbox.
