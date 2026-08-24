# NERV-PROC-SYS

**[MAGI SUPERCOMPUTER] BARE-METAL INFRASTRUCTURE DIAGNOSTICS**

A zero-dependency, ultra-lightweight system monitor written in pure Rust. Rather than relying on bloated external crates or user-space wrappers like `htop`, this binary parses hardware metrics directly from the Linux kernel's virtual filesystems (`/proc` and `/sys`) in milliseconds.

## ⚙️ Core Architecture

- **Zero Allocations:** Utilizes `BufReader` to parse kernel outputs line-by-line, breaking execution instantly once targeted metrics are found to prevent unnecessary heap allocations.
- **Sysfs Interrogation:** Manually routes to physical hardware mappings (e.g., `/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq`) for live clock speeds.
- **Procfs Parsing:** Directly reads and formats `/proc/loadavg`, `/proc/uptime`, and `/proc/meminfo`.
- **Hardware Polling:** Spawns a silent subprocess to interrogate the NVIDIA driver via `nvidia-smi` and extracts raw CSV data for GPU thermals and VRAM utilization.

## 🚀 Execution

Build the optimized release binary:
```bash
cargo build --release
