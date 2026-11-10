> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Quickstart

> Connect two iroh endpoints in five minutes

In five minutes you'll have two iroh endpoints connecting over the [iroh-ping](https://github.com/n0-computer/iroh-ping) protocol. Clone the repo, run the included quickstart example as a receiver in one terminal and a sender in another, and watch the round-trip time print.

### 1. Install Rust

```bash theme={null}
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone iroh-ping

```bash theme={null}
git clone https://github.com/n0-computer/iroh-ping
cd iroh-ping
```

### 3. Run the receiver

```bash theme={null}
cargo run --example quickstart receiver
```

The receiver prints a ticket. Copy it.

### 4. Run the sender

In another terminal:

```bash theme={null}
cargo run --example quickstart sender <TICKET>
```

Replace `<TICKET>` with the ticket from the receiver. The sender will print the round-trip time once the connection succeeds.

### 5. See your metrics (optional)

Now that ping is working, hook the example up to Iroh Services to watch connectivity metrics like direct data rate. Get an API key from your project's **Settings → API Keys** tab, export it, and rerun the example:

```bash theme={null}
export IROH_SERVICES_API_SECRET=<your-api-key>
```

The example picks up the key automatically and starts reporting metrics. See [API Keys](https://docs.iroh.computer/iroh-services/access.md) for the full walkthrough, or [View metrics](https://docs.iroh.computer/iroh-services/quickstart.md) to read the dashboard.

## Next steps

Integrate iroh into your own app by following one of the tutorials below.

> [card] [Connect two endpoints](https://docs.iroh.computer/connect-two-endpoints.md)
>
> Build the receiver/sender ping app from scratch and learn how iroh-ping, tickets, and routers fit together.

> [card] [View metrics](https://docs.iroh.computer/iroh-services/quickstart.md)
>
> See your direct data rate and other connectivity metrics in the dashboard.

> [card] [Add a relay](https://docs.iroh.computer/add-a-relay.md)
>
> Choose Shared, Dedicated, or self-hosted relays for your production endpoints.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.