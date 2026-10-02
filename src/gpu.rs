//! GPU Hardware Acceleration Detector
//!
//! Inspects Linux DRM sysfs (/sys/class/drm/card*/device/vendor) and lspci to
//! auto-detect host GPU architecture (AMD VA-API, Intel QuickSync, NVIDIA NVENC).
//!
//! License: MIT
//! Author: Carlos Alberto <psncarlosalberto4ti@gmail.com>

use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GpuVendorInfo {
    pub vendor_name: &'static str,
    pub encoder_name: &'static str,
    pub rank_string: String,
    pub env_h264_enc: &'static str,
}

/// Detects host GPU vendor(s) via Linux DRM sysfs (/sys/class/drm/card*/device/vendor) and lspci
pub fn detect_gpu_hardware() -> GpuVendorInfo {
    let mut has_amd = false;
    let mut has_intel = false;
    let mut has_nvidia = false;

    // 1. Inspect DRM card vendor IDs in sysfs
    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("card") && name.chars().nth(4).map(|c| c.is_ascii_digit()).unwrap_or(false) {
                let vendor_path = entry.path().join("device/vendor");
                if let Ok(vendor_hex) = fs::read_to_string(vendor_path) {
                    let v = vendor_hex.trim().to_lowercase();
                    match v.as_str() {
                        "0x1002" => has_amd = true,
                        "0x8086" => has_intel = true,
                        "0x10de" => has_nvidia = true,
                        _ => {}
                    }
                }
            }
        }
    }

    // 2. Fallback to lspci if sysfs was inconclusive
    if !has_amd && !has_intel && !has_nvidia {
        if let Ok(output) = Command::new("lspci").output() {
            let s = String::from_utf8_lossy(&output.stdout).to_lowercase();
            if s.contains("amd") || s.contains("radeon") || s.contains("advanced micro devices") {
                has_amd = true;
            }
            if s.contains("intel") {
                has_intel = true;
            }
            if s.contains("nvidia") {
                has_nvidia = true;
            }
        }
    }

    if has_amd && !has_intel && !has_nvidia {
        GpuVendorInfo {
            vendor_name: "AMD Radeon (VA-API / RDNA / Vega / Mendocino)",
            encoder_name: "vaapih264enc / vah264enc",
            rank_string: "vaapih264enc:MAX,vah264enc:MAX".to_string(),
            env_h264_enc: "vaapih264enc",
        }
    } else if has_intel && !has_nvidia && !has_amd {
        GpuVendorInfo {
            vendor_name: "Intel HD / Iris Xe / Arc / UHD (QuickSync)",
            encoder_name: "vaapih264enc / vah264enc / qsvh264enc",
            rank_string: "vaapih264enc:MAX,vah264enc:MAX,qsvh264enc:MAX".to_string(),
            env_h264_enc: "vaapih264enc",
        }
    } else if has_nvidia && !has_amd && !has_intel {
        GpuVendorInfo {
            vendor_name: "NVIDIA GeForce / RTX / Quadro (NVENC)",
            encoder_name: "nvh264enc / vaapih264enc",
            rank_string: "nvh264enc:MAX,vaapih264enc:MAX".to_string(),
            env_h264_enc: "nvh264enc",
        }
    } else if has_nvidia && (has_amd || has_intel) {
        GpuVendorInfo {
            vendor_name: "Hybrid GPU (NVIDIA dGPU + AMD/Intel iGPU)",
            encoder_name: "nvh264enc / vaapih264enc / vah264enc",
            rank_string: "nvh264enc:MAX,vaapih264enc:MAX,vah264enc:MAX,qsvh264enc:MAX".to_string(),
            env_h264_enc: "nvh264enc",
        }
    } else {
        GpuVendorInfo {
            vendor_name: "Universal Hardware GPU (Auto-Ranking)",
            encoder_name: "vaapih264enc / vah264enc / nvh264enc / qsvh264enc",
            rank_string: "vaapih264enc:MAX,vah264enc:MAX,nvh264enc:MAX,qsvh264enc:MAX".to_string(),
            env_h264_enc: "vaapih264enc",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_gpu_hardware() {
        let gpu = detect_gpu_hardware();
        assert!(!gpu.vendor_name.is_empty());
        assert!(!gpu.encoder_name.is_empty());
    }
}
