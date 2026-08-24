use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() {
    println!("\x1b[1;31m==================================================\x1b[0m");
    println!("\x1b[1;31m[MAGI SUPERCOMPUTER] NERV MAIN INFRASTRUCTURE\x1b[0m");
    println!("\x1b[1;31m[WARNING]\x1b[0m INITIATING BARE-METAL DIAGNOSTICS...");
    println!("\x1b[1;31m==================================================\x1b[0m");

    // 1. System Uptime
    if let Ok(uptime_raw) = fs::read_to_string("/proc/uptime") {
        if let Some(uptime_secs) = uptime_raw.split_whitespace().next() {
            if let Ok(secs) = uptime_secs.parse::<f64>() {
                println!(
                    "\x1b[1;35m[SYSTEM STATUS]\x1b[0m Active Cycle   : {:.2} hours",
                    secs / 3600.0
                );
            }
        }
    }

    // 2. CPU Frequency (Live pull from sysfs for Core 0)
    let freq_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq";
    if let Ok(freq_str) = fs::read_to_string(freq_path) {
        if let Ok(freq_khz) = freq_str.trim().parse::<f64>() {
            println!(
                "\x1b[1;33m[CORE FREQ]\x1b[0m     Output (C0)    : {:.2} MHz",
                freq_khz / 1000.0
            );
        }
    } else {
        println!("\x1b[1;33m[CORE FREQ]\x1b[0m     Output         : \x1b[31mOFFLINE\x1b[0m");
    }

    // 3. CPU Load Average
    if let Ok(load) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = load.split_whitespace().collect();
        if parts.len() >= 3 {
            println!(
                "\x1b[1;33m[SYNC RATIO]\x1b[0m    Load Averages  : {} | {} | {}",
                parts[0], parts[1], parts[2]
            );
        }
    }

    // 4. Memory Allocation
    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);
        let mut total_mem = String::new();
        let mut free_mem = String::new();

        for line in reader.lines().flatten() {
            if line.starts_with("MemTotal:") {
                total_mem = line.replace("MemTotal:", "").trim().to_string();
            } else if line.starts_with("MemAvailable:") {
                free_mem = line.replace("MemAvailable:", "").trim().to_string();
            }
            if !total_mem.is_empty() && !free_mem.is_empty() {
                break;
            }
        }
        println!(
            "\x1b[1;36m[LCL POOL]\x1b[0m      Total Capacity : {}",
            total_mem
        );
        println!(
            "\x1b[1;36m[LCL POOL]\x1b[0m      Available      : {}",
            free_mem
        );
    }

    // 5. GPU Interrogation via subprocess
    if let Ok(output) = Command::new("nvidia-smi")
        .arg("--query-gpu=name,temperature.gpu,utilization.gpu,memory.used,memory.total")
        .arg("--format=csv,noheader")
        .output()
    {
        if output.status.success() {
            let gpu_info = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = gpu_info.trim().split(',').collect();

            if parts.len() >= 5 {
                println!("\x1b[1;31m--------------------------------------------------\x1b[0m");
                println!(
                    "\x1b[1;32m[EVANGELION UNIT]\x1b[0m Hardware     :{}",
                    parts[0]
                );
                println!(
                    "\x1b[1;32m[EVANGELION UNIT]\x1b[0m Core Temp    :{} °C",
                    parts[1].trim()
                );
                println!(
                    "\x1b[1;32m[EVANGELION UNIT]\x1b[0m Utilization  :{}",
                    parts[2].trim()
                );
                println!(
                    "\x1b[1;32m[EVANGELION UNIT]\x1b[0m VRAM Load    :{} /{}",
                    parts[3].trim(),
                    parts[4].trim()
                );
            }
        }
    }

    println!("\x1b[1;31m==================================================\x1b[0m");
}
