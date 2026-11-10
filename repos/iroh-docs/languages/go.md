> ## Documentation Index
> Fetch the complete documentation index at: https://docs.iroh.computer/llms.txt
> Use this file to discover all available pages before exploring further.

# Go

> Use iroh from Go via the [iroh-ffi](https://github.com/n0-computer/iroh-ffi) bindings.

[`iroh-go`](https://git.coopcloud.tech/decentral1se/iroh-go) is still
considered experimental but ready for testing and open for contribution to
improve the quality and reliability of the bindings. There is a specific focus
on making the installation and cross platform support [as simple as
possible](https://git.coopcloud.tech/decentral1se/iroh-go#motivation).

| Platform | Architectures |
| - | - |
| Linux | x86\_64, aarch64 (musl) |

* **API reference:** [Go API docs](https://godocs.io/git.coopcloud.tech/decentral1se/iroh-go)
* **Examples:** [iroh-go/examples](https://git.coopcloud.tech/decentral1se/iroh-go/src/branch/main/examples)
* **All languages:** [platform support matrix](https://docs.iroh.computer/languages.md#platform-support)

## Install

We're aiming to publish a stable tag once we see more usage and adoption of the
Go bindings.

```bash theme={null}
go get git.coopcloud.tech/decentral1se/iroh-go
```

## Hello, iroh

```go theme={null}
package main

import (
  "fmt"

  iroh "git.coopcloud.tech/decentral1se/iroh-go"
)

var (
  alpn = []byte("iroh-go-hello/0")
)

func main() {
  preset := iroh.PresetN0()
  opts := iroh.EndpointOptions{
    Preset: &preset,
    Alpns:  &[][]byte{alpn},
  }

  endpoint, err := iroh.EndpointBind(opts)
  if err != nil {
    panic(err)
  }

  endpoint.Online()

  fmt.Println("endpoint id:", endpoint.Id())
}
```

This binds an endpoint using the N0 preset (public discovery + default relays),
waits until it has a home relay, and prints its 64-character endpoint id. Run
it with `go run`.

## Next steps

> [card] [Bidirectional chat](https://git.coopcloud.tech/decentral1se/iroh-go/src/branch/main/examples/pairchat)
>
> Connect two endpoints and stream messages between them.

> [card] [Multiple live connections](https://git.coopcloud.tech/decentral1se/iroh-go/src/branch/main/examples/timeserve)
>
> Connect multiple endpoints and broadcast messages.

> [card] [iroh-go repository](https://git.coopcloud.tech/decentral1se/iroh-go)
>
> Source, examples, and issue tracker for the iroh Go FFI bindings. Contributions are welcome!


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.