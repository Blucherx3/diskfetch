use std::{self, fmt::format, process::Output};
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
    temp: String,
}

impl diskOutput{
    pub fn new(disk: &mut Disk) -> diskOutput{
        let dinfo = disk.identify_parse().unwrap();
        diskOutput{
            asciLogo: fs::read_to_string(get_disk_ascii(disk)).expect("Eror read asci art file"),
            colorLogo: get_disk_logo_color(disk),
            brend: get_disk_brend(disk),
            condition: "CONDITION".to_string(),
            con_bar: get_disk_condition(disk),
            model: format!("MODEL: {}", dinfo.model),
            size: format!("SIZE: {} gb", (disk.get_disk_size().unwrap()/1024_u64.pow(3)).to_string()),
            firmware: format!("FIRMAWARE: {}", dinfo.firmware),
            temp: format!("TEMPERATURE: {}°C", disk.get_temperature().unwrap()/1000 - 273),
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
            "-_-_-_-_-_-_-_-_-_-_-_-_", 
            &self.model, 
            &self.size, 
            &self.firmware,
            &self.temp];

        for i in 0..ascii_strings.len() {
            if i == 1_usize || i == 4_usize {
                print!("{}  {}\n" , self.colorLogo.paint(ascii_strings[i].replace("_", " ")), Color::Cyan.paint(output_strings[i]))
            }else if i < output_strings.len() {
                print!("{}  {}\n" , self.colorLogo.paint(ascii_strings[i].replace("_", " ")), output_strings[i]);
            } else {
                print!("{}\n", self.colorLogo.paint(ascii_strings[i].replace("_", " ")));
            }
        }

    }

}