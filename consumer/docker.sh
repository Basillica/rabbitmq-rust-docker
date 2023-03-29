#!/bin/bash

docker build -t consumer .
docker tag consumer basillica/consumer
docker run consumer