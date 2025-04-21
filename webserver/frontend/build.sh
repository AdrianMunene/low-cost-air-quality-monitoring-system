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

# Compile SCSS to CSS
echo "Compiling SCSS to CSS..."
sass index.scss index.css

# Build the project with Trunk
echo "Building with Trunk..."
trunk build

echo "Build complete! Output is in the dist directory."
