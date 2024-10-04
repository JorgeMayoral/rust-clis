#!/usr/bin/env bash

OUTDIR="tests/expected"

[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Test output" > $OUTDIR/output1.txt
echo "Test"  "output" > $OUTDIR/output2.txt
echo -n "Test  output" > $OUTDIR/output3.txt
echo -n "Test" "output" > $OUTDIR/output4.txt
