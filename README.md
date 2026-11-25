# lib-ws-c

WebSocket client for the `concurrent-epoll` scheduler — libcurl transport, RFC 6455 frames.

## Layout

```text
src/        the .hnl sources
pkg.hnl     package manifest — modules, deps, description, license
scripts/hnl-gate.sh   the gate
scripts/test_server.py   test fixture server
workflows/gate.yaml       bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
