# Local host tools

Build the binary with `cargo build --locked -p spindle-api`. Start `make serve`
and `make worker` in separate terminals. Provision a distinct identity/token for
each user using the [product runbook](../docs/runbooks/local-product.md).
The shared Rust stdio MCP server exposes project, membership, activation,
session, explicit capture, search, inspect and source retraction tools.

For Codex CLI, register the absolute path on your development machine:

```sh
codex mcp add spindle -- /absolute/path/to/spindle/target/debug/spindle-api mcp
```

Ensure `SPINDLE_TOKEN` is available to the launched MCP process using your host's
supported environment/secret configuration. Do not put tokens in Git or command
arguments. `SPINDLE_URL` defaults to `http://127.0.0.1:8080/v1`.

For Claude Code, use its stdio MCP registration with the same executable and `mcp`
argument. For Claude Agent SDK, configure a stdio `mcpServers.spindle` entry with
that command/argument and the user's token in the subprocess environment. Vendor
SDK code stays in its supported runtime; the server itself is Rust.

MCP transport is implemented; interactive Codex and Claude Code/SDK installation
and tool round-trips require separate real-host validation. This does not install
transcript hooks, silently capture sessions, or claim a persistent status-line UI.
Call `spindle_status` and `spindle_start_session` before sharing any evidence.
Only share text explicitly intended for all project members. Automatic transcript
capture remains disabled until privacy filtering and host spooling are qualified.
