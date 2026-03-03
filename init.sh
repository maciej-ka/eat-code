#!/bin/bash

languages=(
  c,
  cpp,
  cs,
  dart,
  erl,
  ex,
  go,
  hs,
  java,
  js,
  kt,
  php,
  py2,
  py3,
  rb,
  rkt,
  rs,
  scala,
  sh,
  swift,
  ts
)

usage() {
  echo ". $0 language destination"
  echo "language (${languages[*]})"
}

(($# != 2)) && usage && return 1

language=$1
[[ ! ${languages[*]} =~ $language ]] && echo "unknown language $language" && usage && return 1

# transform 1077 into 1/1077
destination=$2
if [[ "$destination" =~ ^[0-9]+$ ]]; then
  destination="${destination:0:1}/$destination"
fi

mkdir -p $destination

# transform into 1/1077/2-cpp
count=$(find "$destination" -mindepth 1 -maxdepth 1 -type d | wc -l)
destination="$destination/$((count + 1))-$language"

cp -r $language $destination && cd $destination

