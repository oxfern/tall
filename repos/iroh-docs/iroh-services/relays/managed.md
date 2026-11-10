> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Dedicated Hosting

> Deploy and configure dedicated relay infrastructure through Iroh Services

Managed relays are dedicated relay servers provisioned through the Iroh Services platform and operated by n0.computer. Unlike the [public relays](https://docs.iroh.computer/iroh-services/relays/public.md), managed relays are exclusively yours: authenticated by default, with no shared traffic or rate limits.

## What you get

Each Dedicated relay is a **\$199/month per-region add-on** to the Pro plan and
includes:

* **Fully managed**: n0.computer handles operations, maintenance, and upgrades
* **Authenticated by default**: only your project's endpoints can connect, using an endpoint-bound token derived locally from your API key
* **Isolation**: your traffic only; no noisy neighbors
* **Capacity**: up to 60,000 concurrent connections
* **Egress**: 250 GB per month, then \$0.09/GB
* **[Version locking](#version-locking)**: pin to a specific iroh version or run blue/green deployments
* **Multi-region**: deploy across multiple regions for lowest latency and high availability. [Contact us](mailto:support@iroh.computer) to request a custom region if the available regions don't meet your needs.

Multi-cloud, on-premises deployments, and SLAs are available on Enterprise
plans. [Contact us](mailto:support@iroh.computer) to discuss.

Only traffic carried by a relay counts as relay egress. Direct peer-to-peer
traffic does not.

## Deploy a relay

### 1. Sign up and create a Pro project

Sign up at [services.iroh.computer](https://services.iroh.computer?utm_source=docs\&utm_content=managed-relays) and create a project if you haven't already.

The project must be on the Pro plan to deploy managed relays. You can also upgrade your project to Pro in the billing settings.

![Create a project](https://mintcdn.com/number0/fCc3-LAMKNsA0nlG/images/create-project.png?fit=max\&auto=format\&n=fCc3-LAMKNsA0nlG\&q=85\&s=fc8d970d532011d09d5bab555159ee98)

### 2. Add a relay

Navigate to **Relays** in your project sidebar and click **Deploy Relay**. Select a region for your relay.

![Add relay dialog](https://mintcdn.com/number0/fCc3-LAMKNsA0nlG/images/add-relay.png?fit=max\&auto=format\&n=fCc3-LAMKNsA0nlG\&q=85\&s=10e5946df3291cd63c774bb250c1f549)

### 3. Copy your relay URL

Once deployed, your relay URL will appear in the dashboard. Hit Connect to see a code snippet to connect to your relay.

### 4. Configure your endpoint

Set your endpoint to use your dedicated relay URLs instead of the public relays:

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

### 5. Verify connectivity

Go to your project's **Relays** page to confirm your endpoints are connecting
through your dedicated relay. You should see connected endpoint counts and
traffic metrics.

![Relay metrics](https://mintcdn.com/number0/fCc3-LAMKNsA0nlG/images/metrics-relay.png?fit=max\&auto=format\&n=fCc3-LAMKNsA0nlG\&q=85\&s=758b23e5d682e1e6f2ed541eab2cd158)

## Authentication

Managed relays **require authentication by default** — only your project's
endpoints can connect. There's nothing extra to set up: when you build your
endpoint with the [`preset()`](#4-configure-your-endpoint), the SDK derives an
endpoint-bound relay token locally from your project API key. See [API
Keys](https://docs.iroh.computer/iroh-services/access.md#how-relay-authentication-works) for the authoritative
authentication description.

## Version locking

Dedicated relays can be pinned to a specific supported iroh relay version
instead of moving automatically to a newer release. This lets you coordinate
relay upgrades with your endpoint rollout and operate old and new relay versions
side by side during a blue/green deployment. The pin applies to the relay
software; you remain responsible for verifying compatibility with the iroh
versions used by your endpoints.

## Turn authentication off

If you want any endpoint that has your relay's URL to be able to use it — not
just your project's endpoints — you can turn authentication off in the **Authentication** section.

![Relay access control settings](https://mintcdn.com/number0/fCc3-LAMKNsA0nlG/images/relay-auth-toggle.png?fit=max\&auto=format\&n=fCc3-LAMKNsA0nlG\&q=85\&s=cbd1449fae005492a83f9cd35a97665a)

Turning authentication off doesn't list or advertise your relay anywhere; an
endpoint still has to know the URL to use it. The relay also keeps reporting its
traffic and metrics to your dashboard either way.

Be careful turning authentication off: any iroh endpoint that learns the URL can
then use the relay, so it may be subject to abuse. We recommend keeping
authentication on for production deployments and only turning it off for testing
or development.

Public relays still report their traffic and metrics to your dashboard; they
just don't require your API key to connect.

## Recommended setup

Place relays as close as possible to the users they serve. Shorter network paths
reduce latency and improve response times, including time to first byte when a
connection uses a relay. For a globally distributed user base, prioritize
regions near your largest user populations.

For production, also deploy at least two relays in different geographic
regions. If one relay becomes unreachable, iroh automatically falls back to the
next one in your list, so your peers will still find each other. Choose regions
that balance user proximity with protection from a single regional outage.

Each relay handles up to 60,000 concurrent connections. For larger deployments,
[contact us](mailto:support@iroh.computer) to increase relay capacity.

## Request a custom region

Need a relay closer to your users or in a region that is not available in the
dashboard? Contact [support@iroh.computer](mailto:support@iroh.computer) with:

* The region you need
* Your expected peak concurrent connections
* Your expected monthly relayed egress

We review each request for availability and quote you on the expected monthly
price. Depending on the region, provisioning may be available immediately or may
require additional provider and capacity review. We will confirm availability,
expected provisioning time, and pricing before provisioning the relay. Pricing
for the custom region may differ from the standard \$199/month per-region add-on
for managed relays, and may come with different egress allowances and overage rates.

## Support

Relay status and metrics are available in your project dashboard under
**Relays**. On the Pro plan, we offer email support at [support@iroh.computer](mailto:support@iroh.computer). [Contact
us](mailto:support@iroh.computer) for Enterprise Support SLAs.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.