# Use the official Rust image
FROM rust:latest

# Install dependencies for Jupyter and Rust kernel
RUN apt-get update && apt-get install -y \
    python3-pip \
    python3-dev \
    build-essential \
    libssl-dev \
    libcurl4-openssl-dev \
    libsqlite3-dev \
    python3-venv \
    graphviz \
    tree \
    && apt-get clean

# Create a virtual environment for Python
RUN python3 -m venv /opt/venv

# Activate the virtual environment and install Jupyter
RUN /opt/venv/bin/pip install --upgrade pip && \
    /opt/venv/bin/pip install jupyter

# Install the evcxr Jupyter kernel
RUN cargo install evcxr_jupyter

# Install necessary Rust dependencies
RUN cargo install evcxr

# Install rustfmt as part of the Rust toolchain
RUN rustup component add rustfmt

# Set up the evcxr Jupyter kernel
RUN evcxr_jupyter --install

# Expose port 8888 for Jupyter
EXPOSE 8888

# Set the working directory inside the container (where notebooks will be stored)
WORKDIR /workspace

# Start Jupyter when the container runs, specifying the directory to store notebooks
CMD ["/opt/venv/bin/jupyter", "notebook", "--ip=0.0.0.0", "--allow-root", "--NotebookApp.token=''", "--notebook-dir=/workspace"]

