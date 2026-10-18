# lib-ws-c

WebSocket client for the `concurrent-epoll` scheduler — libcurl transport, RFC 6455 frames.

Part of the `handl-lang` org split (ADR-0128). History is a `git subtree
split` of the platform monorepo's `ws-c/` (via `c`).

## Layout

```text
src/        the .hnl sources
pkg.hnl     dependencies, when any: `{ deps: { <name>: "branch:<ref>" } }`
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```
