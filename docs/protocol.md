# VirtioConsole seiva protocol

## Messages (client → driver)

```rust
pub enum Message<'a> {
    /// Write `bytes` to the console. Bytes live in DMA-mapped memory until
    /// the driver returns `Reply::Ready`.
    Write(&'a [u8]),
    /// No-op sent by client to query readiness.
    Ready,
}
```

## Replies (driver → client)

```rust
pub enum Reply<'a> {
    /// Incoming bytes from the host. Lifetime scoped to the driver's
    /// receive buffer; client must copy or process before returning.
    Read(&'a [u8]),
    /// TX queue would overflow; `dropped` bytes discarded since last Ready.
    /// Indicates a writer is faster than the transport. Not a silent drop —
    /// the counter is part of the reply.
    BackPressure { dropped: u32 },
    Err(DriverError),
}
```

## Lifecycle

1. floresta spawns the driver with the `(mmio, dma, irq)` cap bag.
2. Driver `init` negotiates virtio features (VIRTIO_CONSOLE_F_SIZE, VIRTIO_CONSOLE_F_MULTIPORT as applicable).
3. Driver publishes an `Endpoint<VirtioConsole>` pair (sender/receiver) to floresta.
4. floresta hands the sender side to the kernel console service and the receiver side to the logging service.

## IRQ handling

IRQ waiter is a dedicated `proc_yield` thread. On IRQ:

1. Read the virtio interrupt status register (MMIO).
2. For each completed vring descriptor, decode the buffer.
3. If RX, forward as `Reply::Read`.
4. If TX, advance the TX ring and unblock any pending `Message::Write`.
5. `dev_irq_ack` to rearm.

No allocation on this path. All buffers pre-allocated at `init`.
