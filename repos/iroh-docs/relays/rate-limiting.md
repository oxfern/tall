> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Rate Limiting

> How relays throttle traffic per connection, and how to configure limits on a self-hosted relay

## Why you're here

If an iroh endpoint just logged something like this:

```
The relay is rate-limiting this endpoint; outbound relay traffic is being
throttled. Send less data over the relay, or, if you operate this relay,
raise Limits::client_rx.
```

This means the relay has started throttling how fast it reads data from your connection. This is not an error.
This just means that the connection stays open and traffic keeps moving, just slower than usual.

Two ways to fix it:

* **Get a direct connection instead.** Relay rate limits only apply while traffic is relayed over a relay server. A successful direct P2P path bypasses the relay entirely. If an endpoint keeps falling back to the relay, [diagnose a direct connection](https://docs.iroh.computer/iroh-services/net-diagnostics/quickstart.md).
* **Use a relay with a higher limit.** [Shared relays](https://docs.iroh.computer/iroh-services/relays/shared.md) allow more throughput and higher rate limits, and [dedicated relays](https://docs.iroh.computer/iroh-services/relays/managed.md) remove rate limits entirely. If you run your own relay, [raise or remove its limit](#self-hosted-relays).

This notice is sent at most once per connection, and only to endpoints at iroh
version 1.0.4 or higher. Older clients are throttled the same way but won't see
the warning.

## Public relays

The [public relays](https://docs.iroh.computer/iroh-services/relays/public.md) n0.computer runs are free,
open to every iroh endpoint, and rate-limited to prevent abuse of shared
infrastructure.

The exact rate limits aren't published today, as they get tuned as needed. However, the
mechanism is applied per connection. If you are using the N0 preset, you're
sharing the relay's total capacity with every other tenant, so heavy traffic
from other users can affect your experience.

This means that public infrastructure is meant for development and hobby use. Do not rely on it for production workloads, especially if your application is sensitive to rate limiting, uptime, or relay software version.

## Shared and managed relays

[Shared relays](https://docs.iroh.computer/iroh-services/relays/shared.md) run on separate infrastructure
from the public relays, with far fewer tenants per machine and higher
per-connection limits.

[Dedicated relays](https://docs.iroh.computer/iroh-services/relays/managed.md) provisioned through Iroh
Services are dedicated to your project. These have no other tenants, no rate limits at
all.

## How it works

Rate limiting is a property of a single relay *connection*, not a global switch.

Each client connection gets its own token-bucket limiter on the side of the
relay that reads data sent by that client (`rx`, for "receive"):

* **`bytes_per_second`** — the steady-state rate the relay will keep reading from that connection.
* **`max_burst_bytes`** — how far a client can exceed that rate momentarily before the bucket runs dry.

When a client's bucket empties, the relay stops reading from that connection's
socket until it refills. Data isn't dropped. The client just experiences
backpressure. Writes to the relay stall until the bucket has room again. This is
what triggers the one-time `RateLimited` status notice.

Because the limiter is per-connection, one client hitting its limit doesn't affect other clients on the same relay.

## Self-hosted relays

If you run the `iroh-relay` binary (or embed the `iroh-relay` server crate)
yourself, rate limiting is **off by default**. This means connections can send
unlimited data.

With the `iroh-relay` binary, configure it in your TOML config file:

```toml theme={null}
[limits.client.rx]
bytes_per_second = 1_000_000
max_burst_bytes = 2_000_000
```

`bytes_per_second` is required to enable the limiter; `max_burst_bytes` is
optional and defaults to no burst allowance above the steady rate if omitted.

If you're embedding the relay server as a library, the equivalent is
`iroh_relay::server::Limits::client_rx`. This is the field the client-facing
warning message refers to when it says "raise `Limits::client_rx`".

> [!NOTE]
>
> `Limits` also has `accept_conn_limit` / `accept_conn_burst` fields for rate-limiting new incoming connections. As of this writing they're reserved but not yet enforced — setting them has no effect.

Consider a per-connection limit even on a single-tenant dedicated relay: it protects your egress bandwidth and CPU from a single misbehaving client or a bug in your own application, not just from other tenants.

## See also

* [Public Relays](https://docs.iroh.computer/iroh-services/relays/public.md)
* [Shared Relays](https://docs.iroh.computer/iroh-services/relays/shared.md)
* [Managed Relays](https://docs.iroh.computer/iroh-services/relays/managed.md)
* [Dedicated Infrastructure](https://docs.iroh.computer/deployment/dedicated-infrastructure.md)
* [Relay source (`iroh-relay`)](https://github.com/n0-computer/iroh/tree/main/iroh-relay)


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.