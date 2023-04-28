#!/bin/bash

# Get the current count from the count file
count=$(cat count.txt)

# Increment the count
count=$((count+1))

# Save the new count to the count file
echo $count > count.txt

# Create a folder with the current date
folder_name=$(printf "%04d" $count)
mkdir $folder_name

# Export SYMCC_OUTPUT_DIR variable
export SYMCC_OUTPUT_DIR=`pwd`/$folder_name

# Read user input
#echo "Please enter a command:"
#read command
#$command

#echo 0 | ./int
echo rfjaskld | ./test

# Execute the command on all files in the folder
for file in $folder_name/*
do
  echo "$file"
  cat $file
  echo "\n"
done