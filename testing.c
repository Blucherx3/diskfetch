#include "diskfetch.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ucontext.h>


int main() {
    char color[] = BLUE;
    FILE* ff = fopen("assets/ascii_samsung.txt", "r");
    char *string;
    size_t string_size = 0;
    char **array_string = malloc(sizeof(char*)*64);
    int count = 0;
    while(getline(&string, &string_size, ff) != -1){
        array_string[count] = string;
        count++;
        string = 0;
        string_size = 0;
    }
    char **array_string_with_color = malloc(sizeof(char*)*128);
    for(int i = 0; i <= count; i++){
        snprintf(array_string_with_color[i], 128, "%s%s"RESET, color, array_string[i]);
    }
    fclose(ff);
    return 0;
}