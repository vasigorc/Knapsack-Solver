#!/bin/bash

# Variables
DOCKER_IMAGE_NAME="my-act-image"
GH_ACT_PLATFORM="ubuntu-latest"

# Check if Docker image exists
if [[ "$(docker images -q $DOCKER_IMAGE_NAME 2> /dev/null)" == "" ]]; then
  # Docker image doesn't exist, create it
  echo "Building Docker image..."
  docker build -t $DOCKER_IMAGE_NAME .
fi

# Run `gh act` with the specified platform and Docker image
echo "Running gh act..."
gh act --pull=false --platform $GH_ACT_PLATFORM=$DOCKER_IMAGE_NAME -j build
