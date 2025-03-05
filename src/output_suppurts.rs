use libatasmart::Disk;
use libatasmart::IdentifyParsedData;
use ansi_term::Color;
use std::fs;
use std::io::*;
use std::io;
use std::string;

pub fn get_disk_brend(disk: &mut Disk) -> String{
    let model = disk.identify_parse().unwrap().model.to_uppercase();
    if model.starts_with("WD"){
        "Western Digital".to_string()
    } else if model.starts_with("ST"){
        "Seagate BarraCuda".to_string()
    } else if model.starts_with("SB"){
        "SmartBuy".to_string()
    }else if model.contains("TOSHIBA"){
        "TOSHIBA".to_string()
    } else {
        "Unuknow".to_string()
    }
        
}

pub fn get_disk_ascii(disk: &mut Disk) -> String{
    let model = disk.identify_parse().unwrap().model.to_uppercase();
    if model.contains("WDC"){
        "assets/ascii_westen.txt".to_string()
    } else if model.starts_with("ST"){
        "assets/ascii_seageate.txt".to_string()
    } else {
        "assets/ascii_default.txt".to_string()
    }
}

pub fn get_disk_logo_color(disk: &mut Disk) -> Color{
    let model = disk.identify_parse().unwrap().model.to_uppercase();
    if model.starts_with("WD"){
        Color::Blue
    } else if model.starts_with("ST"){
        Color::Green
    } else {
        Color::Yellow
    }
}

pub fn get_disk_condition(disk: &mut Disk) -> usize{
    let bed: usize = disk.get_bad_sectors().unwrap().try_into().unwrap();
    let disk_name = disk.disk.as_os_str().to_str().unwrap()[5..8].to_string();
    let path_to_sector = format!("/sys/block/{}/queue/hw_sector_size", disk_name);
    let sector_size: usize = fs::read_to_string(path_to_sector).expect("fwef").trim().parse().unwrap();
    let total_size: usize = disk.get_disk_size().expect("wefewfwf").try_into().unwrap();

    (100_usize - (bed*100 / (total_size*100 / sector_size*100)))
}