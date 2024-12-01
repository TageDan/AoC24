#!/bin/bash

while read line ; do

  echo $line | tr -s ' ' | cut -d' ' -f1 >> left
  echo $line | tr -s ' ' | cut -d' ' -f2 >> right

done < input.txt

sum=0

LL=($(sort left))
RL=($(sort right))

for i in "${!LL[@]}"
do
  x=${LL[i]}
  y=${RL[i]}

  
  if [[ $x > $y ]]; then
    sum=$(expr $sum + $x - $y)
  else
    sum=$(expr $sum + $y - $x)
  fi

done

echo $sum

rm left
rm right
