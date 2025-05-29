use colored::*;
use crate::colors::get_theme_colors;

pub fn get_ascii_art(distro: &str) -> Vec<String> {
    let theme = get_theme_colors(distro);
    
    match distro {
        "ubuntu" => ubuntu_ascii(&theme),
        "debian" => debian_ascii(&theme),
        "arch" => arch_ascii(&theme),
        "fedora" => fedora_ascii(&theme),
        "centos" => centos_ascii(&theme),
        "opensuse" => opensuse_ascii(&theme),
        "gentoo" => gentoo_ascii(&theme),
        "alpine" => alpine_ascii(&theme),
        "manjaro" => manjaro_ascii(&theme),
        "mint" => mint_ascii(&theme),
        "freebsd" => freebsd_ascii(&theme),
        "macos" => macos_ascii(&theme),
        "void" => void_ascii(&theme),
        "kali" => kali_ascii(&theme),
        "elementary" => elementary_ascii(&theme),
        "pop" => pop_ascii(&theme),
        "nixos" => nixos_ascii(&theme),
        "slackware" => slackware_ascii(&theme),
        "rhel" => rhel_ascii(&theme),
        "openbsd" => openbsd_ascii(&theme),
        "netbsd" => netbsd_ascii(&theme),
        "endeavouros" => endeavouros_ascii(&theme),
        "zorin" => zorin_ascii(&theme),
        "deepin" => deepin_ascii(&theme),
        "solus" => solus_ascii(&theme),
        "garuda" => garuda_ascii(&theme),
        _ => default_ascii(&theme),
    }
}

fn ubuntu_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "         _".color(theme.primary).to_string(),
        "     ---(_)".color(theme.primary).to_string(),
        " _/  ---  \\".color(theme.primary).to_string(),
        "(_) |   |".color(theme.primary).to_string(),
        "  \\  --- _/".color(theme.primary).to_string(),
        "     ---(_)".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}buntu", "U".color(theme.secondary)),
    ]
}

fn debian_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "  _____".color(theme.primary).to_string(),
        " /  __ \\".color(theme.primary).to_string(),
        "|  /    |".color(theme.primary).to_string(),
        "|  \\___-".color(theme.primary).to_string(),
        "-_".color(theme.primary).to_string(),
        "  --_".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}ebian", "D".color(theme.secondary)),
    ]
}

fn arch_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "       /\\".color(theme.primary).to_string(),
        "      /  \\".color(theme.primary).to_string(),
        "     /\\   \\".color(theme.primary).to_string(),
        "    /      \\".color(theme.primary).to_string(),
        "   /   ,,   \\".color(theme.primary).to_string(),
        "  /   |  |  -\\".color(theme.primary).to_string(),
        " /_-''    ''-_\\".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}rch Linux", "A".color(theme.secondary)),
    ]
}

fn fedora_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "             .',;::::;,'.".color(theme.primary).to_string(),
        "         .';;;;;;;;;;;;;,'.".color(theme.primary).to_string(),
        "      .,;;;;;;;;;;;;;;;;;;;,.".color(theme.primary).to_string(),
        "    .:;;;;;;;;;;;;;;;;;;;;;;;:.".color(theme.primary).to_string(),
        "   .;;;;;;;;;;;;;;;;;;;;;;;;;,.".color(theme.primary).to_string(),
        "  ;;;;;;;;;;;;;;;;;;;;;;;;;;;;".color(theme.primary).to_string(),
        " ;;;;;;;;  ;;;;;  ;;;;;;;;".color(theme.primary).to_string(),
        " ;;;;;;;;;;;;;;;;;;;;;;//////".color(theme.primary).to_string(),
        " ;;;;;;;;;;;;;;;;;;;;;;//////".color(theme.primary).to_string(),
        " ;;;;;;;;  ;;;;;  ;;;;;///".color(theme.primary).to_string(),
        "  ;;;;;;;;;;;;;;;;;;;;;;;;".color(theme.primary).to_string(),
        "   ';;;;;;;;;;;;;;;;;;;;;,'".color(theme.primary).to_string(),
        "    ':;;;;;;;;;;;;;;;;;;;:'".color(theme.primary).to_string(),
        "      ',:;;;;;;;;;;;;;;;,'".color(theme.primary).to_string(),
        "         ',;;;;;;;;;,'".color(theme.primary).to_string(),
        "             ',;;;,'".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}edora", "F".color(theme.secondary)),
    ]
}

fn centos_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "                 ..                   ".color(theme.primary).to_string(),
        "               .PLTJ.                 ".color(theme.primary).to_string(),
        "              <><><><>                ".color(theme.primary).to_string(),
        "     KKSSV' 4KKK LJ KKKL.'VSSKK      ".color(theme.primary).to_string(),
        "     KKV' 4KKKKK LJ KKKKAL 'VKK      ".color(theme.primary).to_string(),
        "     V' ' 'VKKKK LJ KKKKV' ' 'V      ".color(theme.primary).to_string(),
        "     .4MA.' 'VKK LJ KKV' '.4Mb.      ".color(theme.primary).to_string(),
        "   . KKKKKA.' 'V LJ V' '.4KKKKK .    ".color(theme.primary).to_string(),
        " .4D KKKKKKKA.'' LJ ''.4KKKKKKK FA4. ".color(theme.primary).to_string(),
        "<QDD ++++++++++++  ++++++++++++  XDD>".color(theme.primary).to_string(),
        " 'VD KKKKKKKK'.. LJ ..'KKKKKKKK FV'  ".color(theme.primary).to_string(),
        "   ' VKKKKV'. .4 LJ K. .'VKKKV '     ".color(theme.primary).to_string(),
        "      'VK'. .4KK LJ KKA. .'KV'       ".color(theme.primary).to_string(),
        "     A. . .4KKKK LJ KKKKA. . .4      ".color(theme.primary).to_string(),
        "     KKA' 'KKKKK LJ KKKKK' 'AKK      ".color(theme.primary).to_string(),
        "     KKSSA. VKKK LJ KKKV .ASSKK      ".color(theme.primary).to_string(),
        "              <><><><>                ".color(theme.primary).to_string(),
        "               'MKKM'                 ".color(theme.primary).to_string(),
        "                 ''                   ".color(theme.primary).to_string(),
        "".to_string(),
        format!("               {}entOS", "C".color(theme.secondary)),
    ]
}

fn opensuse_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "  _______".color(theme.primary).to_string(),
        " |       |".color(theme.primary).to_string(),
        " |   ,   |".color(theme.primary).to_string(),
        " |       |".color(theme.primary).to_string(),
        " |  _____|".color(theme.primary).to_string(),
        " |       |".color(theme.primary).to_string(),
        " |_______|".color(theme.primary).to_string(),
        "".to_string(),
        format!("open{}USE", "S".color(theme.secondary)),
    ]
}

fn gentoo_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "         -/oyddmdhs+:.".color(theme.primary).to_string(),
        "     -odNMMMMMMMMNNmhy+-".color(theme.primary).to_string(),
        "   -yNMMMMMMMMMMMNNNmmdhy+-".color(theme.primary).to_string(),
        " `omMMMMMMMMMMMMNmdmmmmddhhy+`".color(theme.primary).to_string(),
        " omMMMMMMMMMMMMNhhyyyohmdddhhhdo".color(theme.primary).to_string(),
        ".ydMMMMMMMMMMdhs++so/smdddhhhhdm+`".color(theme.primary).to_string(),
        " oyhdmNMMMMMMMNdyooydmddddhhhhyhNd.".color(theme.primary).to_string(),
        "  :oyhhdNNMMMMMMMNNNmmdddhhhhhyymMh".color(theme.primary).to_string(),
        "   .:+sydNMMMMMNNNmmmdddhhhhhhmMmy".color(theme.primary).to_string(),
        "      /mMMMMMMNNNmmmdddhhhhhmMNhs:".color(theme.primary).to_string(),
        "     `oNMMMMMMMNNNmmmddddhhdmMNhs+`".color(theme.primary).to_string(),
        "   `sNMMMMMMMMNNNmmmdddddmNMmhs/.".color(theme.primary).to_string(),
        "  /NMMMMMMMMNNNNmmmdddmNMNdso:`".color(theme.primary).to_string(),
        "+MMMMMMMNNNNNmmmmdmNMNdso/-".color(theme.primary).to_string(),
        "yMMNNNNNNNmmmmmNNMmhs+/-`".color(theme.primary).to_string(),
        "/hMMNNNNNNNNMNdhs++/-`".color(theme.primary).to_string(),
        "`/ohdmmddhys+++/:.`".color(theme.primary).to_string(),
        "  `-//////:--.".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}entoo", "G".color(theme.secondary)),
    ]
}

fn alpine_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "       /\\ /\\".color(theme.primary).to_string(),
        "      /  V  \\".color(theme.primary).to_string(),
        "     / /   \\ \\".color(theme.primary).to_string(),
        "    /_/     \\_\\".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}lpine Linux", "A".color(theme.secondary)),
    ]
}

fn manjaro_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "||||||||| ||||".color(theme.primary).to_string(),
        "||||||||| ||||".color(theme.primary).to_string(),
        "||||      ||||".color(theme.primary).to_string(),
        "|||| |||| ||||".color(theme.primary).to_string(),
        "|||| |||| ||||".color(theme.primary).to_string(),
        "|||| |||| ||||".color(theme.primary).to_string(),
        "|||| |||| ||||".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}anjaro", "M".color(theme.secondary)),
    ]
}

fn mint_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "             ...-:::::-...".color(theme.primary).to_string(),
        "          .-MMMMMMMMMMMMMMM-.".color(theme.primary).to_string(),
        "      .-MMMM`.:/++++/:.`MMMM-.".color(theme.primary).to_string(),
        "    .:MMMM.:+++++++++++++:MMMM:.".color(theme.primary).to_string(),
        "  .:MMMM-:++++++++++++++++-MMMM:.".color(theme.primary).to_string(),
        " ./MMMM-:++++++++++++++++++-MMMM/.".color(theme.primary).to_string(),
        " /MMMM-:+++++++++++++++++++-MMMM/".color(theme.primary).to_string(),
        "`MMMM-:+++++++++++++++++++-MMMM`".color(theme.primary).to_string(),
        "`MMMM-:+++++++++++++++++++-MMMM`".color(theme.primary).to_string(),
        " /MMMM-:+++++++++++++++++++-MMMM/".color(theme.primary).to_string(),
        " ./MMMM-:++++++++++++++++++-MMMM/.".color(theme.primary).to_string(),
        "  .:MMMM-:++++++++++++++++-MMMM:.".color(theme.primary).to_string(),
        "    .:MMMM.:+++++++++++++:MMMM:.".color(theme.primary).to_string(),
        "      .-MMMM`.:/++++/:.`MMMM-.".color(theme.primary).to_string(),
        "          .-MMMMMMMMMMMMMMM-.".color(theme.primary).to_string(),
        "             ...-:::::-...".color(theme.primary).to_string(),
        "".to_string(),
        format!("Linux {}int", "M".color(theme.secondary)),
    ]
}

fn freebsd_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "```                        `".color(theme.primary).to_string(),
        "  s` `.....---.......--.```   -/".color(theme.primary).to_string(),
        "  +o   .--`         /y:`      +.".color(theme.primary).to_string(),
        "   yo`:.            :o      `+-".color(theme.primary).to_string(),
        "    y/               -/`   -o/".color(theme.primary).to_string(),
        "   .-                  ::/sy+:.".color(theme.primary).to_string(),
        "   /                     `--  /".color(theme.primary).to_string(),
        "  `:                          :`".color(theme.primary).to_string(),
        "  `:                          :`".color(theme.primary).to_string(),
        "   /                          /".color(theme.primary).to_string(),
        "   .-                        -.".color(theme.primary).to_string(),
        "    --                      -.".color(theme.primary).to_string(),
        "     `:.                  .:`".color(theme.primary).to_string(),
        "       .--             `--.".color(theme.primary).to_string(),
        "          .---.....----.".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}reeBSD", "F".color(theme.secondary)),
    ]
}

fn macos_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "                    'c.".color(theme.primary).to_string(),
        "                 ,xNMM.".color(theme.primary).to_string(),
        "               .OMMMMo".color(theme.primary).to_string(),
        "               OMMM0,".color(theme.primary).to_string(),
        "     .;loddo:' loolloddol;.".color(theme.primary).to_string(),
        "   cKMMMMMMMMMMNWMMMMMMMMMM0:".color(theme.primary).to_string(),
        " .KMMMMMMMMMMMMMMMMMMMMMMMWd.".color(theme.primary).to_string(),
        " XMMMMMMMMMMMMMMMMMMMMMMMX.".color(theme.primary).to_string(),
        ";MMMMMMMMMMMMMMMMMMMMMMMM:".color(theme.primary).to_string(),
        ":MMMMMMMMMMMMMMMMMMMMMMMM:".color(theme.primary).to_string(),
        ".MMMMMMMMMMMMMMMMMMMMMMMMX.".color(theme.primary).to_string(),
        " kMMMMMMMMMMMMMMMMMMMMMMMMWd.".color(theme.primary).to_string(),
        " .XMMMMMMMMMMMMMMMMMMMMMMMMMMk".color(theme.primary).to_string(),
        "  .XMMMMMMMMMMMMMMMMMMMMMMMMK.".color(theme.primary).to_string(),
        "    kMMMMMMMMMMMMMMMMMMMMMMd".color(theme.primary).to_string(),
        "     ;KMMMMMMMWXXWMMMMMMMk.".color(theme.primary).to_string(),
        "       .cooc,.    .,coo:.".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}acOS", "m".color(theme.secondary)),
    ]
}

fn void_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "    _______".color(theme.primary).to_string(),
        " _ \\______ -".color(theme.primary).to_string(),
        "| \\  ___  \\ |".color(theme.primary).to_string(),
        "| | /   \\ | |".color(theme.primary).to_string(),
        "| | \\___/ | |".color(theme.primary).to_string(),
        "| \\______ \\_|".color(theme.primary).to_string(),
        " -_______\\".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}oid Linux", "V".color(theme.secondary)),
    ]
}

fn kali_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "..............".color(theme.primary).to_string(),
        "            ..,;:ccc,.".color(theme.primary).to_string(),
        "          ......''';lxO.".color(theme.primary).to_string(),
        ".....''''..........,:ld;".color(theme.primary).to_string(),
        "           .';;;:::;,,.x,".color(theme.primary).to_string(),
        "      ..'''.            0Xxoc:,.  ...".color(theme.primary).to_string(),
        "  ....                ,ONkc;,;cokOdc',.".color(theme.primary).to_string(),
        " .                   OMo           ':ddo.".color(theme.primary).to_string(),
        "                    dMc               :OO;".color(theme.primary).to_string(),
        "                    0M.                 .:o.".color(theme.primary).to_string(),
        "                    ;Wd".color(theme.primary).to_string(),
        "                     ;XO,".color(theme.primary).to_string(),
        "                       ,d0Odlc;,..".color(theme.primary).to_string(),
        "                           ..',;:cdOOd::,.".color(theme.primary).to_string(),
        "                                    .:d;.':;.".color(theme.primary).to_string(),
        "                                       'd,  .'".color(theme.primary).to_string(),
        "                                         ;l   ..".color(theme.primary).to_string(),
        "                                          .o".color(theme.primary).to_string(),
        "                                            c".color(theme.primary).to_string(),
        "                                            .'".color(theme.primary).to_string(),
        "                                             .".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}ali Linux", "K".color(theme.secondary)),
    ]
}

fn elementary_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "         eeeeeeeeeeeeeeeee".color(theme.primary).to_string(),
        "      eeeeeeeeeeeeeeeeeeeeeee".color(theme.primary).to_string(),
        "    eeeeeeeeeeeeeeeeeeeeeeeeee".color(theme.primary).to_string(),
        "   eeeeee  eeeeeeeeee   eeeee".color(theme.primary).to_string(),
        "  eeeeee    eeeee           ee".color(theme.primary).to_string(),
        " eeeeee     eeeee            e".color(theme.primary).to_string(),
        " eeeee       eee".color(theme.primary).to_string(),
        " eeee        ee".color(theme.primary).to_string(),
        " eeee         e".color(theme.primary).to_string(),
        " eee".color(theme.primary).to_string(),
        " eee".color(theme.primary).to_string(),
        " ee".color(theme.primary).to_string(),
        " ee".color(theme.primary).to_string(),
        " ee".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}lementary OS", "E".color(theme.secondary)),
    ]
}

fn pop_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "             /////////////".color(theme.primary).to_string(),
        "         /////////////////////".color(theme.primary).to_string(),
        "      ///////*767////////////////".color(theme.primary).to_string(),
        "    //////7676767676*//////////////".color(theme.primary).to_string(),
        "   /////76767//7676767//////////////".color(theme.primary).to_string(),
        "  /////767676///*76767///////////////".color(theme.primary).to_string(),
        " ///////767676///76767.//////////////".color(theme.primary).to_string(),
        "/////////767676//76767///////////////".color(theme.primary).to_string(),
        "//////////76767676767////////////////".color(theme.primary).to_string(),
        "///////////76767676/////////////////".color(theme.primary).to_string(),
        "////////////,7676,//////////////////".color(theme.primary).to_string(),
        "/////////////*7676//////////////////".color(theme.primary).to_string(),
        "//////////////7676//////////////////".color(theme.primary).to_string(),
        "//////////////7676//////////////////".color(theme.primary).to_string(),
        "//////////////7676//////////////////".color(theme.primary).to_string(),
        "//////////////7676//////////////////".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}op!_OS", "P".color(theme.secondary)),
    ]
}

fn nixos_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "          ::::.    ':::::     ::::'".color(theme.primary).to_string(),
        "          ':::::. '::::::::: .:::::'".color(theme.primary).to_string(),
        "            ::::::'::::::::::::::::'".color(theme.primary).to_string(),
        "            '::::::::::::::::::::::'".color(theme.primary).to_string(),
        "             :::::::::::::::::::::'".color(theme.primary).to_string(),
        "             :::::::::::::::::::::'".color(theme.primary).to_string(),
        "           ::::::::::::::::::::'".color(theme.primary).to_string(),
        "          ::::::::::::::::::'".color(theme.primary).to_string(),
        "         ::::::::::::::::::'".color(theme.primary).to_string(),
        "        ::::::::::::::::'".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}ixOS", "N".color(theme.secondary)),
    ]
}

fn slackware_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "                  ::::::".color(theme.primary).to_string(),
        "             ::::::::::::::::::".color(theme.primary).to_string(),
        "          ::::::::::::::::::::::::".color(theme.primary).to_string(),
        "        ::::::::cllcccccllllllll::".color(theme.primary).to_string(),
        "     :::::::::lc               dc::".color(theme.primary).to_string(),
        "    ::::::::cl   clllccllll    oc:".color(theme.primary).to_string(),
        "   :::::::::o    lc    dc       co".color(theme.primary).to_string(),
        "  ::::::::::o     cccclc         o".color(theme.primary).to_string(),
        " :::::::::co        l          l::".color(theme.primary).to_string(),
        " :::::::::ccl                   :::".color(theme.primary).to_string(),
        "::::::::::::::lccclllllllllllll::::".color(theme.primary).to_string(),
        "::::::::::::::::::::::::::::::::::".color(theme.primary).to_string(),
        "::::::::::::::::::::::::::::::::::".color(theme.primary).to_string(),
        " ::::::::::::::::::::::::::::::::".color(theme.primary).to_string(),
        "  :::::::::::::::::::::::::::::: ".color(theme.primary).to_string(),
        "   :::::::::::::::::::::::::::: ".color(theme.primary).to_string(),
        "     ::::::::::::::::::::::::  ".color(theme.primary).to_string(),
        "        :::::::::::::::::::: ".color(theme.primary).to_string(),
        "             ::::::::::::".color(theme.primary).to_string(),
        "                 :::::: ".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}lackware", "S".color(theme.secondary)),
    ]
}

fn rhel_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "           .MMM..:MMMMMMM".color(theme.primary).to_string(),
        "          MMMMMMMMMMMMMMMMMM".color(theme.primary).to_string(),
        "          MMMMMMMMMMMMMMMMMM.".color(theme.primary).to_string(),
        "         MMMMMMMMMMMMMMMMMMMM".color(theme.primary).to_string(),
        "        ,MMMMMMMMMMMMMMMMMMMM".color(theme.primary).to_string(),
        "        MMMMMMMMMMMMMMMMMMMM'".color(theme.primary).to_string(),
        "  .MMMM'`MMMMMMMMMMMMMMMMMM".color(theme.primary).to_string(),
        " MMMMMM/ `MMMMMMMMMMMMMMMM".color(theme.primary).to_string(),
        " `MMMM'    `MMMMMMMMMMMMM".color(theme.primary).to_string(),
        "  MMM'       `MMMMMMMMM".color(theme.primary).to_string(),
        "   M'           `MMMMM".color(theme.primary).to_string(),
        "   '              `M".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}HEL", "R".color(theme.secondary)),
    ]
}

fn openbsd_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "      _____".color(theme.primary).to_string(),
        "    \\-     -/".color(theme.primary).to_string(),
        " \\_/         \\_/".color(theme.primary).to_string(),
        " |               |".color(theme.primary).to_string(),
        " |   (__)   (__)   |".color(theme.primary).to_string(),
        " |    oo     oo    |".color(theme.primary).to_string(),
        " |    ()     ()    |".color(theme.primary).to_string(),
        " \\              /".color(theme.primary).to_string(),
        "  \\-___-___-___-/".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}penBSD", "O".color(theme.secondary)),
    ]
}

fn netbsd_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "                     `.-.".color(theme.primary).to_string(),
        "                    /--+:.".color(theme.primary).to_string(),
        "                   +:.  :+".color(theme.primary).to_string(),
        "                  .:-.  .-:".color(theme.primary).to_string(),
        "                    ..".color(theme.primary).to_string(),
        "       .:::::::::::::::::::::.".color(theme.primary).to_string(),
        "      .:::::::::::::::::::::::.".color(theme.primary).to_string(),
        "     /:::::::::::::::::::::::::\\".color(theme.primary).to_string(),
        "    |::::::::::::::::::::::::::|".color(theme.primary).to_string(),
        "    |::::::::::::::::::::::::::|".color(theme.primary).to_string(),
        "     \\:::::::::::::::::::::::::/".color(theme.primary).to_string(),
        "      ':::::::::::::::::::::::'".color(theme.primary).to_string(),
        "       ''''::::::::::::''''''".color(theme.primary).to_string(),
        "            '''''::''''''".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}etBSD", "N".color(theme.secondary)),
    ]
}

fn endeavouros_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "                       /\\".color(theme.primary).to_string(),
        "                      /  \\".color(theme.primary).to_string(),
        "                     /    \\".color(theme.primary).to_string(),
        "                    /      \\".color(theme.primary).to_string(),
        "                   /   /\\   \\".color(theme.primary).to_string(),
        "                  /   /  \\   \\".color(theme.primary).to_string(),
        "                 /   /    \\   \\".color(theme.primary).to_string(),
        "                /___/______\\___\\".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}ndeavourOS", "E".color(theme.secondary)),
    ]
}

fn zorin_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "        `osssssssssssssssssso`".color(theme.primary).to_string(),
        "       .osssssssssssssssssssso.".color(theme.primary).to_string(),
        "      .+oooooooooooooooooooooo+.".color(theme.primary).to_string(),
        "    `::::::::::::::::::::::::::`".color(theme.primary).to_string(),
        "   `:::::::::::::::::::::::::::`".color(theme.primary).to_string(),
        "  `:::::::::::::::::::::::::::::`".color(theme.primary).to_string(),
        " `:::::::::::::::::::::::::::::::`".color(theme.primary).to_string(),
        " .::::::::::::::::::::::::::::::.".color(theme.primary).to_string(),
        " .::::::::::::::::::::::::::::::.".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}orin OS", "Z".color(theme.secondary)),
    ]
}

fn deepin_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "             ........".color(theme.primary).to_string(),
        "         .';;;;;.  ccccc;".color(theme.primary).to_string(),
        "      .;;;;;;;;;.  ccccc;".color(theme.primary).to_string(),
        "    .;;;;;;;;;;'  ccccc;".color(theme.primary).to_string(),
        "   .,;;;;;;;;;;'  ccccc;".color(theme.primary).to_string(),
        "   ';;;;;;;;;;' ccccc;".color(theme.primary).to_string(),
        "   ';;;;;;;;;;,.'    ;".color(theme.primary).to_string(),
        "   ';;;;;;;;;'.      ;".color(theme.primary).to_string(),
        "   ';;;;;;;;;'       ;".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}eepin", "D".color(theme.secondary)),
    ]
}

fn solus_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "            e         e".color(theme.primary).to_string(),
        "           eee       eee".color(theme.primary).to_string(),
        "          eeeeeeeeeeeeee".color(theme.primary).to_string(),
        "         eeeeeeeeeeeeeeee".color(theme.primary).to_string(),
        "        eeeee".color(theme.primary).to_string(),
        "       eeeee".color(theme.primary).to_string(),
        "      eeeee".color(theme.primary).to_string(),
        "     eeeee".color(theme.primary).to_string(),
        "    eeeee".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}olus", "S".color(theme.secondary)),
    ]
}

fn garuda_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "                     .%@@@@@@%".color(theme.primary).to_string(),
        "                  .#@@@@@@@@@@#.".color(theme.primary).to_string(),
        "                .@@@@@@@@@@@@@@@@.".color(theme.primary).to_string(),
        "               #@@@@@@@@@@@@@@@@@@#".color(theme.primary).to_string(),
        "              #@@@@@@@@@@@@@@@@@@@@@#".color(theme.primary).to_string(),
        "             @@@@@@@@*      &@@@@@@@@".color(theme.primary).to_string(),
        "            #@@@@@@@&        &@@@@@@@#".color(theme.primary).to_string(),
        "           .@@@@@@@%          %@@@@@@@.".color(theme.primary).to_string(),
        "           #@@@@@@@            @@@@@@@#".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}aruda", "G".color(theme.secondary)),
    ]
}

fn default_ascii(theme: &crate::colors::ThemeColors) -> Vec<String> {
    vec![
        "        .---.".color(theme.primary).to_string(),
        "       /     \\".color(theme.primary).to_string(),
        "      | () () |".color(theme.primary).to_string(),
        "       \\  ^  /".color(theme.primary).to_string(),
        "        |||||".color(theme.primary).to_string(),
        "        |||||".color(theme.primary).to_string(),
        "".to_string(),
        format!("{}nknown", "U".color(theme.secondary)),
    ]
}
