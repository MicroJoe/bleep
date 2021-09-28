#!/bin/sh

# Run this script to integrate MacOs Big Sur sounds into
# the builtin sounds.
#
# This script is here for obvious copyright reasons:
# distributing such files may be a copyright infringement.

if ! command -v ffmpeg
then
    echo "You need to install ffmpeg first."
    exit 1
fi

rm -f macOS-Big-Sur-Sounds.zip
rm -Rf builtins/macos/
wget https://appletldposts.com/wp-content/uploads/2020/07/macOS-Big-Sur-Sounds.zip
unzip -LL macOS-Big-Sur-Sounds.zip
rm macOS-Big-Sur-Sounds.zip
mkdir -p builtins
mv "macos big sur sounds/" builtins/macos/

# rodio does not support AIFF playback.
# files are converted to FLAC which is lossless and compressed
for f in builtins/macos/*.aiff
do
    ffmpeg -y -i $f builtins/macos/$(basename $f .aiff).flac
    rm $f
done