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
#include "diskfetch.h"
#include <stdio.h>
#include <stdlib.h>
#include <libconfig.h>


int main(int argSize, char* argv[])
{   

    if(argSize < 2){
        perror("Error: uvalidable command or argument: try -h");
        return -1;
    }

    int cont;
    struct disk_info_page disk1;
    switch (argv[1][0]) {
        case '/': switch (argv[1][5]) {
            case 's':
                disk1 = get_sata_info_page(argv[1]);
                break;
            case 'n':
                disk1 = get_nvme_info(argv[1], argSize);
                break;
            default:
                printf(RED"Diskfetch: i don`t know what is a disk"RESET);
                return -1;
        }
        break;

        case '-': switch (argv[1][1]) {
            case 'h':
                puts("\ndiskfetch - it's like neofetch, but for disk\nsyntax:\n\t\tdiskfetch <flags> <path to your disk>\n\nflags:\n\t-h\t- outputs this information\n\t-v\t- outputs version\n");
                goto pre_complete;
            case 'v':
                puts("diskfetch - 1.0.0");
                goto pre_complete;
            default:
                perror("Error: i don`t know whot is a flag\n");
                return -1;
        }
        break;

        default:
            puts(RED"Error: underfind argument"RESET);
            return -1;
    }

    char **ascii = get_ascii_art(disk1.vender, &cont);
    
    print_disk_info(disk1, ascii, cont);

    free(ascii);

    
    pre_complete:
    return 0;
}