# monitoring-lab

A docker lab for testing applications

## About this project

Starting the lab sets up a few things.

1. Influxdb, with 3 buckets: cdm (cpu/disk/memory), weather (fake weather data), vault (hashicorp vault), monitoring (future use)
2. Telegraf
    a. outputs to InfluxDB
    b. inputs for cdm/system data
    b. inputs for Hashicorp Vault data
3. Grafana, with an integration to Influxdb
4. A custom rust application that writes fake weather data into Influxdb
5. A custom go application that load tests Vault (and the performance data is logged to Influxdb via the Telegraf input)

## Warning

This lab sets up insecure secrets that are in plain text in the configurations.  These are not meant for production use.  Use at your own risk.

## Dependencies

- docker
- docker-compose

## Quickstart

`make build`

## Supported applications

- InfluxDB v2
- Telegraf
- Grafana

## Custom applications

- rust-weather-app
- vault-load-test (Go)
