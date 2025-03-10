/*
 ██████╗ ██╗███████╗██╗  ██╗███████╗███████╗████████╗ ██████╗██╗  ██╗
 ██╔══██╗██║██╔════╝██║ ██╔╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║
 ██║  ██║██║███████╗█████╔╝ █████╗  █████╗     ██║   ██║     ███████║
 ██║  ██║██║╚════██║██╔═██╗ ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║
 ██████╔╝██║███████║██║  ██╗██║     ███████╗   ██║   ╚██████╗██║  ██║
 ╚═════╝ ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝
    by Blucher
    discord - https://discordapp.com/users/657872729126469642/
    github - https://github.com/Blucherx3
*/
use libatasmart::Disk;
use output::diskOutput;
use std;
use std::fs;
mod output;
mod output_suppurts;
use std::env;
mod psevdoDB;

fn main () {
    let ver = "beta-1.0.0";

    let arg: Vec<String> = env::args().collect();

    let input = &arg[1];

    match input.trim_end().to_string().as_str(){
        "-h" => print!("{}\n", fs::read_to_string("assets/help.txt").unwrap()),
        "-v"  => print!("{}\n", ver),
        "" => print!("Please, enter command with argumet or flag -> diskfetch <flags> <argument>"),
        _ => {
            let path = std::path::Path::new(format!("/dev/{}", input.trim_end()).leak());
            
            let disk = Disk::new(path);
        
            let dsk = diskOutput::new(&mut disk.unwrap(), input);
        
            dsk.output();
        }
    }
}

