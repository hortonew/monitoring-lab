FROM mcr.microsoft.com/playwright/python:v1.43.0-jammy

# Set the working directory in the container
WORKDIR /usr/src/app

# Install Poetry
RUN pip install poetry

# Copy the local code to the container's working directory
COPY . /usr/src/app

# Install project dependencies
RUN poetry lock && poetry install

# This command runs the tests every 20 seconds
CMD sleep 10; \
    while true; do \
    echo "Running unauthenticated tests..."; \
    poetry run pytest -n auto -k test_unauth; \
    echo "Sleeping for 2s..."; \
    sleep 2; \
    echo "Running authenticated tests..."; \
    poetry run pytest -n auto -k test_auth; \
    echo "Sleeping for 10s..."; \
    sleep 10; \
    done
