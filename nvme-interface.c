#include "diskfetch.h"
#include <nvme/types.h>
#include <stdio.h>
#include <fcntl.h>
#include <string.h>
#include <sys/ioctl.h>
#include <linux/fs.h>
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/stat.h>
#include <ctype.h>
#include <atasmart.h>

struct disk_info_page get_nvme_info(char path_j[], int pathSize){

    /*
        0 1 2 3 4 5 6 7 8 9
        | | | | | | | | | |
        / d e v / n v m e 0...
    */

    if(strlen(path_j) < 9){
        perror("Error: get_nvme_info arguments is unvalidable");
    };

    char path[11] = {'/', 'd', 'e', 'v', '/', 'n', 'v', 'm', 'e', path_j[9], '\0'};

    int fd = open(path, O_RDONLY);
    if(fd == -1){
        perror("Error: get device discriptor fail");
    }

    struct nvme_smart_log smarts;
    struct disk_info_page resault;
    
    int rc1 = nvme_get_log_smart(fd, NVME_NSID_ALL, false, &smarts);
    
    if(rc1 == 0)
    {   
        resault.temp = (int)(*((short*)smarts.temperature) - 273);
        resault.health = smarts.percent_used;
        resault.power_cycles = (*((__u64*)smarts.power_cycles));
        resault.hours = *(long long unsigned int*)smarts.power_on_hours;
    } 
    else 
    {
        perror("Error: get smart page fail");
    }
    close(fd);


    char numero_p = '1';
    
    if(strlen(path_j) > 9)
        if (isdigit(path_j[11]))
            numero_p = path_j[11];

    char numero_d = path_j[9];
    char path2[13] = {'/', 'd', 'e', 'v', '/', 'n', 'v', 'm', 'e', numero_d, 'n', numero_p, '\0'};
    
    int ff = open(path2, O_RDONLY);

    __u64 bytes;

    struct stat st;
    if (fstat(ff, &st) == -1 || !S_ISBLK(st.st_mode)) {
        fprintf(stderr, "%s — is not a blocks device!\n", path2);
        close(ff);
        return resault;
    }
    /*
        FUCK! IDK, why if I enter '/dev/nvme0' I can get the smart parameters 
        but I cannot get the disk size, and if I enter '/dev/nvme0n1' then 
        I send the smart parameters incorrectly but the disk size I get is correct. 
        Based on all this, I will simply trim the argument to the format /dev/nvme<number>,
        and then, to get the size, I will receive a descriptor from /dev/nvme<number> + "n1".
    */
    int rc2 = ioctl(ff, BLKGETSIZE64, &bytes);
    
    if(rc2 == 0)
    {
        resault.size = bytes;
    } 
    else perror("Error: get nvme size fail");
    
    close(ff);

    char nvme_name[6] = {'n', 'v', 'm', 'e', path_j[9], '\0'};
    char path_vendor[256];

    snprintf(path_vendor, sizeof(path_vendor), 
                            "/sys/class/nvme/%s/device/vendor", nvme_name);
    FILE* vendor_discriptor = fopen(path_vendor, "r");
    fscanf(vendor_discriptor, "%i", &resault.vender);
    fclose(vendor_discriptor);
    return resault;
    
}