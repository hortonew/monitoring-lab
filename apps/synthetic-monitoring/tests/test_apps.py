import os
import re

from playwright.sync_api import Page, expect


def get_address(application: str):
    applications = {
        "influxdb": "http://influxdb2:8086",
        "grafana": "http://grafana:3000",
        "vault": "http://vault:8200",
        "prometheus": "http://prometheus:9090",
        "unleash": "http://unleashweb:4242",
    }
    # if environment is not running in docker, replace hostname with localhost
    if os.environ.get("DOCKER") != "true":
        for k, v in applications.items():
            # replace everything before the first port with http://localhost
            applications[k] = re.sub(r"http://[^:]+", "http://localhost", v)

    return applications[application]


def test_unauth_influxdb_has_title(page: Page):
    page.goto(get_address("influxdb"))
    expect(page).to_have_title(re.compile("InfluxDB"))


def test_unauth_grafana_has_title(page: Page):
    page.goto(get_address("grafana"))
    expect(page).to_have_title(re.compile("Grafana"))


def test_unauth_vault_has_title(page: Page):
    page.goto(get_address("vault"))
    expect(page).to_have_title(re.compile("Vault"))


def test_unauth_prometheus_has_title(page: Page):
    page.goto(get_address("prometheus"))
    expect(page).to_have_title(re.compile("Prometheus"))


def test_unauth_unleash_has_title(page: Page):
    page.goto(get_address("unleash"))
    expect(page).to_have_title(re.compile("Unleash"))


def test_auth_influxdb_login_works(page: Page):
    page.goto(get_address("influxdb"))
    page.fill("input#login", "admin")
    page.fill("input#password", "admin123456")
    page.click('button[type="submit"]')
    page.goto(page.url + "/load-data/buckets")
    expect(page).to_have_title("InfluxDB", timeout=10000)
    expect(page).to_have_url(re.compile(f"{get_address('influxdb')}.+load-data/buckets"))


def test_auth_unleash_login_works(page: Page):
    page.goto(get_address("unleash"))
    page.fill("input#username", "admin")
    page.fill("input#password", "admin123456")
    page.click('button[type="submit"]')
    page.goto(f"{get_address('unleash')}/projects/default")
    page.wait_for_timeout(1000)
    expect(page).to_have_url(re.compile(f"{get_address('unleash')}/projects/default"))
