use libatasmart::Disk;
use std::fs;
use ansi_term::Color;

use crate::output_suppurts::{get_disk_ascii, get_disk_brend, get_disk_condition, get_disk_logo_color};
pub struct diskOutput{
    asciLogo: String,
    colorLogo: Color,
    brend: String,
    condition: String,
    con_bar: usize,
    model: String,
    size: String,
    firmware: String,
    serial: String,
    temp: String,
    bed_sector: String,
}

impl diskOutput{
    pub fn new(disk: &mut Disk, input: &String) -> diskOutput{
        let dinfo = disk.identify_parse().expect("disk info grt error");
        diskOutput{
            asciLogo: fs::read_to_string(get_disk_ascii(disk)).expect("Eror read asci art file"),
            colorLogo: get_disk_logo_color(disk),
            brend: get_disk_brend(disk).to_string(),
            condition: "CONDITION".to_string(),
            con_bar: get_disk_condition(input, disk),
            model: format!("MODEL: {}", dinfo.model),
            size: format!("SIZE: {} gb", (disk.get_disk_size().expect("disk size get error")/1024_u64.pow(3)).to_string()),
            firmware: format!("FIRMAWARE: {}", dinfo.firmware),
            serial: format!("SERIAL: {}", dinfo.serial),
            temp: format!("TEMPERATURE: {}°C", disk.get_temperature().expect("temperature disk get error")/1000 - 273),
            bed_sector: format!("BED SECTORS: {}", &disk.get_bad_sectors().expect("bad sectors get error")),
        }
    }

    pub fn output(&self) {
        let bar = if self.con_bar == 100{
            format!("[{}{}%{}]", Color::Green.paint("#####"), self.con_bar.to_string(), Color::Green.paint("#####"))
        } else if self.con_bar > 50{
            format!("[{}{}%{}___]", Color::Yellow.paint("#####"), self.con_bar.to_string(), Color::Yellow.paint("##"))
        } else if self.con_bar > 20 && self.con_bar < 50 {
            format!("[{}{}%_____]", Color::RGB(255, 184, 60).paint("#####"), self.con_bar.to_string())
        } else {
            format!("[{}___{}%_____]", Color::Red.paint("##"), self.con_bar.to_string())
        };

        let ascii_strings: Vec<&str> = self.asciLogo.split("\n").map(|s| s.trim()).collect();
        let output_strings: Vec<&str> = vec![
            &self.brend, 
            "-_-_-_-_-_-_-_-_-_-_-_-_", 
            &self.condition, 
            &bar, 
            &self.bed_sector,
            "-_-_-_-_-_-_-_-_-_-_-_-_", 
            &self.model, 
            &self.size, 
            &self.firmware,
            &self.serial,
            &self.temp];

        for i in 0..ascii_strings.len() {
            if i == 1_usize || i == 5_usize {
                print!("{}  {}\n" , self.colorLogo.paint(ascii_strings[i].replace("_", " ")), Color::Cyan.paint(output_strings[i]))
            }else if i < output_strings.len() {
                print!("{}  {}\n" , self.colorLogo.paint(ascii_strings[i].replace("_", " ")), output_strings[i]);
            } else {
                print!("{}\n", self.colorLogo.paint(ascii_strings[i].replace("_", " ")));
            }
        }

    }

}
