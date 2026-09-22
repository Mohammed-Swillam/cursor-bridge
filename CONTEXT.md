# cursor-bridge

A single command that runs Claude Code against a Cursor subscription by standing in for the Anthropic API for the length of one session.

## Language

**Bridge**:
The `cursor-bridge` process as a whole: it owns one session from start to exit.
_Avoid_: wrapper, launcher

**Proxy**:
The Bridge's local HTTP endpoint that speaks the Anthropic Messages API to Claude Code.
_Avoid_: server, daemon, gateway

**Cursor agent**:
The authenticated Cursor `agent` CLI that the Proxy runs to produce each reply.
_Avoid_: backend, Cursor CLI, worker

**Bridge token**:
The per-session secret shared only between the Bridge and the Claude Code it spawned; any caller without it is a stranger.
_Avoid_: API key, auth token, password

**User-facing error**:
A failure the Bridge reports on the terminal before Claude Code takes it over, always shown.
_Avoid_: fatal log, startup log

**Debug log**:
Diagnostic output shown only when `CURSOR_BRIDGE_DEBUG` is on.
_Avoid_: bridge log, verbose output
