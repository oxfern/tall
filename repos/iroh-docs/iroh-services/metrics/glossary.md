> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Metrics Glossary

> Complete reference of all available metrics in Iroh Services

List of all available metrics. Below is a reference grouping of every metric exposed by the frontend metrics registry, organized by level:

## Project-level metrics

These metrics are aggregated at the project level (overall project view / dashboard).

| Metric (group.metric) | Short description | Units |
| - | - | -: |
| direct\_data\_rate | Measured traffic that doesn't pass through a relay, as a percentage of total traffic | Percentage |
| direct\_connection\_rate | Measured connections that become direct at least once during the lifetime of the connection | Percentage |
| blobs.egress | Blobs egress | Bytes |
| blobs.ingress | Blobs ingress | Bytes |
| connections.active | Active connections through relay servers | Count |
| latency.p50 | 50th percentile latency between endpoints | Milliseconds |
| latency.p90 | 90th percentile latency between endpoints | Milliseconds |
| latency.p99 | 99th percentile latency between endpoints | Milliseconds |

## Relay-level metrics

Metrics in this section relate to relays and relay servers (visibility about relay processes/servers).

| Metric (group.metric) | Short description | Units |
| - | - | -: |
| relay.bytes\_recv | Total bytes received (relay side) | Bytes |
| relay.bytes\_sent | Total bytes sent (relay side) | Bytes |
| relay.conns\_rx\_ratelimited | Connections rate-limited events | Count |
| relay.disco\_packets\_dropped | Address Lookup packets dropped | Count |
| relay.disco\_packets\_recv | Address Lookup packets received | Count |
| relay.disco\_packets\_sent | Address Lookup packets sent | Count |
| relay.disconnects | Disconnect events | Count |
| relay.frames\_rx\_ratelimited | Frames rate-limited events | Count |
| relay.got\_ping | Got ping events | Count |
| relay.other\_packets\_dropped | Other packets dropped | Count |
| relay.other\_packets\_recv | Other packets received | Count |
| relay.other\_packets\_sent | Other packets sent | Count |
| relay.relay\_accepts | Relay accepts (new relay connections accepted) | Count |
| relay.send\_packets\_dropped | Sent packets dropped | Count |
| relay.send\_packets\_recv | Sent packets received | Count |
| relay.send\_packets\_sent | Sent packets sent | Count |
| relay.sent\_pong | Sent pong events | Count |
| relay.unique\_client\_keys | Unique client keys observed | Count |
| relay.unknown\_frames | Unknown frame types observed | Count |
| relay.websocket\_accepts | WebSocket accepts | Count |
| relay.relay\_server.connections | Relay server connections | Count |
| relay.relay\_server.egress | Number of bytes relayed (egress) | Bytes |

## Endpoint-level metrics

These metrics are collected at the endpoint/node level to give per-endpoint visibility.

| Metric (group.metric) | Short description | Units |
| - | - | -: |
| connection\_became\_direct | Count of connections that switched from using a relay to a direct connection at least once | Count |
| connection\_handshake\_success | Count of successfully established secure connections | Count |
| recv\_data\_ipv4 | Measured data received directly by nodes through IPv4 | Bytes |
| recv\_data\_ipv6 | Measured data received directly by nodes through IPv6 | Bytes |
| recv\_data\_relay | Measured data received by nodes through relay (IPv4 or IPv6) | Bytes |
| send\_data | Measured data sent from nodes in the network | Bytes |
| send\_ipv4 | Amount of data sent directly via IPv4 | Bytes |
| send\_ipv6 | Amount of data sent directly via IPv6 | Bytes |
| send\_relay | Amount of data sent through relay (measured by nodes sending data) | Bytes |
| send\_data\_network\_down | Data send events when network down | Bytes |
| nodes\_contacted\_directly | Number of nodes contacted directly | Count |
| nodes\_contacted | Number of nodes contacted (any) | Count |
| num\_direct\_conns\_added | Number of direct connections added | Count |
| num\_direct\_conns\_removed | Number of direct connections removed | Count |
| num\_relay\_conns\_added | Number of relay connections added | Count |
| num\_relay\_conns\_removed | Number of relay connections removed | Count |
| recv\_datagrams | Received datagrams | Count |
| recv\_gro\_datagrams | Received GRO datagram counts | Count |
| update\_direct\_addrs | Direct address update events | Count |
| portmap.external\_address\_updated | External address updated events | Count |
| portmap.local\_port\_updates | Local port updates events | Count |
| portmap.mapping\_attempts | Mapping attempts (NAT traversal attempts) | Count |
| portmap.mapping\_failures | Mapping failures (failed NAT mappings) | Count |
| portmap.pcp\_available | PCP (Port Control Protocol) availability events | Count |
| portmap.pcp\_probes | PCP probe events | Count |
| portmap.probes\_started | Portmap probes started | Count |
| portmap.upnp\_available | UPnP availability events | Count |
| portmap.upnp\_gateway\_updated | UPnP gateway updated events | Count |
| portmap.upnp\_probes\_failed | UPnP probes failed | Count |
| portmap.upnp\_probes | UPnP probe events | Count |
| netreport.report\_reports\_full | NetReport: reports\_full (report fullness events) | Count |
| netreport.report\_reports | NetReport: reports (report events) | Count |
| netreport.report\_stun\_packets\_dropped | STUN packets dropped (netreport) | Count |
| netreport.report\_stun\_packets\_recv\_ipv4 | STUN packets received (IPv4) | Count |
| netreport.report\_stun\_packets\_recv\_ipv6 | STUN packets received (IPv6) | Count |
| netreport.report\_stun\_packets\_sent\_ipv4 | STUN packets sent (IPv4) | Count |
| netreport.report\_stun\_packets\_sent\_ipv6 | STUN packets sent (IPv6) | Count |


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.