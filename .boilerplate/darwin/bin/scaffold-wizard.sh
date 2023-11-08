#!/usr/bin/env bash
brew list pkg-config || sudo brew install --prefix=/usr/local pkg-config
brew list gtk+3 || sudo brew install --prefix=/usr/local gtk+3
$(dirname $0)/scaffold-wizard