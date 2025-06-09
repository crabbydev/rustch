use sysinfo::System;
use std::env;
use std::fs;
use std::process::Command;
use std::error::Error;
use std::fmt;
use std::collections::HashSet;

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
    // Try to get prettier name from /etc/os-release first
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                let pretty_name = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
                return Ok(pretty_name.to_string());
            }
        }
    }
    
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
    let mut total_packages = 0;
    let mut found_managers = Vec::new();
    
    // Package managers with their commands and how to count
    let package_managers = vec![
        ("dpkg", vec!["--get-selections"], "dpkg"),
        ("rpm", vec!["-qa"], "rpm"),
        ("pacman", vec!["-Q"], "pacman"),
        ("apk", vec!["list", "--installed"], "apk"),
        ("pkg", vec!["info"], "pkg"),
        ("brew", vec!["list"], "brew"),
        ("nix-env", vec!["-q"], "nix"),
        ("flatpak", vec!["list", "--app"], "flatpak"),
        ("snap", vec!["list"], "snap"),
        ("emerge", vec!["--list-installed"], "portage"),
        ("xbps-query", vec!["-l"], "xbps"),
    ];
    
    for (cmd, args, manager_name) in package_managers {
        if let Ok(output) = Command::new(cmd).args(&args).output() {
            if output.status.success() {
                let count = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.starts_with("Listing..."))
                    .count();
                    
                if count > 0 {
                    total_packages += count;
                    found_managers.push(format!("{} ({})", count, manager_name));
                }
            }
        }
    }
    
    if found_managers.is_empty() {
        Ok("Unknown".to_string())
    } else if found_managers.len() == 1 {
        Ok(found_managers[0].clone())
    } else {
        Ok(format!("{} total", total_packages))
    }
}

fn get_shell() -> Result<String, SystemError> {
    // Check SHELL environment variable
    if let Ok(shell) = env::var("SHELL") {
        if let Some(shell_name) = shell.split('/').last() {
            // Get version if possible
            if let Ok(output) = Command::new(shell_name).arg("--version").output() {
                let version_output = String::from_utf8_lossy(&output.stdout);
                if let Some(first_line) = version_output.lines().next() {
                    // Extract version number
                    if let Some(version) = extract_version(first_line) {
                        return Ok(format!("{} {}", shell_name, version));
                    }
                }
            }
            return Ok(shell_name.to_string());
        }
    }
    
    // Fallback to checking parent process
    if let Ok(output) = Command::new("ps").args(&["-p", &format!("{}", std::process::id()), "-o", "ppid="]).output() {
        if let Ok(ppid) = String::from_utf8_lossy(&output.stdout).trim().parse::<u32>() {
            if let Ok(output) = Command::new("ps").args(&["-p", &ppid.to_string(), "-o", "comm="]).output() {
                let parent_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !parent_name.is_empty() {
                    return Ok(parent_name);
                }
            }
        }
    }
    
    Ok("Unknown".to_string())
}

fn get_desktop_environment() -> Result<String, SystemError> {
    // Priority order for DE detection
    let de_vars = vec![
        ("XDG_CURRENT_DESKTOP", true),
        ("XDG_SESSION_DESKTOP", true),
        ("DESKTOP_SESSION", true),
        ("GNOME_DESKTOP_SESSION_ID", false),
        ("KDE_FULL_SESSION", false),
        ("MATE_DESKTOP_SESSION_ID", false),
    ];
    
    for (var, should_return_value) in de_vars {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() {
                if should_return_value {
                    return Ok(normalize_de_name(&value));
                } else {
                    // For boolean-like vars, return the DE name
                    return Ok(match var {
                        "GNOME_DESKTOP_SESSION_ID" => "GNOME".to_string(),
                        "KDE_FULL_SESSION" => "KDE Plasma".to_string(),
                        "MATE_DESKTOP_SESSION_ID" => "MATE".to_string(),
                        _ => "Unknown".to_string(),
                    });
                }
            }
        }
    }
    
    Ok("Unknown".to_string())
}

fn get_window_manager() -> Result<String, SystemError> {
    // First check environment variables
    if let Ok(wm) = env::var("WINDOW_MANAGER") {
        if !wm.is_empty() {
            return Ok(wm);
        }
    }
    
    // Check for common WMs based on DE
    if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
        match desktop.to_lowercase().as_str() {
            "gnome" | "ubuntu:gnome" => return Ok("Mutter".to_string()),
            "kde" => return Ok("KWin".to_string()),
            "xfce" => return Ok("Xfwm4".to_string()),
            "lxde" | "lxqt" => return Ok("Openbox".to_string()),
            "mate" => return Ok("Marco".to_string()),
            "cinnamon" => return Ok("Muffin".to_string()),
            "pantheon" => return Ok("Gala".to_string()),
            "budgie:gnome" | "budgie" => return Ok("Mutter".to_string()),
            _ => {}
        }
    }
    
    // Check for Wayland vs X11
    if env::var("WAYLAND_DISPLAY").is_ok() || env::var("XDG_SESSION_TYPE").map_or(false, |t| t == "wayland") {
        // Wayland compositors
        let wayland_wms = vec![
            ("sway", "Sway"),
            ("hyprland", "Hyprland"),
            ("river", "River"),
            ("wayfire", "Wayfire"),
            ("weston", "Weston"),
            ("cage", "Cage"),
            ("hikari", "Hikari"),
        ];
        
        if let Ok(output) = Command::new("ps").args(&["-eo", "comm"]).output() {
            let processes = String::from_utf8_lossy(&output.stdout);
            for (process, name) in wayland_wms {
                if processes.contains(process) {
                    return Ok(name.to_string());
                }
            }
        }
    }
    
    // Check for X11 window managers
    if let Ok(output) = Command::new("ps").args(&["-eo", "comm"]).output() {
        let processes = String::from_utf8_lossy(&output.stdout);
        let x11_wms = vec![
            ("mutter", "Mutter"),
            ("kwin", "KWin"), 
            ("kwin_x11", "KWin"),
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
            ("jwm", "JWM"),
            ("icewm", "IceWM"),
            ("fvwm", "FVWM"),
            ("herbstluftwm", "Herbstluftwm"),
            ("spectrwm", "Spectrwm"),
        ];
        
        for (process, name) in x11_wms {
            if processes.contains(process) {
                return Ok(name.to_string());
            }
        }
    }
    
    Ok("Unknown".to_string())
}

fn get_terminal() -> Result<String, SystemError> {
    // Check terminal-specific environment variables first
    if let Ok(term) = env::var("TERM_PROGRAM") {
        if !term.is_empty() && term != "tmux" {
            return Ok(normalize_terminal_name(&term));
        }
    }
    
    // Check other terminal variables
    let term_vars = vec![
        "KONSOLE_VERSION",
        "GNOME_TERMINAL_SCREEN",
        "TERMINATOR_UUID",
        "KITTY_WINDOW_ID",
        "ALACRITTY_SOCKET",
        "WT_SESSION",
    ];
    
    for var in term_vars {
        if env::var(var).is_ok() {
            return Ok(match var {
                "KONSOLE_VERSION" => "Konsole".to_string(),
                "GNOME_TERMINAL_SCREEN" => "GNOME Terminal".to_string(),
                "TERMINATOR_UUID" => "Terminator".to_string(),
                "KITTY_WINDOW_ID" => "kitty".to_string(),
                "ALACRITTY_SOCKET" => "Alacritty".to_string(),
                "WT_SESSION" => "Windows Terminal".to_string(),
                _ => "Unknown".to_string(),
            });
        }
    }
    
    // Try to detect from parent process
    if let Ok(output) = Command::new("ps").args(&["-p", &std::process::id().to_string(), "-o", "ppid="]).output() {
        if let Ok(ppid) = String::from_utf8_lossy(&output.stdout).trim().parse::<u32>() {
            if let Ok(output) = Command::new("ps").args(&["-p", &ppid.to_string(), "-o", "comm="]).output() {
                let parent_comm = String::from_utf8_lossy(&output.stdout).trim();
                let terminal_names = vec![
                    ("gnome-terminal", "GNOME Terminal"),
                    ("konsole", "Konsole"),
                    ("xfce4-terminal", "Xfce Terminal"),
                    ("lxterminal", "LXTerminal"),
                    ("mate-terminal", "MATE Terminal"),
                    ("terminator", "Terminator"),
                    ("tilix", "Tilix"),
                    ("alacritty", "Alacritty"),
                    ("kitty", "kitty"),
                    ("urxvt", "rxvt-unicode"),
                    ("rxvt", "rxvt"),
                    ("xterm", "xterm"),
                    ("st", "st"),
                    ("foot", "foot"),
                    ("wezterm", "WezTerm"),
                ];
                
                for (process, name) in terminal_names {
                    if parent_comm.contains(process) {
                        return Ok(name.to_string());
                    }
                }
                
                if !parent_comm.is_empty() {
                    return Ok(parent_comm.to_string());
                }
            }
        }
    }
    
    // Fallback to TERM variable
    if let Ok(term) = env::var("TERM") {
        Ok(term)
    } else {
        Ok("Unknown".to_string())
    }
}

fn get_cpu_info(sys: &System) -> Result<String, SystemError> {
    let cpus = sys.cpus();
    if let Some(cpu) = cpus.first() {
        let brand = cpu.brand().trim();
        let core_count = cpus.len();
        
        // Clean up CPU brand name
        let cleaned_brand = brand
            .replace("(R)", "")
            .replace("(TM)", "")
            .replace("  ", " ")
            .trim()
            .to_string();
            
        Ok(format!("{} ({} cores)", cleaned_brand, core_count))
    } else {
        Ok("Unknown".to_string())
    }
}

fn get_memory_info(sys: &System) -> Result<String, SystemError> {
    let used_bytes = sys.used_memory();
    let total_bytes = sys.total_memory();
    
    // Convert to appropriate units
    let (used_val, used_unit) = format_bytes(used_bytes);
    let (total_val, total_unit) = format_bytes(total_bytes);
    
    // Calculate percentage
    let percentage = if total_bytes > 0 {
        (used_bytes as f64 / total_bytes as f64 * 100.0) as u32
    } else {
        0
    };
    
    Ok(format!("{:.1}{} / {:.1}{} ({}%)", 
        used_val, used_unit, total_val, total_unit, percentage))
}

// Helper functions
fn extract_version(text: &str) -> Option<String> {
    // Simple regex-like version extraction
    let words: Vec<&str> = text.split_whitespace().collect();
    for word in words {
        if word.chars().next().map_or(false, |c| c.is_ascii_digit()) {
            if let Some(end) = word.find(|c: char| !c.is_ascii_digit() && c != '.') {
                return Some(word[..end].to_string());
            } else if word.chars().all(|c| c.is_ascii_digit() || c == '.') {
                return Some(word.to_string());
            }
        }
    }
    None
}

fn normalize_de_name(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "ubuntu:gnome" | "gnome" => "GNOME".to_string(),
        "kde" => "KDE Plasma".to_string(),
        "xfce" => "Xfce".to_string(),
        "lxde" => "LXDE".to_string(),
        "lxqt" => "LXQt".to_string(),
        "mate" => "MATE".to_string(),
        "cinnamon" => "Cinnamon".to_string(),
        "pantheon" => "Pantheon".to_string(),
        "budgie:gnome" | "budgie" => "Budgie".to_string(),
        "i3" => "i3".to_string(),
        "sway" => "Sway".to_string(),
        _ => name.to_string(),
    }
}

fn normalize_terminal_name(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "apple_terminal" => "Terminal.app".to_string(),
        "iterm.app" => "iTerm2".to_string(),
        "hyper" => "Hyper".to_string(),
        "vscode" => "VS Code".to_string(),
        _ => name.to_string(),
    }
}

fn format_bytes(bytes: u64) -> (f64, &'static str) {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    (size, UNITS[unit_index])
}
