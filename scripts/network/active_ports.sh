#!/bin/bash

echo "Checking active ports..."

sudo ss -tuln | awk 'NR>1 {print $1, $5}' | while read -r protocol port; do
    if [[ "$port" == *:* ]]; then
        port=$(echo "$port" | cut -d':' -f2)
    fi
    echo "Protocol: $protocol, Port: $port"
done