# lib-ws-c

WebSocket client for the `concurrent-epoll` scheduler — libcurl
transport, RFC 6455 frames. Implements the `ws` contract
(`lib-ws`). Pure `.hnl`.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the client implementation |
| `src/ws-c.test.hnl` | the .hnl test suite |
| `pkg.hnl` | manifest — `modules: ["ws-c"]`, deps `ws`, `curl`, `libc` |
| `scripts/hnl-gate.sh` | the gate |
| `scripts/test_server.py` | test fixture server |
| `workflows/gate.yaml` | bupha entry (handl-gates) |

## Contract

- libcurl transport + RFC 6455 framing over the `ws` step surface;
  deps on the `curl`/`libc` codegen bindings.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
# bupha entry: workflows/gate.yaml (handl-gates campaign)
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
