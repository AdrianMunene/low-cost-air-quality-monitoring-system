#!/bin/bash

# Generate certificates if they don't exist
if [ ! -f "backend/certs/cert.pem" ] || [ ! -f "backend/certs/key.pem" ]; then
    echo "Generating certificates..."
    cd backend
    cargo run --bin generate_certs
    cd ..
fi

# Run the backend with HTTPS
echo "Starting backend with HTTPS..."
cd backend
USE_HTTPS=true cargo run --bin backend &
BACKEND_PID=$!
cd ..

# Wait for the backend to start
echo "Waiting for backend to start..."
sleep 5

# Run the frontend
echo "Starting frontend..."
cd frontend
trunk serve --open

# When the frontend is stopped, also stop the backend
kill $BACKEND_PID
