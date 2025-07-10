# Testing the installation of the Python interface of the Rust function
FROM python:3.10-slim
LABEL authors="frrit"

# Confirm the wheel index is reachable (optional but useful for debugging)

# Upgrade pip
RUN pip install --upgrade pip

# Install the Rust-based Python package from the custom GitHub Pages index
RUN pip install string-processing

# Install test dependencies
RUN pip install pytest

# Copy test suite into container
COPY python_testing/ /python_testing/

# Run tests
WORKDIR /python_testing
RUN pytest

# Prevent container from exiting immediately (optional, for manual inspection)
ENTRYPOINT ["tail", "-f", "/dev/null"]
