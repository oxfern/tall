> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# What is iroh?

iroh is a modular networking stack written in Rust. It provides the building
blocks to create applications that can communicate using fast, cheap, and
reliable connections.

It's a lightweight native library meant to be embedded directly into your
application — in Rust, or in C, C++, Swift, Python, JavaScript, and Kotlin
through [our bindings](https://docs.iroh.computer/languages.md).

![iroh embedded across six device contexts: iOS, Android, desktop, browser, server, embedded](https://www.iroh.computer/animations/embedding-phone.svg)

## Core features

* **Fast**: iroh enables direct connections between
  devices, allowing them to communicate without relying on centralized servers.
* **Reliable**: iroh is designed to work in challenging network conditions.  It
  uses relay servers as a fallback when direct connections are not possible.
* **Secure**: All connections established through iroh are authenticated and
  encrypted end-to-end using the QUIC protocol, ensuring data privacy and
  integrity.
* **Modular**: iroh is built around a system of composable protocols
  that can be mixed and matched to suit the needs of different applications. This
  allows developers to easily add functionality such as file sharing, messaging,
  and real-time collaboration.

## Use cases

* **Real-time communication**: Build [chat applications](https://docs.iroh.computer/examples/chat.md), [RPC
  services](https://docs.iroh.computer/protocols/rpc.md), and [streaming data](https://docs.iroh.computer/protocols/streaming.md) with
  iroh's communication protocols.
* **Files & blobs**: With protocols like [iroh-blobs](https://docs.iroh.computer/protocols/blobs.md), iroh enables efficient
  file transfer.
* **Local-first, offline-first, peer-to-peer applications**: iroh provides the networking foundation for
  building applications that can operate without reliance on servers.
* **Structured data**: iroh's support for flexible data protocols like
  [Documents](https://docs.iroh.computer/protocols/documents.md) and [Automerge](https://docs.iroh.computer/protocols/automerge.md) allows
  developers to build applications that support real-time collaborative editing
  and data synchronization. Any kind of CRDT or OT sync protocol can be integrated.

## How iroh works

iroh's job comes down to two promises: you can reach a peer no matter where it
is, and the connection you get is the best one available.

### Connect by key

Every iroh [endpoint](https://docs.iroh.computer/concepts/endpoints.md) is identified by a cryptographic key.
That key — not an IP address — is the address. It lets you connect to a peer no
matter where it is, even as its network location changes underneath it.

![Alice connects to Bob by key — IPs change, the key stays stable](https://www.iroh.computer/animations/connect-by-key.svg)

### Reliable connections

We make sure you get a connection whenever possible, and that it's as fast as
possible. When network conditions change, iroh immediately reacts and switches
to the new best path — transparently, without dropping the connection.

![Alice and Bob connected via a shared router, with two more routers available for when Bob moves](https://www.iroh.computer/animations/routing-moves.svg)

Now that we've seen *what* iroh does, here's *how*.

### Library and relays

There are two pieces to the system:

* **The iroh library** runs inside your application. It provides the API to
  create endpoints, connect to other endpoints, and open connections and data
  streams.
* **[Relays](https://docs.iroh.computer/concepts/relays.md)** run in the background on publicly reachable
  servers. They help endpoints find each other and establish direct
  connections, and relay data as a fallback when a direct connection isn't
  possible.

### Endpoint startup

An iroh endpoint is configured with a set of relays — the rate-limited public
relays operated by number 0, relays from iroh services, or self-hosted ones.
iroh works fine with a single relay, but works best with relays in every region
where your users are.

On startup, an endpoint sends
[QAD](https://datatracker.ietf.org/doc/draft-ietf-quic-address-discovery/)
probes to all configured relays. This does two things: it learns the endpoint's
own public IP address, and it learns which relay is closest by latency. The
closest relay becomes the **home relay**, and the endpoint keeps a secure
WebSocket connection open to it.

![A map of the US and Europe with iroh relays in us-west (Seattle), us-east (Delaware), and eu-west (Frankfurt), and an endpoint in Florida](https://www.iroh.computer/animations/endpoint-startup.svg)

### Finding peers

When there are multiple relays, Alice needs to know which relay to use to reach
Bob. iroh offers two ways to publish and look up that information.

#### DNS-based lookup

Out of the box, iroh uses DNS. Bob publishes a signed record with his home relay
to a DNS server operated by number 0 via an HTTPS `PUT`. Anyone who wants to
reach Bob resolves that record with a DNS or HTTPS query. See
[DNS address lookup](https://docs.iroh.computer/connecting/dns-address-lookup.md).

![Bob publishes a signed DNS record with his home relay to dns.iroh.link via an HTTPS PUT; Alice resolves it with a DNS lookup](https://www.iroh.computer/animations/publish-relay.svg)

#### Mainline DHT lookup

DNS works in most cases, but sometimes you want something fully peer-to-peer.
For that, iroh offers an optional lookup over the Mainline DHT: it publishes the
exact same signed record using the
[BEP 44](https://www.bittorrent.org/beps/bep_0044.html) extension. See
[DHT address lookup](https://docs.iroh.computer/connecting/dht-address-lookup.md).

![Bob publishes his signed record to several nodes of the Mainline DHT; Alice resolves it by querying several nodes](https://www.iroh.computer/animations/publish-relay-dht.svg)

### Direct connections

So far we've described a system where endpoints can always talk — but all data
flows through the relays. That adds latency, limits throughput, and costs the
relay operator money. Making connections fast and direct is arguably the most
complex part of the system.

#### Hole punching

Hole punching happens *inside* the QUIC connection via an
`n0_nat_traversal` extension, inspired by the
[QUIC NAT traversal draft](https://datatracker.ietf.org/doc/draft-seemann-quic-nat-traversal/)
but using its own transport parameter ID. See
[NAT traversal](https://docs.iroh.computer/concepts/nat-traversal.md).

![Alice and Bob, each behind a home router, connected through the relay — all data flows up to the relay and back down](https://www.iroh.computer/animations/hole-punching.svg)

#### Local connections

If two devices are on the same network, hole punching isn't needed — each side
just needs to learn the other's local address. See
[local address lookup](https://docs.iroh.computer/connecting/local-address-lookup.md).

![Alice and Bob on the same home network discover each other over the relay, then connect directly across the LAN — the relay uplink fades away once the local path is validated](https://www.iroh.computer/animations/hole-punching-lan.svg)

## How the pieces stack together

An iroh application is a stack of small layers, each with one job:

![An iroh application as a stack of layers inside your application: Protocols (blobs, gossip, yours), Router (dispatches connections by ALPN), Endpoint (identity, address lookup, NAT, relay), QUIC + TLS 1.3 (encryption, auth, stream mux), and Transport (UDP and relay by default, swappable) — with Ethernet, Wi-Fi, and Tor as interchangeable transports underneath](https://www.iroh.computer/animations/layer-stack.svg)

* **Transport** carries encrypted bytes between machines. UDP is the default;
  you can swap in [Tor](https://docs.iroh.computer/transports/tor.md), [Nym](https://docs.iroh.computer/transports/nym.md), or
  [Bluetooth](https://docs.iroh.computer/transports/bluetooth.md) when you need a different wire.
* **QUIC + TLS 1.3** provides end-to-end encryption, authentication, and
  stream multiplexing over that transport.
* **[Endpoint](https://docs.iroh.computer/concepts/endpoints.md)** is the connection-level API. It gives
  each node a stable `EndpointID`, finds peers through
  [address lookup](https://docs.iroh.computer/concepts/address-lookup.md), traverses NATs, and falls back to
  [relays](https://docs.iroh.computer/concepts/relays.md) when a direct path isn't available.
* **Router** listens on an endpoint and dispatches each incoming connection to
  the right protocol handler based on its
  [ALPN](https://docs.iroh.computer/concepts/protocols.md) string. This is what lets several protocols
  share one endpoint.
* **[Protocols](https://docs.iroh.computer/concepts/protocols.md)** define what two peers actually do once
  connected — transfer files, sync documents, broadcast messages. Mix them
  freely: `iroh-docs`, for example, is built on top of `iroh-blobs` and
  `iroh-gossip`.

Composing multiple protocols on a single endpoint looks like this:

```rust theme={null}
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{store::mem::MemStore, BlobsProtocol};
use iroh_gossip::net::Gossip;

let endpoint = Endpoint::bind(presets::N0).await?;

// Two independent protocols, sharing one endpoint.
let store = MemStore::new();
let blobs = BlobsProtocol::new(&store, None);
let gossip = Gossip::builder().spawn(endpoint.clone());

// The router routes each incoming connection to the right handler by ALPN.
let _router = Router::builder(endpoint)
    .accept(iroh_blobs::ALPN, blobs)
    .accept(iroh_gossip::ALPN, gossip)
    .spawn();
```

Two `accept` calls, one endpoint: both blobs and gossip are now reachable on
this node, routed by their ALPN.

## Getting started

To get started with iroh, check out the [quickstart guide](https://docs.iroh.computer/quickstart.md) or explore the
[protocols documentation](https://docs.iroh.computer/protocols/documents.md) to see what protocols are available and
how to use them in your applications.

Read the [how it works documentation](https://docs.iroh.computer/concepts/endpoints.md) to understand the underlying
principles and architecture of iroh.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.