# Use an official image with cmake, g++ and cargo preinstalled
FROM ubuntu:20.04

# Set environment variables to prevent interactive prompt during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies for both C++ and Rust projects
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    wget \
    curl \
    xz-utils \
    libsqlite3-dev \
    clang \
    zlib1g-dev \
    libbz2-dev \
    liblzma-dev \
    pkg-config \
    libzstd-dev \
    libssl-dev \
    cargo \
    libssl-dev \
    libpq-dev

# Set default build type to Debug if not specified
ARG BUILD_TYPE=Debug

# Create a working directory
WORKDIR /app

# Clone atinyvectors repo using GitHub token and build it
RUN git clone https://github.com/billionvectors/atinyvectors.git \
    && cd atinyvectors \
    && mkdir -p build && cd build \
    && cmake -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_BUILD_TYPE=${BUILD_TYPE} .. \
    && make -j$(nproc) \
    && mkdir -p /app/lib \
    && cp libatinyvectors.so /app/lib/

# Clone asimplevectors
COPY ./ /app/asimplevectors

# Copy .env.local to .env in asimplevectors directory
RUN cp /app/asimplevectors/.env.local /app/asimplevectors/.env

# Move the compiled library to asimplevectors/lib/
RUN mkdir -p /app/asimplevectors/lib/ \
    && cp /app/lib/libatinyvectors.so /app/asimplevectors/lib/

# Build asimplevectors using cargo, build in release mode if BUILD_TYPE is Release
WORKDIR /app/asimplevectors
RUN if [ "${BUILD_TYPE}" = "Release" ]; then \
      cargo build --release; \
    else \
      cargo build; \
    fi

RUN if [ "${BUILD_TYPE}" = "Release" ]; then \
    ln -sf /app/asimplevectors/target/release/asimplevectors /app/asimplevectors/run_asimplevectors; \
  else \
    ln -sf /app/asimplevectors/target/debug/asimplevectors /app/asimplevectors/run_asimplevectors; \
  fi

# Expose the necessary ports (21001 and 21002)
EXPOSE 21001
EXPOSE 21002

# Set the entrypoint to the correct binary
ENTRYPOINT ["/app/asimplevectors/run_asimplevectors"]