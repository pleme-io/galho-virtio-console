//! # galho-virtio-console
//!
//! virtio-console driver. The first driver brasa spawns on boot; provides
//! the typed bidirectional byte stream used as the console endpoint.
//!
//! See [`docs/protocol.md`](../docs/protocol.md) for the seiva protocol.

#![cfg_attr(not(feature = "std"), no_std)]

use galho::{Driver, DriverError, DeviceHandle};

/// PCI vendor/device IDs for legacy + modern virtio-console.
pub const VIRTIO_VENDOR: u16 = 0x1af4;
pub const VIRTIO_CONSOLE_DEVICE: u16 = 0x1003;

/// The virtio-console seiva protocol. Messages are typed byte windows over
/// shared DMA buffers; the driver never allocates in the hot path.
pub enum Message<'a> {
    Write(&'a [u8]),
    Ready,
}

pub enum Reply<'a> {
    Read(&'a [u8]),
    BackPressure { dropped: u32 },
    Err(DriverError),
}

pub struct VirtioConsole {
    // Phase 1: ring buffers, DMA caps, IRQ waiter state.
}

impl Driver for VirtioConsole {
    fn init(&mut self) -> Result<(), DriverError> {
        // Phase 1: enumerate virtio config space, feature negotiation,
        // set up TX/RX vring, publish Endpoint<VirtioConsole>.
        Err(DriverError::Unsupported)
    }

    fn attach(&mut self, _device: DeviceHandle) -> Result<(), DriverError> {
        Err(DriverError::Unsupported)
    }

    fn detach(&mut self) -> Result<(), DriverError> {
        Err(DriverError::Unsupported)
    }
}
