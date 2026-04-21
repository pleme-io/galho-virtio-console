# galho-virtio-console

> virtio-console driver — the first galho to sprout on every brasa boot.

A userspace driver for the virtio-console transport (VIRTIO 1.2, §5.3). Provides a typed bidirectional byte stream over IPC — used as the kernel/userspace boot log sink in Phase 1 and as a generic console endpoint thereafter.

**Status:** Phase 0 — Design.
**License:** MIT.

## What it does

- Enumerates virtio-console PCI devices from its granted `Cap<EnumCap>`.
- Requests MMIO/DMA/IRQ caps per device from floresta at startup.
- Spins up a `seiva::Protocol<VirtioConsole>` endpoint pair (sender/receiver).
- Translates `Message::Write(bytes)` → virtio TX queue.
- Translates virtio RX queue → `Message::Read(bytes)` on the receiver endpoint.

## Why virtio-console is the *first* galho

Per [brasa ADR-0006](https://github.com/pleme-io/brasa/blob/main/docs/adrs/0006-first-target-apple-silicon-kasou.md), the first target is Apple Silicon via kasou. Kasou exposes a virtio-console by default. Without this driver, no text leaves the VM — no "it works" signal, no debug log.

Phase 0 deliverable in [brasa's roadmap](https://github.com/pleme-io/brasa/blob/main/docs/roadmap.md) calls for *"`tronco` boots under QEMU aarch64, prints a typed message via a single syscall, exits cleanly."* That print lands through this driver.

## Build

Phase 0: nothing builds. Phase 1: builds as a `[[bin]]` targeting the brasa triple; produces a service ELF that floresta spawns at boot.

## Dependencies

- [`galho`](https://github.com/pleme-io/brasa/tree/main/crates/galho) — driver framework (`Driver` trait)
- [`casca`](https://github.com/pleme-io/brasa/tree/main/crates/casca) — syscall ABI
- [`seiva`](https://github.com/pleme-io/brasa/tree/main/crates/seiva) — typed IPC
- [`folha`](https://github.com/pleme-io/brasa/tree/main/crates/folha) — userspace runtime

## See also

- [`docs/protocol.md`](./docs/protocol.md) — the `VirtioConsole` seiva protocol
- [virtio-v1.2 §5.3](https://docs.oasis-open.org/virtio/virtio/v1.2/os-virtio-v1.2.html) — upstream spec
