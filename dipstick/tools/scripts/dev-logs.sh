#!/bin/bash
# Tail logs from all services

SERVICE=${1:-all}

if [ "$SERVICE" = "all" ]; then
    echo "📋 Tailing logs from all services..."
    tail -f logs/*/*.log
elif [ -d "logs/$SERVICE" ]; then
    echo "📋 Tailing logs from $SERVICE..."
    tail -f logs/$SERVICE/*.log
else
    echo "Usage: $0 [all|collector|analyzer|publisher|dashboard]"
    exit 1
fi
