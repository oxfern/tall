> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Add a relay

> Choose and configure production relays for your endpoints

By default, iroh endpoints use the public relays maintained by
[n0.computer](https://n0.computer) to facilitate connections when direct
peer-to-peer links aren't possible. The public relays are for development and
testing. We recommend that you do not use the public relays for
production workloads.

Use authenticated [Shared Relays](https://docs.iroh.computer/iroh-services/relays/shared.md) for growth and
development. If you need more control over what version of iroh relay is used,
no rate limits, and better uptime, use [dedicated
infrastructure](https://docs.iroh.computer/deployment/dedicated-infrastructure.md) instead.

## Configure your endpoint

Once you have one or more relay URLs, configure your endpoint to use them:

By default, your endpoint authenticates to Shared and Dedicated relays. The
iroh-services preset uses your project API key locally to mint a token bound to
the endpoint's Endpoint ID and scoped to relay use. The project API key is not
sent to the relay and does not expire automatically. Delete the key to revoke
relay access for endpoints using it. See [API
Keys](https://docs.iroh.computer/iroh-services/access.md#how-relay-authentication-works) for details.

```rust Rust theme={null}
use iroh::Endpoint;
use iroh_services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Build a preset pointing at your dedicated relays, authenticated with
    // your project's API key. In production, load the key from a config file
    // or environment variable instead of hardcoding it.
    let preset = iroh_services::preset()
        .relays([
            "YOUR_RELAY_URL_US",
            "YOUR_RELAY_URL_EU",
        ])?
        .api_secret_from_str("YOUR_API_KEY")?
        .build()?;

    // Bind the endpoint with the preset, then wait until it's online to
    // confirm it has an authorized connection to a relay.
    let endpoint = Endpoint::bind(preset).await?;
    endpoint.online().await;

    Ok(())
}
```

```python Python theme={null}
import asyncio
import iroh

async def main():
    # Build a preset pointing at your dedicated relays, authenticated with
    # your project's API key. In production, load the key from a config file
    # or environment variable instead of hardcoding it.
    preset = iroh.preset_iroh_services(
        iroh.ServicesPresetOptions(
            relays=["YOUR_RELAY_URL_US", "YOUR_RELAY_URL_EU"],
            api_secret="YOUR_API_KEY",
        )
    )

    # Bind the endpoint with the preset, then wait until it's online to
    # confirm it has an authorized connection to a relay.
    ep = await iroh.Endpoint.bind(iroh.EndpointOptions(preset=preset))
    await ep.online()

asyncio.run(main())
```

```swift Swift theme={null}
import IrohLib

// Build a preset pointing at your dedicated relays, authenticated with your
// project's API key. In production, load the key from a config file or
// environment variable instead of hardcoding it.
let preset = try presetIrohServices(options: ServicesPresetOptions(
    relays: ["YOUR_RELAY_URL_US", "YOUR_RELAY_URL_EU"],
    apiSecret: "YOUR_API_KEY"
))

// Bind the endpoint with the preset, then wait until it's online to confirm
// it has an authorized connection to a relay.
let ep = try await Endpoint.bind(options: EndpointOptions(preset: preset))
await ep.online()
```

```kotlin Kotlin theme={null}
import computer.iroh.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    // Build a preset pointing at your dedicated relays, authenticated with
    // your project's API key. In production, load the key from a config file
    // or environment variable instead of hardcoding it.
    val preset = presetIrohServices(
        ServicesPresetOptions(
            relays = listOf("YOUR_RELAY_URL_US", "YOUR_RELAY_URL_EU"),
            apiSecret = "YOUR_API_KEY",
        ),
    )

    // Bind the endpoint with the preset, then wait until it's online to
    // confirm it has an authorized connection to a relay.
    val ep = Endpoint.bind(EndpointOptions(preset = preset))
    ep.online()
    ep.shutdown()
}
```

```javascript JavaScript theme={null}
import { Endpoint, presetIrohServices } from '@number0/iroh'

// Apply a preset pointing at your dedicated relays, authenticated with your
// project's API key. In production, load the key from a config file or
// environment variable instead of hardcoding it.
const builder = Endpoint.builder()
presetIrohServices(builder, {
  relays: ['YOUR_RELAY_URL_US', 'YOUR_RELAY_URL_EU'],
  apiSecret: 'YOUR_API_KEY',
})

// In JavaScript, presets are functions applied to an `EndpointBuilder`, so use
// `Endpoint.builder()` rather than `Endpoint.bind()` — `bind()` always applies the
// n0 preset.

// Bind the endpoint, then wait until it's online to confirm it has an
// authorized connection to a relay.
const ep = await builder.bind()
await ep.online()
```

On a free project, omit the relay URLs. Then, the preset will use the n0 public relays instead.

Relays you self-host outside of Iroh Services are configured directly with `RelayMode::Custom`; see [Dedicated Infrastructure](https://docs.iroh.computer/deployment/dedicated-infrastructure.md).

## Why use your own relay?

Running dedicated relays gives you:

* **Isolation**: your traffic isn't mixed with other applications
* **Performance**: relays close to your users reduce latency and improve NAT traversal success
* **Capacity**: the shared public infrastructure is rate-limited
* **Redundancy**: distribute relays across regions or cloud providers for failover
* **Compliance**: keep relayed traffic inside your own network or jurisdiction

Iroh's relay architecture is uniquely suited to multi-relay deployments because
relays are stateless. Clients automatically fail over between relays in your
list, so adding capacity or surviving an outage is just a matter of running more
relay processes. See [Dedicated
Infrastructure](https://docs.iroh.computer/deployment/dedicated-infrastructure.md) for the deeper
architecture story.

## Get a relay

Choose Shared Relays for the standard Pro production setup. Choose dedicated
infrastructure when you need isolation, [version locking](https://docs.iroh.computer/iroh-services/relays/managed.md#version-locking), or blue/green deployments. [Iroh
Services](https://services.iroh.computer) can manage it for you, or you can self-host.

> [card] [Use Shared Relays](https://docs.iroh.computer/iroh-services/relays/shared.md)
>
> Use authenticated, multi-tenant production relays included with the Pro plan.

> [card] [Deploy a dedicated relay](https://services.iroh.computer?utm_source=docs\&utm_content=add-a-relay)
>
> Add single-tenant capacity, custom regions, and [version locking](https://docs.iroh.computer/iroh-services/relays/managed.md#version-locking) through Iroh Services.

> [card] [Self-host a relay](https://docs.iroh.computer/iroh-services/relays/self-hosted.md)
>
> Run the `iroh-relay` binary or image on a server with a public IP and DNS name. Automatic TLS via ACME is built in.

## Recommended setup

Place relays as close as possible to the users they serve. Shorter network paths
reduce latency and improve response times, including time to first byte when a
connection uses a relay.

For dedicated production deployments, also run at least two relays in different
geographic regions, for example one in North America and one in Europe. iroh
clients try multiple relays automatically, so if one becomes unreachable
they'll seamlessly fall back to another. For a globally distributed user base,
prioritize regions near your largest user populations while maintaining this
regional redundancy.

Each relay handles up to 60,000 concurrent connections. For larger deployments, run multiple relays per region or [contact us](https://cal.com/team/number-0/n0-protocol-services) to size up.

## Learn more

For the full architecture story (why stateless relays make uptime management
easier, how multiple relays provide resilience, and managed relay deployment
steps), see [Dedicated Infrastructure](https://docs.iroh.computer/deployment/dedicated-infrastructure.md).


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.