#!/usr/bin/env bash
brew list pkg-config || sudo brew install pkg-config
brew list gtk+3      || sudo brew install gtk+3
$(dirname $0)/scaffold-wizard