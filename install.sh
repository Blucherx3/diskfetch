#!/bin/sh


FILES="main.c nvme-interface.c health-progres-bar.c output.c ata-interface.c"
LIBS="nvme atasmart"


gcc -g3 $FILES -o diskfetch $(echo "$LIBS" | sed 's/ / -l/g' | sed 's/^/-l/') 


mkdir /usr/local/bin/diskfetch 
mv $(pwd)/diskfetch /usr/local/bin/diskfetch/ 

cp -r $(pwd)/assets /usr/local/bin/diskfetch/

touch /usr/bin/diskfetch

cat > /usr/bin/diskfetch << EOF
#!/bin/bash

if [ \$# -eq 0 ]; then
    echo "Error: argument of device not found" >&2
    echo "Please use this: diskfetch <device>"
    exit 1
fi

sudo /usr/local/bin/diskfetch/diskfetch "\$@"
EOF

chmod +x /usr/bin/diskfetch

