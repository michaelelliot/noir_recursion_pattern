#!/usr/bin/env sh

NOIR_VERSION_REQUIRED="0.32.0"
BB_VERSION_REQUIRED="0.46.1"

# Version checks
if [[ ! $(nargo --version | head -n 1) == "nargo version = $NOIR_VERSION_REQUIRED" ]]; then
    echo "Error: nargo version $NOIR_VERSION_REQUIRED required"
    exit 1
fi
if [[ ! $(bb --version) == "$BB_VERSION_REQUIRED" ]]; then
    echo "Error: bb version $BB_VERSION_REQUIRED required"
    exit 1
fi
