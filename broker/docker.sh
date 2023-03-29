#!/bin/bash

docker build -t broker .
docker tag broker basillica/broker
docker run broker