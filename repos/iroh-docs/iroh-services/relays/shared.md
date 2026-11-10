> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Shared Relays

> Multi-tenant relays with authentication, higher rate limits, and predictable performance

Shared relays are multi-tenant relays run by n0.computer on infrastructure that
is separate from the [public relays](https://docs.iroh.computer/iroh-services/relays/public.md). They sit
between the free public relays and fully [dedicated
relays](https://docs.iroh.computer/iroh-services/relays/managed.md): you share a box with a small number of
other tenants instead of the entire internet.

## Why not the public relays

The public relays are rate-limited based on total traffic across every iroh user on them. When traffic is high or bursty, your application shares those limits with everyone else.

Shared relays:

* Run on separate infrastructure from the public relays, so they aren't affected by bursty traffic or DDoS attacks against them
* Have far fewer tenants per machine
* Have higher rate limits than the public relays
* Optional API key authentication, so only your applications can connect
* Provide more predictable performance

## What you get

For **\$19/month**:

* Four relays across the globe
* Up to 10,000 concurrent connections, then \$0.003 per additional endpoint per month
* 100 GB of relay egress per month, then \$0.09/GB
* A 5 MB/s rate limit
* API key authentication, so you can revoke access for endpoints as needed

Only traffic carried by a relay counts as relay egress. Direct peer-to-peer
traffic does not.

Authentication uses an endpoint-bound capability token signed locally with your
project API key. See [API Keys](https://docs.iroh.computer/iroh-services/access.md#how-relay-authentication-works)
for how tokens and project-wide credentials differ.

Metrics are already included with the Pro plan: throughput, direct connection rates, relay usage, and rate limits. See [Metrics](https://docs.iroh.computer/iroh-services/metrics/index.md).

## Free trial

The Pro plan includes a 30-day free trial.

## How to get shared relays

Sign up at [services.iroh.computer](https://services.iroh.computer?utm_source=docs\&utm_content=shared-relays)
and create a Pro project, or upgrade an existing project to Pro. Shared relays
are included with the plan.

## Configure your endpoint

Click **Connect** in your project or in the relays section to get the
configuration for your endpoints.

![Relay code snippet](https://mintcdn.com/number0/fCc3-LAMKNsA0nlG/images/relay-code-snippet.png?fit=max\&auto=format\&n=fCc3-LAMKNsA0nlG\&q=85\&s=c6b3d8c596283a1aecabe05596258922)

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

## Shared vs. Dedicated

Shared relays give you authentication, isolation from public traffic, and higher
rate limits at a low fixed price. Choose [Dedicated
Relays](https://docs.iroh.computer/iroh-services/relays/managed.md) when you need single-tenant isolation,
[version locking](https://docs.iroh.computer/iroh-services/relays/managed.md#version-locking), blue/green deployments, or custom regions. Choose Enterprise
when you need an SLA or multi-cloud deployment.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.