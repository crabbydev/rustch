use std::fs;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DistroError(String);

impl fmt::Display for DistroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Distribution detection error: {}", self.0)
    }
}

impl Error for DistroError {}

pub fn detect_distribution() -> Result<String, DistroError> {
    // Try /etc/os-release first (most modern systems)
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        if let Some(distro) = parse_os_release(&content) {
            return Ok(distro);
        }
    }
    
    // Try /etc/lsb-release
    if let Ok(content) = fs::read_to_string("/etc/lsb-release") {
        if let Some(distro) = parse_lsb_release(&content) {
            return Ok(distro);
        }
    }
    
    // Try specific distribution files
    let distro_files = vec![
        ("/etc/debian_version", "debian"),
        ("/etc/redhat-release", "rhel"),
        ("/etc/centos-release", "centos"),
        ("/etc/fedora-release", "fedora"),
        ("/etc/SuSE-release", "opensuse"),
        ("/etc/arch-release", "arch"),
        ("/etc/gentoo-release", "gentoo"),
        ("/etc/alpine-release", "alpine"),
        ("/etc/void-release", "void"),
        ("/etc/slackware-version", "slackware"),
    ];
    
    for (file, distro) in distro_files {
        if fs::metadata(file).is_ok() {
            return Ok(distro.to_string());
        }
    }
    
    // Check for macOS
    if cfg!(target_os = "macos") {
        return Ok("macos".to_string());
    }
    
    // Check for FreeBSD
    if cfg!(target_os = "freebsd") {
        return Ok("freebsd".to_string());
    }
    
    // Check for OpenBSD
    if cfg!(target_os = "openbsd") {
        return Ok("openbsd".to_string());
    }
    
    // Check for NetBSD
    if cfg!(target_os = "netbsd") {
        return Ok("netbsd".to_string());
    }
    
    Ok("unknown".to_string())
}

fn parse_os_release(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.starts_with("ID=") {
            let id = line.trim_start_matches("ID=").trim_matches('"');
            return Some(normalize_distro_name(id));
        }
    }
    None
}

fn parse_lsb_release(content: &str) -> Option<String> {
    for line in content.lines() {
        if line.starts_with("DISTRIB_ID=") {
            let id = line.trim_start_matches("DISTRIB_ID=").trim_matches('"');
            return Some(normalize_distro_name(id));
        }
    }
    None
}

fn normalize_distro_name(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "ubuntu" => "ubuntu".to_string(),
        "debian" => "debian".to_string(),
        "arch" | "archlinux" => "arch".to_string(),
        "fedora" => "fedora".to_string(),
        "centos" => "centos".to_string(),
        "rhel" | "redhat" => "rhel".to_string(),
        "opensuse" | "suse" => "opensuse".to_string(),
        "gentoo" => "gentoo".to_string(),
        "alpine" => "alpine".to_string(),
        "void" => "void".to_string(),
        "slackware" => "slackware".to_string(),
        "manjaro" => "manjaro".to_string(),
        "mint" | "linuxmint" => "mint".to_string(),
        "elementary" => "elementary".to_string(),
        "pop" | "pop_os" => "pop".to_string(),
        "kali" => "kali".to_string(),
        "nixos" => "nixos".to_string(),
        "endeavouros" => "endeavouros".to_string(),
        "zorin" => "zorin".to_string(),
        "deepin" => "deepin".to_string(),
        "solus" => "solus".to_string(),
        "garuda" => "garuda".to_string(),
        _ => name.to_lowercase(),
    }
}
