#!/bin/bash
for dir in $(find benchmarks/suite -type d) 
do
  if [ $dir = "benchmarks/suite" ]; then
    continue
  fi 
  if [ !  $(find $dir -name "*.rs") ] ; then
    echo $dir
  fi
done
