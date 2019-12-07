#!/usr/bin/env sh

day=$(date -d "$D" '+%-d');
folder="day$day"

cargo new "$folder"
cp skeleton/rustfmt.toml $folder/
cp skeleton/main.rs $folder/src/
cp skeleton/.gitignore $folder
python scripts/get-input-data.py
