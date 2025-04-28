#include "diskfetch.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <fstab.h>
#include <linux/fs.h>
#include <stdio.h>


int print_disk_info(struct disk_info_page disk_info, char** ascii, int len_ascii)
{
    for(int i = 0; i < len_ascii; i++){
        if(i < 9){
            printf("%s %s", ascii[i], get_info_string(i, "abcdbefgh", disk_info));
        }else{
            printf("%s\n", ascii[i]);
        }
    }
    return 0;
}

char* get_vender_name(int v_code)
{
    switch (v_code) {
        case 0x144d:
            return "Samsung";
        case 0x15b7:
            return "Sandisk";
        case 0x1e0f:
            return "Kingston";
        case 0x8086:
            return "Intel";
        case 0x10DE:
            return "Western Digital";
        case 0x1B4B:
            return "Seagate";
        default:
            return "unuknown";
    }
}

int non_enter(char *s)
{
    for(int i = 0; i < strlen(s); i++){
        if(s[i] == '\n'){
            s[i] = ' ';
            break;
        }
    }
    return 0;
}

char** get_ascii_art(int v_code, int* len_aski)
{
    char path[64];
    char color[32];
    switch (v_code) {
        case 0x144d:
            strcpy(path, "assets/ascii_samsung.txt");
            strcpy(color, BLUE);
            break;
        case 0x15b7:
            strcpy(path, "assets/ascii_sandisk.txt");
            strcpy(color, RED);
            break;
        case 0x1e0f:
            strcpy(path, "assets/ascii_kingston.txt");
            strcpy(color, RED);
            break;
        case 0x8086:
            strcpy(path, "assets/ascii_intel.txt");
            strcpy(color, BLUE);
            break;
        case 0x10DE:
            strcpy(path, "assets/ascii_westen.txt");
            strcpy(color, BLUE);
        case 0x1B4B:
            strcpy(path,"assets/ascii_seageate.txt");
            strcpy(color, GREEN);
        default:
            strcpy(path, "assets/ascii_default.txt");
            strcpy(color, YELLOW);
    }

    FILE* ff = fopen(path, "r");

    char *string;
    size_t string_size = 0;
    char **array_string = malloc(sizeof(char*)*128);
    int count = 0;

    while(getline(&string, &string_size, ff) != -1){
        array_string[count] = string;
        count++;
        string = 0;
        string_size = 0;
    }
    
    char **array_string_with_color = malloc(sizeof(char*)*256);
    for(int i = 0; i <= count; i++){
        char* stf = malloc(sizeof(char)*256);
        snprintf(stf, 256, "%s%s"RESET, color, array_string[i]);
        non_enter(stf);
        array_string_with_color[i] = stf;
    }
    fclose(ff);
    
    free(array_string);

    *len_aski = count;

    return array_string_with_color;
}

int free_ascii_art(char** art, int lines) 
{
    for (int i = 0; i < lines; i++) {
        free(art[i]);
    }
    free(art);
    return 0;
}

char* get_info_string(int index,const char mask[], struct disk_info_page disk)
{
    /*
        a - first string with vendor name
        b - -_-_-_-_-_-_-_-_-_-_-_-_
        c - string "HEALTH"
        d - health progres bar
        e - string with size
        f - string with temperature
        g - wtring with power cycles counter
        h - power hours
    */
    if(index > strlen(mask)){
        perror(RED"Error:"RESET" index or mask is unvalidable");                               
    }
    char* resault = malloc(sizeof(char)*128);
    switch (mask[index]) {
        case 'a':
            snprintf(resault, 128, "-----%s-----\n", get_vender_name(disk.vender));
            break;
        case 'b':
            snprintf(resault, 128, HEALT_COLOR_75"-_-_-_-_-_-_-_-_-_-_-_-_\n"RESET);
            break;
        case 'c':
            snprintf(resault, 128, "HEALTH\n");
            break;
        case 'd':
            snprintf(resault, 128, "%s\n", get_health_bar(100 - disk.health));
            break;
        case 'e':
            snprintf(resault, 128, "SIZE: %.2f gb\n", (float)disk.size/(1024 * 1024 * 1024));
            break;
        case 'f':
            snprintf(resault, 128, "TEMPERATURE: %d °C\n", disk.temp);
            break;
        case 'g':
            snprintf(resault, 128, "POWER CYCLES: %llu\n", disk.power_cycles);
            break;
        case 'h':
            snprintf(resault, 128, "HOURS: %llu h\n", disk.hours);
            break;
        default:
            snprintf(resault, 128, " ");
    }

    return resault;

}