use colored::Color;

pub struct ThemeColors {
    pub primary: Color,
    pub secondary: Color,
    pub text: Color,
    pub accent: Color,
}

pub fn get_theme_colors(distro: &str) -> ThemeColors {
    match distro {
        "ubuntu" => ThemeColors {
            primary: Color::TrueColor { r: 233, g: 84, b: 32 },  // Ubuntu Orange
            secondary: Color::TrueColor { r: 119, g: 41, b: 83 }, // Ubuntu Purple
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "debian" => ThemeColors {
            primary: Color::TrueColor { r: 215, g: 7, b: 81 },   // Debian Red
            secondary: Color::TrueColor { r: 215, g: 7, b: 81 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "arch" => ThemeColors {
            primary: Color::TrueColor { r: 23, g: 147, b: 209 },  // Arch Blue
            secondary: Color::TrueColor { r: 23, g: 147, b: 209 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "fedora" => ThemeColors {
            primary: Color::TrueColor { r: 51, g: 74, b: 255 },   // Fedora Blue
            secondary: Color::TrueColor { r: 41, g: 65, b: 114 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "centos" => ThemeColors {
            primary: Color::TrueColor { r: 137, g: 83, b: 171 },  // CentOS Purple
            secondary: Color::TrueColor { r: 137, g: 83, b: 171 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "opensuse" => ThemeColors {
            primary: Color::TrueColor { r: 115, g: 186, b: 37 },  // openSUSE Green
            secondary: Color::TrueColor { r: 35, g: 31, b: 32 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "gentoo" => ThemeColors {
            primary: Color::TrueColor { r: 84, g: 72, b: 122 },   // Gentoo Purple
            secondary: Color::TrueColor { r: 255, g: 255, b: 255 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "alpine" => ThemeColors {
            primary: Color::TrueColor { r: 13, g: 71, b: 161 },   // Alpine Blue
            secondary: Color::TrueColor { r: 13, g: 71, b: 161 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "manjaro" => ThemeColors {
            primary: Color::TrueColor { r: 52, g: 190, b: 91 },   // Manjaro Green
            secondary: Color::TrueColor { r: 52, g: 190, b: 91 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "mint" => ThemeColors {
            primary: Color::TrueColor { r: 135, g: 200, b: 61 },  // Mint Green
            secondary: Color::TrueColor { r: 135, g: 200, b: 61 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "freebsd" => ThemeColors {
            primary: Color::TrueColor { r: 204, g: 0, b: 0 },     // FreeBSD Red
            secondary: Color::TrueColor { r: 153, g: 0, b: 0 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "macos" => ThemeColors {
            primary: Color::TrueColor { r: 0, g: 122, b: 255 },   // macOS Blue
            secondary: Color::TrueColor { r: 88, g: 86, b: 214 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "void" => ThemeColors {
            primary: Color::TrueColor { r: 72, g: 123, b: 56 },   // Void Green
            secondary: Color::TrueColor { r: 72, g: 123, b: 56 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "kali" => ThemeColors {
            primary: Color::TrueColor { r: 43, g: 149, b: 237 },  // Kali Blue
            secondary: Color::TrueColor { r: 255, g: 255, b: 255 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "elementary" => ThemeColors {
            primary: Color::TrueColor { r: 100, g: 181, b: 246 }, // Elementary Blue
            secondary: Color::TrueColor { r: 255, g: 255, b: 255 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "pop" => ThemeColors {
            primary: Color::TrueColor { r: 255, g: 184, b: 17 },  // Pop Orange
            secondary: Color::TrueColor { r: 72, g: 185, b: 199 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "nixos" => ThemeColors {
            primary: Color::TrueColor { r: 126, g: 188, b: 255 }, // NixOS Blue
            secondary: Color::TrueColor { r: 126, g: 188, b: 255 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "slackware" => ThemeColors {
            primary: Color::TrueColor { r: 71, g: 108, b: 142 },  // Slackware Blue
            secondary: Color::TrueColor { r: 255, g: 255, b: 255 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "rhel" => ThemeColors {
            primary: Color::TrueColor { r: 204, g: 41, b: 29 },   // Red Hat Red
            secondary: Color::TrueColor { r: 204, g: 41, b: 29 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "openbsd" => ThemeColors {
            primary: Color::TrueColor { r: 255, g: 191, b: 0 },   // OpenBSD Yellow
            secondary: Color::TrueColor { r: 255, g: 191, b: 0 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "netbsd" => ThemeColors {
            primary: Color::TrueColor { r: 255, g: 139, b: 0 },   // NetBSD Orange
            secondary: Color::TrueColor { r: 255, g: 139, b: 0 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "endeavouros" => ThemeColors {
            primary: Color::TrueColor { r: 125, g: 115, b: 255 }, // EndeavourOS Purple
            secondary: Color::TrueColor { r: 255, g: 92, b: 87 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "zorin" => ThemeColors {
            primary: Color::TrueColor { r: 21, g: 149, b: 246 },  // Zorin Blue
            secondary: Color::TrueColor { r: 255, g: 255, b: 255 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "deepin" => ThemeColors {
            primary: Color::TrueColor { r: 0, g: 150, b: 136 },   // Deepin Teal
            secondary: Color::TrueColor { r: 255, g: 255, b: 255 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "solus" => ThemeColors {
            primary: Color::TrueColor { r: 24, g: 144, b: 255 },  // Solus Blue
            secondary: Color::TrueColor { r: 255, g: 255, b: 255 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        "garuda" => ThemeColors {
            primary: Color::TrueColor { r: 255, g: 64, b: 129 },  // Garuda Pink
            secondary: Color::TrueColor { r: 63, g: 81, b: 181 },
            text: Color::White,
            accent: Color::TrueColor { r: 255, g: 255, b: 255 },
        },
        _ => ThemeColors {
            primary: Color::Cyan,
            secondary: Color::Blue,
            text: Color::White,
            accent: Color::Yellow,
        },
    }
}
