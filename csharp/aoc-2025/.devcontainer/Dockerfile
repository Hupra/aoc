FROM mcr.microsoft.com/dotnet/sdk:9.0

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    python3 \
    git \
    libgomp1 \
    curl \
    unzip \
    && rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/Z3Prover/z3.git /opt/z3 \
    && cd /opt/z3 \
    && python3 scripts/mk_make.py --dotnet \
    && cd build \
    && make -j$(nproc) \
    && make install