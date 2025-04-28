#!/bin/sh


FILES="main.c nvme-interface.c health-progres-bar.c output.c"
LIBS="nvme atasmart"


gcc -O3 $FILES -o diskfetch $(echo "$LIBS" | sed 's/ / -l/g' | sed 's/^/-l/') 