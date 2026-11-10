> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Overview

By default, iroh will use public shared infrastructure to facilitate connections over
address lookup and end-to-end encryption over relays. This infrastructure comprises:

1. [Relays](https://docs.iroh.computer/concepts/relays.md)
2. [Address Lookup](https://docs.iroh.computer/concepts/address-lookup.md)

Relays forward traffic when direct connections are not possible as well
as facilitates NAT traversal for direct connections. These servers are managed and
maintained by [n0.computer](https://n0.computer), and are shared by a global public network of
developers.

We recommend using the public relays for development and testing, as they are
free to use and require no setup. For most production systems, use [Shared
Relays](https://docs.iroh.computer/iroh-services/relays/shared.md), the multi-tenant relay option included
with the Pro plan. Use dedicated relays when you need single-tenant capacity,
custom regions, or [version locking](https://docs.iroh.computer/iroh-services/relays/managed.md#version-locking). Use an Enterprise plan when you need an
SLA.

> [card] [Use Shared Relays](https://docs.iroh.computer/iroh-services/relays/shared.md)
>
> Use authenticated, multi-tenant relays for the standard Pro production setup.

> [card] [Deploy a dedicated relay](https://services.iroh.computer?utm_source=docs\&utm_content=concepts-relays)
>
> Sign up for Iroh Services and spin up a managed relay for your project in minutes.

> [card] [Self-host a relay](https://docs.iroh.computer/iroh-services/relays/self-hosted.md)
>
> Learn how to self-host a relay for your project.

## Using your own relays

To use a specific set of relays with your iroh endpoint, configure your relay URLs as part of an iroh-services preset:

```rust Rust theme={null}
use iroh::{Endpoint, RelayMap, RelayMode, RelayUrl, endpoint::presets};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let relay_url1: RelayUrl = "YOUR_RELAY_URL_US".parse()?;
    let relay_url2: RelayUrl = "YOUR_RELAY_URL_EU".parse()?;

    let endpoint = Endpoint::builder(presets::N0)
        .relay_mode(RelayMode::Custom(RelayMap::from_iter([
            relay_url1,
            relay_url2,
        ])))
        .bind()
        .await?;

    Ok(())
}
```

```python Python theme={null}
import asyncio
import iroh

async def main():
    relay_mode = iroh.RelayMode.custom_from_urls([
        "YOUR_RELAY_URL_US",
        "YOUR_RELAY_URL_EU",
    ])
    ep = await iroh.Endpoint.bind(
        iroh.EndpointOptions(preset=iroh.preset_n0(), relay_mode=relay_mode)
    )

asyncio.run(main())
```

```swift Swift theme={null}
import IrohLib

let relayMode = try RelayMode.customFromUrls(urls: [
    "YOUR_RELAY_URL_US",
    "YOUR_RELAY_URL_EU",
])
let ep = try await Endpoint.bind(options: EndpointOptions(
    preset: presetN0(),
    relayMode: relayMode
))
```

```kotlin Kotlin theme={null}
import computer.iroh.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val relayMode = RelayMode.customFromUrls(listOf(
        "YOUR_RELAY_URL_US",
        "YOUR_RELAY_URL_EU",
    ))
    val ep = Endpoint.bind(
        EndpointOptions(preset = presetN0(), relayMode = relayMode),
    )
    ep.shutdown()
}
```

Managed relays from Iroh Services **supply authentication by default**. The SDK
uses your project API key locally to create an endpoint-bound relay token; it
does not send the API key to the relay. For that flow and full deployment steps,
see the [managed relay guide](https://docs.iroh.computer/iroh-services/relays/managed.md).

## Recommended setup

Place relays as close as possible to the users they serve. Shorter network paths
reduce latency and improve response times, including time to first byte when a
connection uses a relay. For a globally distributed user base, prioritize
regions near your largest user populations.

For redundancy, run at least two relays in different regions. This placement
should complement proximity: choose regions that keep relays near users while
avoiding a single regional point of failure.

## Why this architecture is powerful

This approach makes uptime management significantly easier compared to
traditional client-server architectures:

**Stateless servers, stateful clients**\
Unlike traditional servers that store your application's data and state, relay
servers are just connection facilitators. All your business logic and data lives
in your clients. This means:

* **No database synchronization** - You don't need to worry about keeping multiple server databases in sync or handling data replication
* **No state migration** - When a relay goes down, clients simply reconnect to another relay without any data loss or state transfer
* **Simple server management** - Relay servers are lightweight and easy to spin up or down. No complex deployment procedures or data migration steps

**Automatic failover**\
iroh clients automatically try multiple relays when connecting. If one relay is unavailable, clients seamlessly fall back to another relay in your list without application-level retry logic. Your peers will find each other as long as at least one relay is reachable.

**Multi-cloud resilience**\
Enterprise plans can distribute relays across multiple cloud providers. If one
provider experiences an outage, your application keeps running on relays hosted
elsewhere. Since relays don't store state, you can mix providers without
worrying about cross-cloud data consistency.

**Cost-effective scaling**\
Adding capacity means spinning up more lightweight relay instances, not provisioning databases or managing complex stateful server infrastructure. You can easily scale up for peak usage and scale down during quiet periods.

This architecture inverts the traditional model: instead of treating servers as precious stateful resources and clients as disposable, relay-based architectures treat relays as disposable connection facilitators while clients own the application state and logic.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.