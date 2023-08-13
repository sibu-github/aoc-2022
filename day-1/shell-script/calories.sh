#!/bin/bash

file="inputs.txt"

declare -i highest1=0
declare -i highest2=0
declare -i highest3=0
declare -i sum=0

while read line
do 
if [ "$line" == "" ];then
  if [ $sum -gt $highest1 ];then
    highest3=highest2
    highest2=highest1
    highest1=sum
  else
    if [ $sum -gt $highest2 ];then
      highest3=highest2
      highest2=sum
    else
      if [ $sum -gt $highest3 ];then
        highest3=sum
      fi
    fi
  fi
  sum=0
  continue
fi
sum=$((sum+line))
done < $file

if [ $sum -gt $highest1 ];then
  highest2=highest1
  highest1=sum
else
  if [ $sum -gt $highest2 ];then
    highest3=highest2
    highest2=sum
  else
    if [ $sum -gt $highest3 ];then
      highest3=sum
    fi
  fi
fi

total=$((highest1+highest2+highest3))

echo "Highest = $highest1"
echo "Total of highest 3 = $total"



