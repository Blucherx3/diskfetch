use libatasmart::Disk;
use ansi_term::Color;
use crate::psevdoDB::*;
use std::fs;

pub fn get_disk_brend(disk: &mut Disk) -> &str{
    let model = disk.identify_parse().expect("Fn: get_disk_brend: error get model").model.to_uppercase();
    for (index, &key) in KEY_WORDS.iter().enumerate() {
        if model.contains(key) && index < FULL_BREND.len() {
           return FULL_BREND[index];
        }
    }
    FULL_BREND[FULL_BREND.len() - 1]
}

pub fn get_disk_ascii(disk: &mut Disk) -> &str{
    let model = disk.identify_parse().expect("Fn: get_disk_ascii: error get model").model.to_uppercase();
    for (index, &key) in KEY_WORDS.iter().enumerate() {
        if model.contains(key) && index < ASCII_PATH.len() {
            return ASCII_PATH[index];
        }
    }
    ASCII_PATH[ASCII_PATH.len() - 1]
}

pub fn get_disk_logo_color(disk: &mut Disk) -> Color{
    let model = disk.identify_parse().expect("Fn: get_disk_logo_color: error get model").model.to_uppercase();
    for (index, &key) in KEY_WORDS.iter().enumerate() {
        if model.contains(key) && index < LOGO_COLOR.len(){
            return LOGO_COLOR[index];
        }
    }
    LOGO_COLOR[LOGO_COLOR.len() - 1]
}

pub fn get_disk_condition(input: &str, disk: &mut Disk) -> usize{
    let bed: usize = disk.get_bad_sectors().unwrap().try_into().unwrap();
    let disk_name = disk.disk.as_os_str().to_str().unwrap()[5..8].to_string();
    let path_to_sector = format!("/sys/block/{}/queue/hw_sector_size", disk_name);
    let sector_size: usize = fs::read_to_string(path_to_sector).expect("fwef").trim().parse().unwrap();
    let total_size: usize = disk.get_disk_size().expect("wefewfwf").try_into().unwrap();

    (100_usize - (bed*100 / (total_size*100 / sector_size*100)))
}
