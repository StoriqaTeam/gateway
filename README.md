# Gateway
Graphql backend service that acts as a gateway for web browser / mobile

## Getting started

1. Install docker CE
2. `docker network create storiqa`
3. `cd docker && docker-compose run gateway`
4. To run project `cargo run`

### Run shell with Rust

`docker-compose run --service-ports gateway bash`

### Check if front is able to fetch schema

After running gateway in Docker Compose, from `docker` directory run:

```
docker-compose -f compose-test.yml up
```

## Integration tests

```
docker-compose run -e RUN_MODE=test.toml --rm gateway cargo test -- --test-threads=1
```
