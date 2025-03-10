use ansi_term::Color;

pub const KEY_WORDS:[&str; 4] = [
    "WD",
    "ST",
    "SB",
    "TOSHIBA",
];

//last index this \|/ arrais is a default value

pub const FULL_BREND:[&str; 5] = [
    "Western Digital",
    "Seagate",
    "SmartBuy",
    "TOSHIBA",
    "Unuknown",
];

pub const ASCII_PATH:[&str; 3] = [
    "assets/ascii_westen.txt",
    "assets/ascii_seageate.txt",
    "assets/ascii_default.txt",
];

pub const LOGO_COLOR:[Color; 3] = [
    Color::Blue,
    Color::Green,
    Color::Yellow,
];
