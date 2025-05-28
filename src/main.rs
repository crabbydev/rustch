use clap::Parser;
use colored::*;
use std::process;

mod ascii;
mod colors;
mod distro;
mod system;

use ascii::get_ascii_art;
use colors::get_theme_colors;
use distro::detect_distribution;
use system::SystemInfo;

#[derive(Parser)]
#[command(name = "rustch")]
#[command(about = "A lightning-fast Rust-based system information tool with ASCII art logos")]
#[command(version = "1.0.0")]
struct Cli {
    /// Disable ASCII art
    #[arg(short = 'a', long)]
    no_art: bool,
    
    /// Disable colors
    #[arg(short = 'c', long)]
    no_color: bool,
    
    /// Force specific distribution detection
    #[arg(short, long)]
    distro: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    
    // Detect the distribution
    let distro = if let Some(forced_distro) = cli.distro {
        forced_distro
    } else {
        match detect_distribution() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error detecting distribution: {}", e);
                "unknown".to_string()
            }
        }
    };
    
    // Get system information
    let system_info = match SystemInfo::new() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error gathering system information: {}", e);
            process::exit(1);
        }
    };
    
    // Get ASCII art and colors
    let ascii_lines = if !cli.no_art {
        get_ascii_art(&distro)
    } else {
        vec![]
    };
    
    let theme = get_theme_colors(&distro);
    
    // Prepare info lines
    let info_lines = vec![
        format!("OS: {}", system_info.os_name),
        format!("Host: {}", system_info.hostname),
        format!("Kernel: {}", system_info.kernel_version),
        format!("Uptime: {}", system_info.uptime),
        format!("Packages: {}", system_info.package_count),
        format!("Shell: {}", system_info.shell),
        format!("DE: {}", system_info.desktop_environment),
        format!("WM: {}", system_info.window_manager),
        format!("Terminal: {}", system_info.terminal),
        format!("CPU: {}", system_info.cpu),
        format!("Memory: {}", system_info.memory),
    ];
    
    // Display the output
    display_output(&ascii_lines, &info_lines, &theme, cli.no_color);
}

fn display_output(ascii_lines: &[String], info_lines: &[String], theme: &colors::ThemeColors, no_color: bool) {
    let max_ascii_width = ascii_lines.iter().map(|line| strip_ansi_codes(line).len()).max().unwrap_or(0) + 4; // Add padding
    let max_lines = ascii_lines.len().max(info_lines.len());
    
    for i in 0..max_lines {
        let ascii_part = if i < ascii_lines.len() {
            if no_color {
                strip_ansi_codes(&ascii_lines[i])
            } else {
                ascii_lines[i].clone()
            }
        } else {
            String::new()
        };
        
        let info_part = if i < info_lines.len() {
            if no_color {
                info_lines[i].clone()
            } else {
                format_info_line(&info_lines[i], theme)
            }
        } else {
            String::new()
        };
        
        // Calculate proper spacing
        let ascii_display_width = strip_ansi_codes(&ascii_part).len();
        let padding_needed = if max_ascii_width > ascii_display_width {
            max_ascii_width - ascii_display_width
        } else {
            4 // Minimum padding
        };
        
        let padding = " ".repeat(padding_needed);
        
        if !ascii_part.is_empty() || !info_part.is_empty() {
            println!("{}{}{}", ascii_part, padding, info_part);
        }
    }
}

fn format_info_line(line: &str, theme: &colors::ThemeColors) -> String {
    if let Some(colon_pos) = line.find(':') {
        let (label, value) = line.split_at(colon_pos);
        format!("{}{}{}", 
            label.color(theme.primary),
            ":".color(theme.secondary),
            value.color(theme.text)
        )
    } else {
        line.color(theme.text).to_string()
    }
}

fn strip_ansi_codes(input: &str) -> String {
    // Simple ANSI code stripping - removes color codes
    let mut result = String::new();
    let mut chars = input.chars();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence
            if chars.next() == Some('[') {
                while let Some(c) = chars.next() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}
