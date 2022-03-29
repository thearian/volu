#!/usr/bin/bash

if [ -z "$1" ]; then
  echo "Please enter version like below:"
  echo "./package-bin.sh 0.1.0"
  exit 1
fi
version=$1

list=$(ls bin)
dirs=($list)

for item in "${dirs[@]}"
do
  tar cvzf release/volu-v$version-$item.tar.gz bin/$item/release/
done

