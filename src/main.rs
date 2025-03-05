use libatasmart::nix::sys::termios::FlowArg;
/*
 ________      ___      ________       ___  __        ________  _______       _________    ________      ___  ___     
|\   ___ \    |\  \    |\   ____\     |\  \|\  \     |\  _____\|\  ___ \     |\___   ___\ |\   ____\    |\  \|\  \    
\ \  \_|\ \   \ \  \   \ \  \___|_    \ \  \/  /|_   \ \  \__/ \ \   __/|    \|___ \  \_| \ \  \___|    \ \  \\\  \   
 \ \  \ \\ \   \ \  \   \ \_____  \    \ \   ___  \   \ \   __\ \ \  \_|/__       \ \  \   \ \  \        \ \   __  \  
  \ \  \_\\ \   \ \  \   \|____|\  \    \ \  \\ \  \   \ \  \_|  \ \  \_|\ \       \ \  \   \ \  \____    \ \  \ \  \ 
   \ \_______\   \ \__\    ____\_\  \    \ \__\\ \__\   \ \__\    \ \_______\       \ \__\   \ \_______\   \ \__\ \__\
    \|_______|    \|__|   |\_________\    \|__| \|__|    \|__|     \|_______|        \|__|    \|_______|    \|__|\|__|
                          \|_________|                                                                                
    by Blucher
    discord - https://discordapp.com/users/657872729126469642/
    github - https://github.com/Blucherx3
*/
use libatasmart::Disk;
use libatasmart::SkSmartSelfTest;
use libatasmart::smart_test_to_string;
use output::diskOutput;
use std;
use std::fs;
use std::io;
use std::io::*;
mod output;
mod output_suppurts;
use std::env;

fn osnova1(input: &String){
    let path = std::path::Path::new(input.trim_end().to_string().leak());
    
    let disk = Disk::new(path);

    let dsk = diskOutput::new(&mut disk.unwrap());

    dsk.output();
}
fn main () {
    let ver = "beta-1.0.0";

    let arg: Vec<String> = env::args().collect();

    let mut input = &arg[1];

    match input.trim_end().to_string().as_str(){
        "-h" => print!("{}\n", fs::read_to_string("assets/help.txt").unwrap()),
        "-v"  => print!("{}\n", ver),
        _ => osnova1(input),
    }

}
