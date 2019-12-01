#!/bin/bash
message=$(< $1)

regex=":[a-z_]+: .+"

if ! [[ $message =~ $regex ]];
then
  echo "Your message has to start with a gitmoji: https://gitmoji.carloscuesta.me/"
  exit 1
fi
