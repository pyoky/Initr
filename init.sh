#!/bin/bash
cd $HOME
#echo -e "- Download and run (d)"
#echo -e "- Run only         (r)"
#echo -e "- Upload profile   (u)"
#echo -e "- Download vimrc   (v)"
#echo -n "            Option: "
#read OPTION

Help()
{
  # Display Help
  echo "Usage: (-h|d|r|u|v)"
}

DownloadFF()
{
  rm -rf ~/.wine/ ~/aqua/
	wget https://github.com/prasmussen/gdrive/releases/download/2.1.1/gdrive_2.1.1_linux_386.tar.gz
 	tar -xf gdrive_2.1.1_linux_386.tar.gz
  kill -KILL $(pgrep Hamoni | awk '{print $1 }')
 	chmod +x ./gdrive
  ./gdrive about | tee >(egrep -o -m 1 'https?://[^ ]+' | xargs xdg-open > /dev/null 2>&1)
 	./gdrive list
 	./gdrive download 1RespoMBLqUoo1lIu9osm3HHYWRHgIXQK
 	# FIXME: why is the second download so slow?
 	./gdrive download 1nuDAWIF4JTVQFYa_a4WuJ00vB1qdegKg
 	unzip mozilla.zip -d $HOME/
 	chmod +x ff.appimage
 	# start, wait for few seconds, kill, then restart. 
 	# This allows plugins to load 
	nohup ./ff.appimage --appimage-extract-and-run > /dev/null 2>&1 &
 	sleep 6
 	kill -KILL $(pgrep ff-bin | awk '{print $1 }')
 	sleep 3
	nohup ./ff.appimage --appimage-extract-and-run > /dev/null 2>&1 &
}
UploadFF()
{
	cd $HOME
  	rm mozilla.zip
  	zip -r mozilla.zip .mozilla
  	./gdrive update 1nuDAWIF4JTVQFYa_a4WuJ00vB1qdegKg mozilla.zip
}

DownloadOB()
{
	cd ~/	
	./gdrive download 10DPvbuzLfaNcF5TrrJ3Un-_ALvnl4ice
	./gdrive download 16ygpJ2yxsoPC7c3ft74p0Kyfc4fo0Juj
    unzip obsidian.zip -d /
	unzip projectLife.zip -d $HOME/

	cd /tmp
	wget https://github.com/obsidianmd/obsidian-releases/releases/download/v0.12.15/Obsidian-0.12.15.AppImage
	chmod +x Obsidian-0.12.15.AppImage
	./Obsidian-0.12.15.AppImage --appimage-extract-and-run --no-sandbox

	cd ~/

}
UploadOB()
{
	cd ~/
	zip -r obsidian.zip ~/.config/obsidian/
	zip -r projectLife.zip 'Project Life'
	./gdrive update 10DPvbuzLfaNcF5TrrJ3Un-_ALvnl4ice projectLife.zip
	./gdrive update 16ygpJ2yxsoPC7c3ft74p0Kyfc4fo0Juj obsidian.zip
}
DownloadVS()
{
	cd ~/	

	./gdrive download 1lWvDzguUoUmESYW-mc-bOtGePwTeU6cy
	./gdrive download 1DBNXstHxfDJ7dCEMO5RWt1nYzs6fBk6b
    unzip -o vsext.zip
	mkdir ~/.ssh
	mv config ~/.ssh/

	cd ~/

}
UploadVS()
{
	cd ~/
	zip -r vsext.zip .vscode
	./gdrive update 1lWvDzguUoUmESYW-mc-bOtGePwTeU6cy vsext.zip
}


while getopts ":hdruvb:" OPTION; do
  case $OPTION in
    h) # display help
      Help
      ;;
  	d)
      DownloadFF;
	  DownloadVS
  		;;
  	r)
		nohup ./ff.appimage --appimage-extract-and-run > /dev/null 2>&1 &
  		;;
  	u)
  		UploadFF;
		UploadVS
  		;;
    v)
      wget https://raw.githubusercontent.com/pyoky/ff/main/.vimrc
      echo "Downloaded to ~/.vimrc"
      ;;
    # x)
    #   xrandr --newmode "1680x1050_60.00"  146.25  1680 1784 1960 2240  1050 1053 1059 1089 -hsync +vsync
    #   xrandr --addmode DP-3 1680x1050_60.00
    #   echo "Display resolution will change now"
    #   wait 3
    #   xrandr -s 1680x1050
    #   ;;
	
  	\?)
      echo "Use with -h to show list of options"
      ;;
  esac
done
