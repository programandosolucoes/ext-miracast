//! Autonomous Pure-Rust Universal Miracast (Wi-Fi Display / WFD 1.0 / MS-MICE) Engine
//!
//! A high-performance, zero-GTK, pure-Rust implementation of the Wi-Fi Display (Miracast)
//! protocol stack, RTSP 1.0 WFD M1-M7 state machine, 188-byte MPEG-TS/PES packetizer,
//! and multi-architecture hardware accelerated desktop video streaming.
//!
//! Features:
//! - Multi-architecture GPU hardware video encoding (AMD VA-API, Intel QuickSync, NVIDIA NVENC, CPU OpenH264)
//! - Multiple selectable desktop capture backends:
//!   - `mutter`: GNOME Wayland ScreenCast D-Bus (`RecordVirtual` for extend, `RecordMonitor` for clone)
//!   - `kernel`: Direct Linux Kernel DRM/KMS scanout (/dev/dri/card* & /dev/fb0)
//!   - `x11`: Standard X11 XShm screen grabber
//! - Display modes: `--mode extend` (virtual monitor) and `--mode clone` (primary monitor)
//! - Sub-100ms graceful session termination with atomic RTSP TEARDOWN
//! - Automatic network sink discovery via mDNS, SSDP, and Direct-USB
//!
//! License: MIT
//! Author: Carlos Alberto <psncarlosalberto4ti@gmail.com>

pub mod client;
pub mod discovery;
pub mod encoder;
pub mod gpu;
pub mod kms;
pub mod mpegts;
pub mod screencast;
pub mod wfd_client;

pub use client::{MiracastConfig, MiracastSession};
pub use discovery::{interactive_select_sink, scan_sinks, DiscoveredSink};
pub use gpu::{detect_gpu_hardware, GpuVendorInfo};
pub use mpegts::MpegTsMuxer;
pub use wfd_client::WfdClient;
