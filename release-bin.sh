#!/usr/bin/bash
echo "Thank you for trying to release volu"
echo "Is Docker installed and running? (y/n)"
read yn
if [ "$yn" != "y" ]; then
  echo "Please install and run Docker, it is needed for cross platform release"
  exit 1
fi

cargo install cross

version=$(grep version ./Cargo.toml | head -1 | cut -c12-16)
list=$(ls bin)
dirs=($list)
for item in "${dirs[@]}"
do
  cross build --release --target $item
  cp target/$item/release/volu bin/$item/release/volu
  tar cvzf release/volu-v$version-$item.tar.gz bin/$item/release/
done
