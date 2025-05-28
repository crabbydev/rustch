#!/bin/bash

# rustch installer script
# This script installs the rustch system information tool

set -e

# Configuration
REPO_URL="https://github.com/rustch-project/rustch"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="rustch"
TMP_DIR="/tmp/rustch-install"
VERSION="latest"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root for system-wide installation
check_privileges() {
    if [[ $EUID -eq 0 ]]; then
        log_info "Running as root - installing system-wide"
        INSTALL_DIR="/usr/local/bin"
    else
        log_info "Running as user - installing to ~/.local/bin"
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    fi
}

# Check if required tools are available
check_dependencies() {
    local deps=("curl" "tar")
    local missing=()
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        log_error "Missing required dependencies: ${missing[*]}"
        log_info "Please install the missing dependencies and try again"
        exit 1
    fi
}

# Detect system architecture and OS
detect_system() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$arch" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        armv7l)
            arch="armv7"
            ;;
        *)
            log_error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac
    
    case "$os" in
        linux)
            PLATFORM="$arch-unknown-linux-gnu"
            ;;
        darwin)
            PLATFORM="$arch-apple-darwin"
            ;;
        freebsd)
            PLATFORM="$arch-unknown-freebsd"
            ;;
        *)
            log_error "Unsupported operating system: $os"
            exit 1
            ;;
    esac
    
    log_info "Detected platform: $PLATFORM"
}

# Check if Rust is installed and install if needed
check_rust() {
    if command -v cargo &> /dev/null; then
        log_info "Rust toolchain found"
        return 0
    else
        log_warning "Rust toolchain not found"
        read -p "Would you like to install Rust? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            install_rust
        else
            log_error "Rust is required to build rustch"
            exit 1
        fi
    fi
}

# Install Rust using rustup
install_rust() {
    log_info "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    log_success "Rust installed successfully"
}

# Download and build from source
build_from_source() {
    log_info "Building rustch from source..."
    
    # Create temporary directory
    mkdir -p "$TMP_DIR"
    cd "$TMP_DIR"
    
    # Download source code
    log_info "Downloading source code..."
    curl -L "${REPO_URL}/archive/refs/heads/main.tar.gz" -o rustch.tar.gz
    tar -xzf rustch.tar.gz
    cd rustch-main

    
    # Build the project
    log_info "Building rustch..."
    cargo build --release
    
    # Install the binary
    log_info "Installing rustch to $INSTALL_DIR"
    cp target/release/rustch "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/rustch"
    
    # Clean up
    cd /
    rm -rf "$TMP_DIR"
    
    log_success "rustch built and installed successfully"
}

# Try to download pre-built binary
download_binary() {
    local binary_url="${REPO_URL}/releases/latest/download/rustch-${PLATFORM}"
    
    log_info "Attempting to download pre-built binary..."
    log_info "URL: $binary_url"
    
    if curl -L --fail "$binary_url" -o "$INSTALL_DIR/rustch" 2>/dev/null; then
        chmod +x "$INSTALL_DIR/rustch"
        log_success "Pre-built binary downloaded and installed"
        return 0
    else
        log_warning "Pre-built binary not available for $PLATFORM"
        return 1
    fi
}

# Add install directory to PATH if not already present
update_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        log_info "Adding $INSTALL_DIR to PATH"
        
        # Determine which shell config file to update
        if [[ -n "$ZSH_VERSION" ]]; then
            shell_config="$HOME/.zshrc"
        elif [[ -n "$BASH_VERSION" ]]; then
            shell_config="$HOME/.bashrc"
        else
            shell_config="$HOME/.profile"
        fi
        
        echo "" >> "$shell_config"
        echo "# Added by rfetch installer" >> "$shell_config"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$shell_config"
        
        log_info "Added PATH export to $shell_config"
        log_warning "Please restart your shell or run: source $shell_config"
    fi
}

# Verify installation
verify_installation() {
    if command -v rustch &> /dev/null; then
        log_success "rustch installed successfully!"
        log_info "Version: $(rustch --version)"
        log_info "Location: $(which rustch)"
        log_info "Run 'rustch' to see your system information"
    else
        log_error "Installation verification failed"
        log_info "rustch binary is at: $INSTALL_DIR/rustch"
        log_info "Make sure $INSTALL_DIR is in your PATH"
        exit 1
    fi
}

# Uninstall function
uninstall() {
    log_info "Uninstalling rustch..."
    
    # Remove binary from common locations
    local locations=("/usr/local/bin/rustch" "$HOME/.local/bin/rustch" "/usr/bin/rustch")
    local removed=false
    
    for location in "${locations[@]}"; do
        if [[ -f "$location" ]]; then
            rm "$location"
            log_success "Removed $location"
            removed=true
        fi
    done
    
    if [[ "$removed" == true ]]; then
        log_success "rustch uninstalled successfully"
    else
        log_warning "rustch binary not found in common locations"
    fi
}

# Show help
show_help() {
    cat << EOF
rustch installer script

Usage: $0 [OPTIONS]

OPTIONS:
    --uninstall     Uninstall rustch
    --help, -h      Show this help message
    --force-build   Force building from source (skip binary download)

Examples:
    $0                  # Install rustch
    $0 --uninstall      # Uninstall rustch
    $0 --force-build    # Build from source

EOF
}

# Main installation function
main() {
    local force_build=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --uninstall)
                uninstall
                exit 0
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            --force-build)
                force_build=true
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    log_info "Starting rustch installation..."
    
    check_dependencies
    check_privileges
    detect_system
    
    # Try to download binary first, unless forced to build
    if [[ "$force_build" == false ]] && download_binary; then
        # Binary download successful
        :
    else
        # Fallback to building from source
        check_rust
        build_from_source
    fi
    
    # Update PATH if installing to user directory
    if [[ "$INSTALL_DIR" == "$HOME/.local/bin" ]]; then
        update_path
    fi
    
    verify_installation
    
    log_success "Installation complete!"
    echo
    log_info "Try running: rustch"
}

# Run main function with all arguments
main "$@"
