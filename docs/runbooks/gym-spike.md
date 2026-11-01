# NeMo Gym qualification spike

Pinned upstream: `NVIDIA-NeMo/Gym@0c7ac5119c99e0aa0dfeda7be59157c589680ffd`.
The runtime and lock digest are recorded in [gym/nemo-pin.toml](../../gym/nemo-pin.toml).
Spindle-owned application code remains Rust. This spike executes the external
framework's Python tests/API; it does not introduce a Python application adapter.

```sh
make gym-bootstrap
make gym-contracts
```

`uv 0.12.23` was used with Python 3.13.14 and upstream `uv.lock`, frozen dev extras,
no default dependency groups. A preexisting checkout can be selected with
`NEMO_GYM_DIR=/absolute/path/to/nemo-gym`. In this environment it is
`/workspace/tooling/nemo-gym`. `gym-contracts` records JUnit in ignored artifacts.
The initial direct invocation of these four suites passed 332 tests, skipped one
Python 3.8 worker test (interpreter unavailable), and emitted one warning.
These are upstream fixture-provider supervisor/session/bootstrap tests, not paid
agent runs or a product-benefit measurement.

A separate live Docker provider diagnostic used the upstream documented
`Sandbox`/`SandboxSpec` API with two personas and the already pinned PostgreSQL
image as a small shell-capable container. It verified distinct private seed files,
artifact download, absence of the Docker socket, and container removal on context
exit. Configuration: `network=none`, dropped all capabilities,
`no-new-privileges`, PID limit 64, one CPU, 256 MiB, 120-second TTL. No model calls,
external sandbox allocations or credentials were involved.

The first diagnostic exposed a real integration detail: `docker cp` preserved
0600 source-file ownership, so root with all capabilities dropped could not read
the controller-owned seed. Running task commands with the controller's unprivileged
UID and a writable `/tmp` workdir passed, while retaining all isolation settings.
A production agent image must instead establish its own explicit task UID and
matching seed ownership; do not assume root bypasses permissions after dropping
capabilities. This is not yet a validated coding-agent image.

## Boundary still to qualify

The pinned NeMo Codex harness defaults to CLI 0.144.4, while the factory environment
has 0.163.0. Do not silently mix them. The harness expects a sandbox-reachable Gym
Model Server with Responses API compatibility and configured model limits; the
current interactive Codex login is not automatically a compatible Model Server
credential. No such model-server profile or API credential is configured here.

Remaining: build a task image; expose authenticated Spindle tools through the
supported MCP grants; supply a qualified hosted model transport and price/usage
accounting; run two actual personas against retained shared project state; capture
trajectories through the product integration; verify independent grading and
cleanup on failures; then compare matched enabled/disabled arms. The user's
$100/four-hour ceiling remains the maximum full run. Unknown pricing/usage must
not be labeled a hard-enforced dollar budget. Daytona remains optional and unrun.
