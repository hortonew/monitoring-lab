#!/bin/sh

set -e

# When updating this file, do:
# docker-compose build --no-cache
# docker-compose up -d
influx bucket create -n monitoring -o sre -r 0
influx bucket create -n weather -o sre -r 0
influx bucket create -n vault -o sre -r 0
