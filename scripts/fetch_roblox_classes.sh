#!/bin/bash

set -euo pipefail

URL="https://raw.githubusercontent.com/MaximumADHD/Roblox-Client-Tracker/refs/heads/roblox/API-Dump.json"
OUTPUT_FILE="crates/zap-documentation/docs/generated/instance_classes.txt"

if ! command -v jq &> /dev/null; then
    echo "jq is not installed"
    exit 1
fi

curl -s "$URL" | jq -r '.Classes[].Name' | sort | uniq > "$OUTPUT_FILE"

echo "Extracted unique class names to $OUTPUT_FILE"
