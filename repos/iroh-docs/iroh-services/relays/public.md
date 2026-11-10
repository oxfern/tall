> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Public Relays

> Free shared relays for development and hobby use

iroh includes a set of public relays provided by [n0.computer](https://n0.computer), available to all iroh users at no cost. No setup required; they work out of the box.

Public relays are open to every iroh endpoint and need no authentication. This is different from [managed relays](https://docs.iroh.computer/iroh-services/relays/managed.md), which require authentication by default so only your project's endpoints can use them.

Public relays are suitable for **development and hobby use only**. For production, use [managed relays](https://docs.iroh.computer/iroh-services/relays/managed.md).

## Support policy

* No SLA or uptime guarantee
* Only the latest stable release of iroh is officially supported (older versions may work, but this is not guaranteed and can change at any time)
* No [version locking](https://docs.iroh.computer/iroh-services/relays/managed.md#version-locking); n0.computer reserves the right to remove support for older iroh versions at any time
* Traffic is rate-limited to prevent abuse

For [version locking](https://docs.iroh.computer/iroh-services/relays/managed.md#version-locking), blue/green deployments, or multi-region support, [deploy a managed relay](https://docs.iroh.computer/iroh-services/relays/managed.md).

## Privacy and security

All traffic through the public relays is end-to-end encrypted. The relays cannot read any of the traffic they forward.

However, the relays can see connection metadata: source and destination IP addresses, connection times, and the amount of data transferred. We recommend against using public relays for sensitive or confidential data.

We monitor public relays for abuse. If we detect malicious activity, we reserve the right to block offending IP addresses or users.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.