# ─── Builder (all on Debian Bookworm) ────────────────────────────────────────────
FROM rust:1.87.0-bookworm AS builder

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libpq-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

    
    # Install diesel_cli (Postgres) for the GNU target
    RUN cargo install \
    diesel_cli \
    --no-default-features \
    --features postgres \
    --root /app/diesel-install
    
WORKDIR /app

# Copy manifests first (to cache deps)
COPY Cargo.toml Cargo.lock ./

COPY src/ ./src/

# Build the Linux server (default x86_64-unknown-linux-gnu)
RUN cargo build --release



# ─── Runner (Debian Bookworm-slim) ───────────────────────────────────────────────
FROM debian:bookworm-slim AS runner

RUN apt-get update && \
    apt-get install -y \
    libpq5 \
    openssl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

WORKDIR /app

COPY migrations ./migrations
COPY diesel.toml ./
COPY --from=builder /app/target/release/epi-sign-backend ./server
COPY --from=builder /app/diesel-install/bin/diesel /usr/local/bin/diesel

# Create startup script
RUN echo '#!/bin/bash\n\
set -e\n\
\n\
# Check if this is the first run by looking for a marker file\n\
if [ ! -f /app/.migrations-complete ]; then\n\
    echo "Running database migrations..."\n\
    diesel migration run\n\
    # Create marker file to indicate migrations have been run\n\
    touch /app/.migrations-complete\n\
    echo "Migrations completed successfully"\n\
else\n\
    echo "Migrations already completed, skipping..."\n\
fi\n\
\n\
# Start the server\n\
exec ./server' > /app/start.sh

RUN chmod +x /app/start.sh server /usr/local/bin/diesel

EXPOSE 80

CMD ["/app/start.sh"]