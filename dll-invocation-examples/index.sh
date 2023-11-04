#!/usr/bin/env bash
export DYLD_LIBRARY_PATH=../target/setup-lib/lib:$DYLD_LIBRARY_PATH
node index.js $*