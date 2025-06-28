@echo off
cd /d cookie-worker

echo Checking if cookie-worker Docker image exists...
docker images -q cookie-worker >nul 2>&1
if %errorlevel% neq 0 (
    echo Cookie-worker image not found. Building image...
    docker build -t cookie-worker .
    if %errorlevel% neq 0 (
        echo Failed to build cookie-worker image!
        pause
        exit /b 1
    )
    echo Image built successfully!
) else (
    echo Cookie-worker image already exists.
)

echo Starting cookie-worker container...
docker run --rm cookie-worker --network epi-sign
