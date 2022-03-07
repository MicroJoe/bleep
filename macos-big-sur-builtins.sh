#!/bin/sh

# Run this script to integrate MacOs Big Sur sounds into
# the builtin sounds.
#
# This script is here for obvious copyright reasons:
# distributing such files may be a copyright infringement.

ZIPFILE=BigSur-Sounds-All.zip
URL=https://bigsur-sounds.itsnoahevans.co.uk/$ZIPFILE
DOWNLOAD=true

if ! command -v ffmpeg > /dev/null
then
    echo "You need to install ffmpeg first."
    exit 1
fi


if "$DOWNLOAD"
then
	rm -r "__macos"
	rm -r "sounds"
	rm "$ZIPFILE"
	rm -R builtins/macos/

	wget "$URL"
	unzip -LL "$ZIPFILE"

	rm "$ZIPFILE"
	mkdir -p builtins
	mv "sounds/" builtins/macos/
fi

# rodio does not support AIFF playback.
# files are converted to FLAC which is lossless and compressed
for ext in 'aiff' 'aif' 'caf' 'm4r'
do
	for f in $(find builtins/macos/ -type f -name "*.$ext")
	do
	    ffmpeg -y -i $f builtins/macos/$(basename $f ".$ext").flac
	    rm $f
	done
done

for d in $(find builtins/macos/ -mindepth 1 -type d)
do
	rm -r "$d"
done
