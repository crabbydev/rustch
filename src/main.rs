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
    
    /// Minimal output (less information)
    #[arg(short, long)]
    minimal: bool,
    
    /// Show all available information
    #[arg(short = 'A', long)]
    all: bool,
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
    
    // Prepare info lines based on mode
    let info_lines = if cli.minimal {
        get_minimal_info(&system_info)
    } else if cli.all {
        get_all_info(&system_info)
    } else {
        get_default_info(&system_info)
    };
    
    // Display the output
    display_output(&ascii_lines, &info_lines, &theme, cli.no_color);
}

fn get_minimal_info(info: &SystemInfo) -> Vec<String> {
    vec![
        format!("OS: {}", info.os_name),
        format!("Host: {}", info.hostname),
        format!("Kernel: {}", info.kernel_version),
        format!("CPU: {}", info.cpu),
        format!("Memory: {}", info.memory),
    ]
}

fn get_default_info(info: &SystemInfo) -> Vec<String> {
    let mut info_lines = vec![
        format!("OS: {}", info.os_name),
        format!("Host: {}", info.hostname),
        format!("Kernel: {}", info.kernel_version),
        format!("Uptime: {}", info.uptime),
        format!("Shell: {}", info.shell),
    ];
    
    // Only show DE/WM if they're not "Unknown"
    if info.desktop_environment != "Unknown" {
        info_lines.push(format!("DE: {}", info.desktop_environment));
    }
    
    if info.window_manager != "Unknown" && info.window_manager != info.desktop_environment {
        info_lines.push(format!("WM: {}", info.window_manager));
    }
    
    if info.terminal != "Unknown" {
        info_lines.push(format!("Terminal: {}", info.terminal));
    }
    
    info_lines.extend(vec![
        format!("CPU: {}", info.cpu),
        format!("Memory: {}", info.memory),
    ]);
    
    if info.package_count != "Unknown" {
        info_lines.push(format!("Packages: {}", info.package_count));
    }
    
    info_lines
}

fn get_all_info(info: &SystemInfo) -> Vec<String> {
    vec![
        format!("OS: {}", info.os_name),
        format!("Host: {}", info.hostname),
        format!("Kernel: {}", info.kernel_version),
        format!("Uptime: {}", info.uptime),
        format!("Packages: {}", info.package_count),
        format!("Shell: {}", info.shell),
        format!("DE: {}", info.desktop_environment),
        format!("WM: {}", info.window_manager),
        format!("Terminal: {}", info.terminal),
        format!("CPU: {}", info.cpu),
        format!("Memory: {}", info.memory),
    ]
}

fn display_output(ascii_lines: &[String], info_lines: &[String], theme: &colors::ThemeColors, no_color: bool) {
    // Calculate the maximum width of ASCII art for proper alignment
    let max_ascii_width = ascii_lines
        .iter()
        .map(|line| strip_ansi_codes(line).chars().count())
        .max()
        .unwrap_or(0);
    
    let max_lines = ascii_lines.len().max(info_lines.len());
    let padding_base = if max_ascii_width > 0 { 4 } else { 0 }; // Base padding between ASCII and info
    
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
        
        // Calculate dynamic padding
        let ascii_display_width = strip_ansi_codes(&ascii_part).chars().count();
        let padding_needed = if max_ascii_width > ascii_display_width {
            max_ascii_width - ascii_display_width + padding_base
        } else {
            padding_base
        };
        
        let padding = " ".repeat(padding_needed);
        
        // Print the line
        if !ascii_part.is_empty() || !info_part.is_empty() {
            print!("{}", ascii_part);
            if !info_part.is_empty() {
                print!("{}{}", padding, info_part);
            }
            println!();
        }
    }
}

fn format_info_line(line: &str, theme: &colors::ThemeColors) -> String {
    if let Some(colon_pos) = line.find(':') {
        let (label, value) = line.split_at(colon_pos);
        let value_part = &value[1..]; // Remove the colon
        
        // Handle special cases for better formatting
        let formatted_value = if value_part.trim().is_empty() || value_part.trim() == "Unknown" {
            " N/A".dimmed().to_string()
        } else {
            format!(" {}", value_part.color(theme.text))
        };
        
        format!("{}{}{}", 
            label.color(theme.primary).bold(),
            ":".color(theme.secondary),
            formatted_value
        )
    } else {
        line.color(theme.text).to_string()
    }
}

fn strip_ansi_codes(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Handle ANSI escape sequences
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Skip until we find a letter (the command)
                while let Some(c) = chars.next() {
                    if c.is_ascii_alphabetic() || c == 'm' {
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
