#!/bin/bash

last_modified=$(stat -f "%m" /Users/sheki/.newsboat/cache.db)
current_time=$(date +%s)
time_diff=$((current_time - last_modified))

if [ $time_diff -gt 432000 ]; then
    /opt/homebrew/bin/newsboat
else
    echo "nope"
fi
