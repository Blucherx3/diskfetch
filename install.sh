#!/bin/sh


FILES="main.c nvme-interface.c health-progres-bar.c output.c ata-interface.c"
LIBS="nvme atasmart"


gcc -g3 $FILES -o diskfetch $(echo "$LIBS" | sed 's/ / -l/g' | sed 's/^/-l/') 


mkdir /usr/local/bin/diskfetch 
mv $(pwd)/diskfetch /usr/local/bin/diskfetch/ 

cp -r $(pwd)/assets $HOME/.local/bin/diskfetch/

touch $HOME/.local/bin/diskfetch/diskfetch.sh

cd $HOME/.local/bin/diskfetch
echo "#!/bin/bash" > diskfetch.sh
echo "sudo /usr/local/bin/diskfetch/diskfetch \"\$@\"" >> diskfetch.sh

chmod +x diskfetch.sh

sudo ln -sf diskfetch.sh /usr/bin/diskfetch
