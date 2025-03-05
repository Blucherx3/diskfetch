#!/bin/bash

cargo build --release 

mkdir $HOME/diskfetch 
mv $(pwd)/target/release/diskfetch $HOME/diskfetch 

cp -r $(pwd)/assets $HOME/diskfetch/

touch $HOME/diskfetch/diskfetch_fakeroot.sh

cd $HOME/diskfetch
echo "#!/bin/bash" > $HOME/diskfetch/diskfetch_fakeroot.sh
echo "cd \$HOME/diskfetch" >> $HOME/diskfetch/diskfetch_fakeroot.sh
echo "sudo \$HOME/diskfetch/diskfetch \"\$@\"" >> $HOME/diskfetch/diskfetch_fakeroot.sh

chmod +x $HOME/diskfetch/diskfetch_fakeroot.sh

sudo ln -sf $HOME/diskfetch/diskfetch_fakeroot.sh /usr/local/bin/diskfetch