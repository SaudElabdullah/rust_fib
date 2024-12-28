FROM python:3.9

# Install required dependencies for Rust and Python
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN /root/.cargo/bin/rustup install nightly
RUN /root/.cargo/bin/rustup override set nightly
ENV PATH="/root/.cargo/bin:${PATH}"
RUN pip install maturin

# Set the working directory in the container
WORKDIR /

COPY rust_app/ ./rust_app

# Fetch Rust dependencies (to leverage Docker caching)
RUN cargo fetch -Z unstable-options --manifest-path /rust_app/Cargo.toml --lockfile-path /rust_app/Cargo.lock

# Build the Rust Python module
RUN maturin build -m /rust_app/Cargo.toml --release
RUN pip install /rust_app/target/wheels/*.whl

# Clean up build artifacts to save space
RUN rm -rf /rust_app/target

# Copy the requirements file into the container
COPY requirements.txt .

# Install dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of the application into the container
COPY . ./app

# Expose the port the app runs on
EXPOSE 8000

# Command to run the FastAPI app with Uvicorn
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000", "--reload"]
