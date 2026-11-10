# tall

One channel of video chat, tiled like xmonad. Read [README.md](README.md)
first; it is the description of the system.

- `core/`: pure Rust shared by both sides. `layout.rs` is the xmonad port
  (keep names and arithmetic aligned with `repos/xmonad/src/XMonad/Layout.hs`
  and `StackSet.hs`); `proto.rs` is the wire format and its documentation.
- `server/`: the one process (`tall`): iroh relay, hub, static files.
- `web/`: the browser client, Rust compiled to WebAssembly. The page is one
  canvas drawn with WebGPU. Do not add DOM elements, CSS interface, a
  framework or hand-written JavaScript; browser APIs are called through
  `web/src/js.rs`.
- `repos/`: vendored reference source, listed in `repos/sources.json`. Read
  it before guessing an API; never build, import or edit it.
- `.agents/skills/`: the Fly.io skills (`fly`, `brand`) from superfly/skills.
  Load `fly` before any Fly.io operation.

iroh is pinned to 1.4.1 in the browser, the hub and the relay; bump them
together. Only this deployment's own relay is used: no public relays, no
address lookup services.

Check with `cargo test -p tall-core`, `cargo build -p tall-server` and
`./build.sh dev`, then open two tabs on `http://localhost:8080`.

The server keeps the relay's clients and the roster in memory: one Fly
machine, `fly deploy --ha=false`.
