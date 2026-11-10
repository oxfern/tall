> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# DHT

DHT address lookup publishes and resolves endpoint records on the
[BitTorrent Mainline](https://en.wikipedia.org/wiki/Mainline_DHT) distributed hash
table. The records are the same signed records that
[DNS address lookup](https://docs.iroh.computer/connecting/dns-address-lookup.md) uses. The difference is where they
are stored. DNS address lookup publishes them to a hosted server and resolves over DNS,
while DHT address lookup puts them on the BitTorrent Mainline DHT. That removes the
dependency on a hosted server: any endpoint can publish and resolve without a
central party, at the cost of slower lookups than DNS.

![Bob publishes his signed record to several nodes of the Mainline DHT; Alice resolves it by querying several nodes](https://www.iroh.computer/animations/publish-relay-dht.svg)

DHT address lookup is not enabled by default. It lives in the separate
[`iroh-mainline-address-lookup`](https://crates.io/crates/iroh-mainline-address-lookup)
crate, which you add alongside `iroh`.

```toml theme={null}
[dependencies]
iroh = "1"
iroh-mainline-address-lookup = "0.4"
```

Add the `DhtAddressLookup` to the endpoint with `Endpoint::builder`. You can run it
next to the default DNS address lookup, or build from `presets::Minimal` to use only the
DHT.

```rust theme={null}
use iroh::{Endpoint, endpoint::presets};
use iroh_mainline_address_lookup::DhtAddressLookup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let endpoint = Endpoint::builder(presets::N0)
        .address_lookup(DhtAddressLookup::builder())
        .bind()
        .await?;
    // your code here
    Ok(())
}
```

DHT address lookup also combines well with [mDNS address lookup](https://docs.iroh.computer/connecting/local-address-lookup.md)
for both global and local address lookup without depending on centralized infrastructure.

For the record format shared with DNS address lookup, see the
[Address Lookup concept page](https://docs.iroh.computer/concepts/address-lookup.md).


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.