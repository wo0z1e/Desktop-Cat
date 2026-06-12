use windows_sys::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTOPRIMARY,
};
use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::UI::HiDpi::GetDpiForSystem;

/// Structure to hold troubleshoot-ready display statistics
#[derive(Debug, Clone, Copy)]
pub struct ScreenMetrics {
    pub work_width: f32,
    pub work_height: f32,
    pub scale_factor: f32,
}

/// Dynamic logger that formats system outputs uniformly for easy tracking
pub fn log_diagnostic(level: &str, message: &str) {
    println!("[DIAGNOSTIC][{}] {}", level, message);
}

/// Queries the Windows Kernel natively to pull true desktop parameters
pub fn fetch_desktop_metrics() -> ScreenMetrics {
    log_diagnostic("INFO", "Initiating native DPI-aware display handshake...");

    let mut final_width = 1920.0;
    let mut final_height = 1080.0;
    
    // Read the DPI setting directly into the value allocation to prevent warnings
    let system_dpi = unsafe { GetDpiForSystem() };
    let scale_factor = system_dpi as f32 / 96.0;
    log_diagnostic("INFO", &format!("OS Kernel reported Scaling DPI: {} ({:.2}x multiplier)", system_dpi, scale_factor));

    unsafe {
        let test_point = POINT { x: 1, y: 1 };
        let h_monitor = MonitorFromPoint(test_point, MONITOR_DEFAULTTOPRIMARY);
        
        if h_monitor == 0 {
            log_diagnostic("ERROR", "Null pointer received for monitor address handle. Using standard 1080p fallbacks.");
        } else {
            let mut monitor_info: MONITORINFO = std::mem::zeroed();
            monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

            if GetMonitorInfoW(h_monitor, &mut monitor_info) == 0 {
                log_diagnostic("ERROR", "GetMonitorInfoW call rejected by core kernel. Driver mismatch suspected.");
            } else {
                let calculated_width = (monitor_info.rcWork.right - monitor_info.rcWork.left).abs();
                let calculated_height = (monitor_info.rcWork.bottom - monitor_info.rcWork.top).abs();

                if calculated_width > 0 && calculated_height > 0 {
                    final_width = calculated_width as f32;
                    final_height = calculated_height as f32;
                    log_diagnostic("INFO", &format!("Verified Screen Dimensions: {}x{}", final_width, final_height));
                }
            }
        }
    }

    let logical_width = final_width / scale_factor;
    let logical_height = final_height / scale_factor;

    ScreenMetrics {
        work_width: logical_width,
        work_height: logical_height,
        scale_factor,
    }
}