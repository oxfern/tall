> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# C

> Use iroh from C via the iroh-c-ffi bindings.

| Platform | Architectures |
| - | - |
| Linux | x86\_64 (glibc) |
| macOS | arm64 |
| Windows | x86\_64 |

* **Header:** [`irohnet.h`](https://github.com/n0-computer/iroh-c-ffi/blob/main/irohnet.h), checked into the repo
* **Examples:** [main.c](https://github.com/n0-computer/iroh-c-ffi/blob/main/main.c) and the [multi-thread examples](https://github.com/n0-computer/iroh-c-ffi/tree/main)
* **All languages:** [platform support matrix](https://docs.iroh.computer/languages.md#platform-support)

[`iroh-c-ffi`](https://github.com/n0-computer/iroh-c-ffi) is a raw C API generated with [safer-ffi](https://github.com/getditto/safer-ffi) — unlike the Python, Swift, Kotlin, and JavaScript bindings, it isn't built on `iroh-ffi`/uniffi. There are no prebuilt binaries; you build the static/shared library from source and link against it.

## Install

```bash theme={null}
git clone https://github.com/n0-computer/iroh-c-ffi
cd iroh-c-ffi
cargo build --release
```

This produces `target/release/libiroh_c_ffi.{a,so,dylib}`. The matching header is already checked in as `irohnet.h`; regenerate it only if you're tracking an API change:

```bash theme={null}
cargo run --features headers --bin generate_headers --release
```

## Hello, iroh

```c theme={null}
#include <stdio.h>
#include <string.h>
#include "irohnet.h"

int main(void) {
  char alpn[] = "hello-c-ffi/0";
  slice_ref_uint8_t alpn_slice;
  alpn_slice.ptr = (uint8_t *) alpn;
  alpn_slice.len = strlen(alpn);

  EndpointConfig_t config = endpoint_config_default();
  endpoint_config_add_alpn(&config, alpn_slice);

  Endpoint_t *ep = endpoint_default();
  if (endpoint_bind(&config, NULL, NULL, &ep) != 0) {
    fprintf(stderr, "failed to bind\n");
    return 1;
  }
  endpoint_online(&ep, 5000);

  EndpointAddr_t addr = endpoint_addr_default();
  endpoint_addr(&ep, &addr);
  printf("endpoint id: %s\n", public_key_as_base32(&addr.id));

  endpoint_free(ep);
  endpoint_config_free(config);
  return 0;
}
```

Build and link against the library from the previous step:

```bash theme={null}
cc -o hello hello.c -L target/release -liroh_c_ffi -lm
```

This binds an endpoint with a single ALPN, waits up to 5 seconds for a home relay, and prints the endpoint id. See the [CI workflow](https://github.com/n0-computer/iroh-c-ffi/blob/main/.github/workflows/ci.yml) for the extra link flags Windows needs.

## Next steps

> [card] [main.c example](https://github.com/n0-computer/iroh-c-ffi/blob/main/main.c)
>
> A full server/client pair: bind, accept, send, and receive over uni- and bi-directional streams.

> [card] [Multi-threaded examples](https://github.com/n0-computer/iroh-c-ffi/tree/main)
>
> `multi-thread-server.c`, `multi-thread-client.c`, and `single-thread-server.c` for handling concurrent connections.

> [card] [iroh-c-ffi repository](https://github.com/n0-computer/iroh-c-ffi)
>
> Source, generated header, and issue tracker for the C bindings.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.