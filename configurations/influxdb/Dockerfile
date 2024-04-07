FROM influxdb:latest

COPY scripts/create_buckets.sh /docker-entrypoint-initdb.d/
RUN chmod +x /docker-entrypoint-initdb.d/create_buckets.sh

# The entrypoint and cmd are inherited from the base image