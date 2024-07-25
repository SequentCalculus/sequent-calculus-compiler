#!/bin/bash
EXAMPLEDIR="./examples"
OUTFILE="./web-app/examples.js"

echo "" > $OUTFILE
ALLEXAMPLES=()
for INFILE in $EXAMPLEDIR/*.sc 
do
  BASENAME=$(basename -- "$INFILE")
  FILENAME="${BASENAME%.*}"

  echo "const $FILENAME = \`" >> $OUTFILE
  cat $INFILE | sed 's/\\/\\\\/g' >> $OUTFILE
  echo "\`;" >> $OUTFILE
  echo "" >> $OUTFILE 

  ALLEXAMPLES+=($FILENAME)
done

echo "">>$OUTFILE
echo "const all_examples={" >> $OUTFILE

for ex_name in ${ALLEXAMPLES[*]}
do 
  echo "'$ex_name':$ex_name," >> $OUTFILE
done
echo "};" >> $OUTFILE
