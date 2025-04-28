#!/bin/bash
for dir in $(find suite -type d) 
do
  if [ $dir = "suite" ]; then
    continue
  fi 
  if [ !  $(find $dir -name "*.rs") ] ; then
    echo $dir
  fi
done
