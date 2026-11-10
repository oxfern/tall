> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Rust

> Add iroh to your Rust project.

| Platform | Architectures |
| - | - |
| iOS | device and simulator |
| Android | aarch64, armv7 |
| macOS | arm64, x86\_64 |
| Linux | x86\_64, aarch64, armv7 (glibc and musl) |
| Windows | x86\_64, aarch64 |

* **API reference:** [docs.rs/iroh](https://docs.rs/iroh)
* **Example app:** [sendme](https://github.com/n0-computer/sendme), a CLI for direct file transfers between devices
* **All languages:** [platform support matrix](https://docs.iroh.computer/languages.md#platform-support)

iroh is written in Rust, so the Rust crate is the most fully-featured way to use it. Everything in these docs assumes Rust unless explicitly tagged otherwise.

## Install

Add iroh to your project:

```bash theme={null}
cargo add iroh
```

For tickets, common protocols, and async runtime support you'll usually want a few more:

```bash theme={null}
cargo add iroh-tickets iroh-ping anyhow
cargo add tokio --features full
```

## Hello, iroh

```rust theme={null}
use anyhow::Result;
use iroh::{Endpoint, endpoint::presets};

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = Endpoint::bind(presets::N0).await?;
    endpoint.online().await;
    println!("endpoint id: {}", endpoint.id());
    Ok(())
}
```

This binds an endpoint using the `N0` preset (public discovery + default relays), waits until it has a home relay, and prints its 64-character endpoint id. Run it with `cargo run`.

## Next steps

> [card] [Quickstart](https://docs.iroh.computer/quickstart.md)
>
> Run two endpoints and watch them ping each other in five minutes.

> [card] [Connect two endpoints](https://docs.iroh.computer/connect-two-endpoints.md)
>
> Build the sender/receiver flow from scratch with tickets and a custom protocol handler.

> [card] [Rust API docs](https://docs.rs/iroh)
>
> Full type and function reference on docs.rs.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.