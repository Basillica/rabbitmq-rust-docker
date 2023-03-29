#!/bin/bash

docker build -t flask-api .
docker tag flask-api basillica/flask-api:v1
docker run flask-api:v1