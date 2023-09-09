#!/bin/bash

#### Description ####

# Script that generates random files for you.
# Use the following environment variables to customize, check the defaults below.
#
# OUTPUT_DIR: Where the files should be placed
# FILE_SIZE_MAX: How large each file can
# COUNT: How many files should be created

#### Setup ####

if [[ $OUTPUT_DIR == "" ]]; then
  OUTPUT_DIR="./tmp/random"
fi

if [[ $FILE_SIZE_MAX == "" ]]; then
  FILE_SIZE_MAX=10000
fi

if [[ $COUNT == "" ]]; then
  COUNT=20000
fi

#### Script ####

mkdir -p "$OUTPUT_DIR"

for i in $(seq 1 $COUNT); do
  FILE_NAME="file_$i"
  FILE_PATH="$OUTPUT_DIR/$FILE_NAME"

  # Dont create new files unless you need them
  if [ -f "$FILE_PATH" ]; then
    continue
  fi

  # Randomly decide the file size
  FILE_SIZE=$((1 + $RANDOM % $FILE_SIZE_MAX))

  echo "Creating file: $FILE_PATH"

  # Create the file using random bytes so we get different hashes
  dd if=/dev/urandom bs="$FILE_SIZE" count=1 of="$FILE_PATH" 2> /dev/null
done
