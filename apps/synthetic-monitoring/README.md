# synthetic-monitoring

Playwright is used to synthetically navigate to pages, logging in or clicking around like a user would do.

## Setup

1. pip install poetry
2. poetry lock && poetry install
3. playwright install
4. pytest -n auto

## Testing

Note: The authenticated tests can affect the unauthenticated tests due to session state.  Here we try to isolate each.

```sh
# Unauthenticated
pytest -n auto -k test_unauth

# Unauthenticated, and retry each test 5 times
pytest -n auto --count=5 -k test_unauth

# Authenticated
pytest -n auto -k test_auth
```
