use sysinfo::{System, SystemExt, CpuExt};
use std::env;
use std::fs;
use std::process::Command;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct SystemError(String);

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "System error: {}", self.0)
    }
}

impl Error for SystemError {}

pub struct SystemInfo {
    pub os_name: String,
    pub hostname: String,
    pub kernel_version: String,
    pub uptime: String,
    pub package_count: String,
    pub shell: String,
    pub desktop_environment: String,
    pub window_manager: String,
    pub terminal: String,
    pub cpu: String,
    pub memory: String,
}

impl SystemInfo {
    pub fn new() -> Result<Self, SystemError> {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        Ok(SystemInfo {
            os_name: get_os_name(&sys)?,
            hostname: get_hostname(&sys)?,
            kernel_version: get_kernel_version(&sys)?,
            uptime: get_uptime(&sys)?,
            package_count: get_package_count()?,
            shell: get_shell()?,
            desktop_environment: get_desktop_environment()?,
            window_manager: get_window_manager()?,
            terminal: get_terminal()?,
            cpu: get_cpu_info(&sys)?,
            memory: get_memory_info(&sys)?,
        })
    }
}

fn get_os_name(sys: &System) -> Result<String, SystemError> {
    Ok(format!("{} {}", 
        sys.name().unwrap_or_else(|| "Unknown".to_string()),
        sys.os_version().unwrap_or_else(|| "Unknown".to_string())
    ))
}

fn get_hostname(sys: &System) -> Result<String, SystemError> {
    Ok(sys.host_name().unwrap_or_else(|| "Unknown".to_string()))
}

fn get_kernel_version(sys: &System) -> Result<String, SystemError> {
    Ok(sys.kernel_version().unwrap_or_else(|| "Unknown".to_string()))
}

fn get_uptime(sys: &System) -> Result<String, SystemError> {
    let uptime_seconds = sys.uptime();
    let days = uptime_seconds / 86400;
    let hours = (uptime_seconds % 86400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    
    if days > 0 {
        Ok(format!("{}d {}h {}m", days, hours, minutes))
    } else if hours > 0 {
        Ok(format!("{}h {}m", hours, minutes))
    } else {
        Ok(format!("{}m", minutes))
    }
}

fn get_package_count() -> Result<String, SystemError> {
    // Try different package managers
    let package_managers = vec![
        ("dpkg", vec!["--get-selections"]),
        ("rpm", vec!["-qa"]),
        ("pacman", vec!["-Q"]),
        ("apk", vec!["list", "--installed"]),
        ("pkg", vec!["info"]),
        ("brew", vec!["list"]),
        ("nix-env", vec!["-q"]),
        ("flatpak", vec!["list"]),
        ("snap", vec!["list"]),
    ];
    
    for (cmd, args) in package_managers {
        if let Ok(output) = Command::new(cmd).args(&args).output() {
            if output.status.success() {
                let count = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .count();
                return Ok(format!("{} ({})", count, cmd));
            }
        }
    }
    
    Ok("Unknown".to_string())
}

fn get_shell() -> Result<String, SystemError> {
    if let Ok(shell) = env::var("SHELL") {
        if let Some(shell_name) = shell.split('/').last() {
            return Ok(shell_name.to_string());
        }
    }
    Ok("Unknown".to_string())
}

fn get_desktop_environment() -> Result<String, SystemError> {
    let de_vars = vec![
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
        "XDG_SESSION_DESKTOP",
        "GNOME_DESKTOP_SESSION_ID",
        "KDE_FULL_SESSION",
    ];
    
    for var in de_vars {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() {
                return Ok(value);
            }
        }
    }
    
    Ok("Unknown".to_string())
}

fn get_window_manager() -> Result<String, SystemError> {
    // Check common window manager environment variables
    let wm_vars = vec!["WINDOW_MANAGER", "_JAVA_AWT_WM_NONREPARENTING"];
    
    for var in wm_vars {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() {
                return Ok(value);
            }
        }
    }
    
    // Check for GNOME/Mutter specifically
    if let Ok(session) = env::var("XDG_SESSION_DESKTOP") {
        match session.to_lowercase().as_str() {
            "gnome" | "ubuntu" => return Ok("Mutter".to_string()),
            _ => {}
        }
    }
    
    if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
        match desktop.to_lowercase().as_str() {
            "gnome" | "ubuntu:gnome" => return Ok("Mutter".to_string()),
            "kde" => return Ok("KWin".to_string()),
            "xfce" => return Ok("Xfwm4".to_string()),
            "lxde" => return Ok("Openbox".to_string()),
            "mate" => return Ok("Marco".to_string()),
            "cinnamon" => return Ok("Muffin".to_string()),
            _ => {}
        }
    }
    
    // Try to detect from running processes
    if let Ok(output) = Command::new("ps").args(&["-eo", "comm"]).output() {
        let processes = String::from_utf8_lossy(&output.stdout);
        let wms = vec![
            ("mutter", "Mutter"),
            ("kwin", "KWin"), 
            ("xfwm4", "Xfwm4"),
            ("openbox", "Openbox"),
            ("i3", "i3"),
            ("bspwm", "bspwm"),
            ("dwm", "dwm"),
            ("fluxbox", "Fluxbox"),
            ("awesome", "awesome"),
            ("xmonad", "xmonad"),
            ("qtile", "qtile"),
            ("marco", "Marco"),
            ("muffin", "Muffin"),
            ("enlightenment", "Enlightenment"),
            ("weston", "Weston"),
            ("sway", "Sway"),
            ("hyprland", "Hyprland"),
        ];
        
        for (process, name) in wms {
            if processes.contains(process) {
                return Ok(name.to_string());
            }
        }
    }
    
    Ok("Unknown".to_string())
}

fn get_terminal() -> Result<String, SystemError> {
    if let Ok(term) = env::var("TERM_PROGRAM") {
        return Ok(term);
    }
    
    if let Ok(term) = env::var("TERM") {
        return Ok(term);
    }
    
    Ok("Unknown".to_string())
}

fn get_cpu_info(sys: &System) -> Result<String, SystemError> {
    if let Some(cpu) = sys.cpus().first() {
        Ok(format!("{} ({} cores)", cpu.brand().trim(), sys.cpus().len()))
    } else {
        Ok("Unknown".to_string())
    }
}

fn get_memory_info(sys: &System) -> Result<String, SystemError> {
    let used_mb = (sys.used_memory() as f64) / (1024.0 * 1024.0);
    let total_mb = (sys.total_memory() as f64) / (1024.0 * 1024.0);
    let used_gb = used_mb / 1024.0;
    let total_gb = total_mb / 1024.0;
    
    if total_gb >= 1.0 {
        Ok(format!("{:.1}GB / {:.1}GB", used_gb, total_gb))
    } else {
        Ok(format!("{:.0}MB / {:.0}MB", used_mb, total_mb))
    }
}
