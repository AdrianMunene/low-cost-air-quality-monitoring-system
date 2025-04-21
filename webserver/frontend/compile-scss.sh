#!/bin/bash

# Check if sass is installed
if ! command -v sass &> /dev/null; then
    echo "sass is not installed. Please install it with npm install -g sass"
    echo "Alternatively, you can use the CSS file directly by changing index.html"
    exit 1
fi

# Compile SCSS to CSS
echo "Compiling SCSS to CSS..."
sass index.scss index.css

echo "SCSS compilation complete!"
