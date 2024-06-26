#!/bin/bash

DOCKER_IMAGE_NAME="my-act-image"
ARC_PLATFORM="linux/$(uname -m)"
CONTAINER_ARCHITECTURE=""

if [[ $ARC_PLATFORM == "linux/arm64" ]]; then
  CONTAINER_ARCHITECTURE="--container-architecture=$ARC_PLATFORM"
fi

# Check if Docker image exists
if [[ "$(docker images -q $DOCKER_IMAGE_NAME 2> /dev/null)" == "" ]]; then
  echo "Building Docker image..."
  # Check if the machine architecture is not arm64
  if [[ $ARC_PLATFORM == "linux/arm64" ]]; then
    docker buildx create --use
    docker buildx build --platform $ARC_PLATFORM -t $DOCKER_IMAGE_NAME -f Dockerfile.arm64 . --load
  else
    docker build -t $DOCKER_IMAGE_NAME -f Dockerfile.amd64 .
  fi
fi

echo "Running gh act on $ARC_PLATFORM..."
gh act "$CONTAINER_ARCHITECTURE" --pull=false --platform ubuntu-latest=$DOCKER_IMAGE_NAME -j build
