# Use Ubuntu as base image
FROM ubuntu:22.04

# Set environment variables to avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive
ENV DISPLAY=:99

# Install system dependencies
RUN apt-get update && apt-get install -y \
    # Node.js and npm
    curl \
    gnupg \
    # X11 and display dependencies
    xvfb \
    x11vnc \
    fluxbox \
    # Chrome/Chromium dependencies
    wget \
    ca-certificates \
    fonts-liberation \
    libappindicator3-1 \
    libasound2 \
    libatk-bridge2.0-0 \
    libatk1.0-0 \
    libc6 \
    libcairo2 \
    libcups2 \
    libdbus-1-3 \
    libexpat1 \
    libfontconfig1 \
    libgbm1 \
    libgcc1 \
    libglib2.0-0 \
    libgtk-3-0 \
    libnspr4 \
    libnss3 \
    libpango-1.0-0 \
    libpangocairo-1.0-0 \
    libstdc++6 \
    libx11-6 \
    libx11-xcb1 \
    libxcb1 \
    libxcomposite1 \
    libxcursor1 \
    libxdamage1 \
    libxext6 \
    libxfixes3 \
    libxi6 \
    libxrandr2 \
    libxrender1 \
    libxss1 \
    libxtst6 \
    lsb-release \
    xdg-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 18.x
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# Create app directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci


# Copy application code
COPY script.js .

# Create startup script
RUN echo '#!/bin/bash\n\
    # Start Xvfb\n\
    Xvfb :99 -screen 0 1024x768x24 -ac +extension GLX +render -noreset &\n\
    \n\
    # Start window manager (optional, for better window handling)\n\
    fluxbox -display :99 &\n\
    \n\
    # Wait for X server to start\n\
    sleep 2\n\
    \n\
    # Start your application\n\
    exec node "script.js"\n\
    ' > /start.sh && chmod +x /start.sh

ENV DATABASE_URL=postgres://postgres:postgres@epi-sign-db/postgres

# Set the startup script as entrypoint
ENTRYPOINT ["/start.sh"]