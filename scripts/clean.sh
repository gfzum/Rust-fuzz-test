echo 0 > count.txt

for dir in [0-9][0-9][0-9][0-9]*/
do
    rm -rf "$dir"
done