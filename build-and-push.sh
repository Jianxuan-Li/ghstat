#!/bin/bash

IMAGE_NAME="freeyeti/ghstat"
TAG="latest"

docker build -t $IMAGE_NAME:$TAG .
docker push $IMAGE_NAME:$TAG
