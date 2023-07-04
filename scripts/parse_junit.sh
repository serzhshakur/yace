#!/bin/bash
set -e
JUNIT_XML_FILE="test-reports/report.xml"

cat $JUNIT_XML_FILE |
  rg -o -w "<testcase .*?>" |
  sed 's/^.* name="\(.*\)" time="\(.*\)">$/\2 "\1"/g' |
  sort -r -t '.' -n -k1
