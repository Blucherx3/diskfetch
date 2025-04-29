#!/bin/sh


FILES="main.c nvme-interface.c health-progres-bar.c output.c ata-interface.c"
LIBS="nvme atasmart"


gcc -g3 $FILES -o diskfetch $(echo "$LIBS" | sed 's/ / -l/g' | sed 's/^/-l/') 


mkdir /usr/local/bin/diskfetch 
mv $(pwd)/diskfetch /usr/local/bin/diskfetch/ 

cp -r $(pwd)/assets /usr/local/bin/diskfetch/

touch /usr/bin/diskfetch

echo "#!/bin/bash" > /usr/bin/diskfetch
echo "sudo /usr/local/bin/diskfetch/diskfetch \"\$@\"" >> /usr/bin/diskfetch

chmod +x /usr/bin/diskfetch

