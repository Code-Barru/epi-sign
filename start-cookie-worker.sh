#!/bin/bash

cd cookie-worker

echo "Checking if cookie-worker Docker image exists..."
if ! docker images -q cookie-worker >/dev/null 2>&1; then
    echo "Cookie-worker image not found. Building image..."
    if ! docker build -t cookie-worker .; then
        echo "Failed to build cookie-worker image!"
        exit 1
    fi
    echo "Image built successfully!"
else
    echo "Cookie-worker image already exists."
fi

echo "Starting cookie-worker container..."
docker run --rm cookie-worker

echo "Cookie-worker execution completed."
read -p "Press Enter to continue..."