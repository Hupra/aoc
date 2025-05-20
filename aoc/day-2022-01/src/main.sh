#!/bin/bash

cat test.txt | awk -v RS='' '
{
    sum = 0
    for (i = 1; i <= NF; i++) {
        sum += $i
    }
    if (sum > max) {
        max = sum
    }
}
END {
    print max
}'


#######################################

max=0
sum=0

while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ -z "$line" ]]; then
        if (( sum > max )); then
            max=$sum
        fi
        sum=0
    else
        for num in $line; do
            sum=$((sum + num))
        done
    fi
done < test.txt

if (( sum > max )); then
    max=$sum
fi

echo "$max"