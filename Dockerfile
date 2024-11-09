# Use an official image with cmake, g++ and cargo preinstalled
FROM ubuntu:20.04

# Set environment variables to prevent interactive prompt during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies and prerequisites for adding Kitware repository
RUN apt-get update && apt-get install -y \
    build-essential \
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
    libomp-dev \
    libblas-dev \
    liblapack-dev \
    apt-transport-https \
    ca-certificates \
    gnupg \
    software-properties-common && \
    # Add Kitware APT repository for the latest CMake
    wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc | gpg --dearmor -o /usr/share/keyrings/kitware-archive-keyring.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ focal main" | tee /etc/apt/sources.list.d/kitware.list >/dev/null && \
    apt-get update && \
    apt-get install -y cmake && \
    # Clean up to reduce image size
    rm -rf /var/lib/apt/lists/*

# Set default build type to Release if not specified
ARG BUILD_TYPE=Release

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
