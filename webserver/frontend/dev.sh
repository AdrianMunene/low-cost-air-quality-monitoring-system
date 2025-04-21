#!/bin/bash

# Check if sass is installed
if ! command -v sass &> /dev/null; then
    echo "sass is not installed. Please install it with npm install -g sass"
    exit 1
fi

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "trunk is not installed. Please install it with cargo install trunk"
    exit 1
fi

# Start sass in watch mode
echo "Starting sass in watch mode..."
sass --watch index.scss:index.css &
SASS_PID=$!

# Trap to kill sass process when script exits
trap "kill $SASS_PID" EXIT

# Start trunk serve
echo "Starting trunk serve..."
trunk serve

# This will not be reached until trunk serve is stopped
kill $SASS_PID
