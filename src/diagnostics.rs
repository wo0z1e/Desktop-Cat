use sysinfo::{Pid, System};
use std::process;
use crate::windows_api::log_diagnostic;

pub struct ResourceMonitor {
    sys: System,
    pid: Pid,
}

impl ResourceMonitor {
    pub fn new() -> Self {
        // In v0.30, we use System::new_all() directly without needing SystemExt
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let current_pid = Pid::from(process::id() as usize);

        Self {
            sys,
            pid: current_pid,
        }
    }

    /// Polls the OS for current hardware usage and prints a formatted diagnostic report
    pub fn sample_usage(&mut self) {
        // Modern syntax: single process refreshing is handled directly on the system instance
        self.sys.refresh_process(self.pid);

        if let Some(process) = self.sys.process(self.pid) {
            // Memory reading remains in bytes, conversion stays the same
            let ram_bytes = process.memory();
            let ram_mb = ram_bytes as f32 / 1_048_576.0;

            // CPU calculation handles core metrics automatically
            let cpu_usage = process.cpu_usage();
            let general_status = process.status();

            log_diagnostic(
                "PERFORMANCE",
                &format!(
                    "CPU: {:.2}% | RAM: {:.2} MB | Status: {:?}",
                    cpu_usage, ram_mb, general_status
                ),
            );
        } else {
            log_diagnostic("ERROR", "Failed to target active Process ID profile.");
        }
    }
}