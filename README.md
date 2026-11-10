# tall

A video channel that tiles like [xmonad](https://xmonad.org). Open the page
and you are in the channel with whoever else has it open: up to seven people,
720p video and Opus audio, sent peer to peer over [iroh](https://iroh.computer).

The page is one `<canvas>`. Everything on it (the tiles, the borders, the
bar) is drawn with WebGPU by a Rust program compiled to WebAssembly. There is
no DOM interface, no framework and no JavaScript beyond the generated
WebAssembly loader.

## Using it

| Key | Tap | xmonad action |
| --- | --- | --- |
| `j` / `k` | a tile | focus next / previous |
| `Enter` | the focused tile | swap the focused tile with the master |
| `J` / `K` | | move the focused tile down / up |
| `h` / `l` | | shrink / grow the master pane |
| `,` / `.` | | more / fewer tiles in the master pane |
| `Space` | `[Tall]` in the bar | next layout: Tall, Mirror Tall, Full |
| `m` | `mic` in the bar | mute or unmute |
| `v` | `cam` in the bar | camera off (released) or on |

A red border is the focused tile, a green one is whoever is speaking, a red
square marks a muted microphone, and a coloured square stands in for a camera
that is off.

It needs a browser with WebGPU, WebCodecs and `MediaStreamTrackProcessor` on
the main thread: Chrome or Edge on desktop and Android. Safari and Firefox
do not have all three yet.

## How it works

```
browser A ──┐                         ┌── browser B
  WASM      │   WebSocket (relay)     │     WASM
  iroh  ────┼──▶  tall server  ◀──────┼──── iroh
endpoint    │   relay · hub · files   │   endpoint
            └── end-to-end QUIC ──────┘
```

- **One server process** ([server/](server/src/main.rs)) is an iroh relay, the
  channel hub and the file server. iroh-relay's HTTP service owns the port; the
  page's three files are extra handlers on it. The hub is an iroh endpoint in
  the same process that tells members each other's endpoint ids.
- **Peers talk directly.** Every two members hold one iroh connection: QUIC,
  encrypted end to end between keys generated in each tab. Browsers cannot
  open UDP sockets, so the packets travel through the relay, which only sees
  ciphertext.
- **Video** is captured as `VideoFrame`s, encoded once with WebCodecs (H.264,
  else VP9 or VP8) and written to every peer as one QUIC stream per two-second
  group of pictures. A receiver plays the newest group and stops older
  streams, and a sender drops a peer's backlog and asks the encoder for a key
  frame, so a slow link skips ahead instead of falling behind. Decoded frames
  are sampled by the GPU as external textures, without a copy.
- **Audio** is Opus in QUIC datagrams, 20 ms each, never retransmitted,
  scheduled on an `AudioContext` behind an 80 ms cushion.
- **Tiling** is xmonad's: [core/src/layout.rs](core/src/layout.rs) ports
  `Tall`, `Mirror`, `Full` and the stack operations from the Haskell source
  in [repos/xmonad](repos/xmonad/src/XMonad/Layout.hs), with the same integer
  arithmetic.

The wire formats are documented in [core/src/proto.rs](core/src/proto.rs).

Each member uploads its video once per peer: 4 Mbit/s each for up to three
peers, 2.5 Mbit/s each (15 Mbit/s in all) in a full channel of seven.

## Running it

Rust 1.91 with the `wasm32-unknown-unknown` target (`rust-toolchain.toml`
selects both under rustup) and `wasm-bindgen-cli` 0.2.129:

```sh
cargo install wasm-bindgen-cli --version 0.2.129 --locked
./build.sh dev                 # the client, into web/dist
cargo run -p tall-server       # http://localhost:8080
cargo test -p tall-core        # the layout and wire-format tests
```

Cameras need a secure context: `localhost` or HTTPS.

## Deploying to Fly.io

```sh
fly launch --copy-config --no-deploy --name <your-app>   # once
fly deploy --ha=false
```

Run exactly one machine: the relay and the roster live in the process's
memory. `fly.toml` stops it when nobody is connected; the next visitor
waits a second or two for it to start.

## Vendored reference

[repos/](repos/sources.json) holds source that this was written against,
for reading only: xmonad (the algorithm), the iroh documentation and browser
examples, and the code of [rauchg/vinyl](https://github.com/rauchg/vinyl)
(a page that is a GPU surface with no dependencies). The Fly.io agent skills
from [superfly/skills](https://github.com/superfly/skills) are in
`.agents/skills`.

## Licence

MIT. Vendored sources keep their own licences.
