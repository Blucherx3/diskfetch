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
#include <string.h>


int main(int argSize, char* argv[])
{   
    if(strlen(argv[1]) < 8){
        perror("Error: diskfetch argument unvalidate");
        return -1;
    }
    int cont;
    
    struct disk_info_page disk1;

    switch (argv[1][5]) {
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
    char **ascii = get_ascii_art(disk1.vender, &cont);
    
    print_disk_info(disk1, ascii, cont);

    free(ascii);

    return 0;
}