# galho-virtio-console — operator instructions

The first userspace driver on every brasa boot. virtio-console provides the typed bidirectional byte stream used as the boot log sink and the generic console endpoint.

See [brasa ADR-0004](https://github.com/pleme-io/brasa/blob/main/docs/adrs/0004-tatara-lisp-authoring.md) for the `(defdriver …)` authoring surface this driver is generated around.

## Non-negotiables

- No allocation in the RX/TX hot path. Ring buffers are pre-allocated at `init`.
- Messages are bounded. If a writer floods the console, backpressure is applied via `seiva::Protocol::Reply::BackPressure`. No drops without notice.
- The console is never treated as a trusted text sink. All output is typed bytes; formatting is the caller's concern.

## Authoring surface

```lisp
(defdriver :name virtio-console
           :bus :pci
           :match {:vendor 0x1af4 :device 0x1003}
           :caps-requested [(mmio :device-bound)
                            (dma :size 64KB)
                            (irq :any)]
           :protocol virtio-console
           :impl (rust-crate "galho-virtio-console"))
```

`forja` expands this into the `impl Driver for VirtioConsole { … }` boilerplate. The body of `init`, `attach`, `detach` is hand-written in `src/lib.rs`.
