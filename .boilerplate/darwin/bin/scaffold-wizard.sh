#!/usr/bin/env bash
WORK_DIR=$(dirname $0)
DYLD_LIBRARY_PATH=$WORK_DIR/../lib:$DYLD_LIBRARY_PATH $WORK_DIR/scaffold-wizard