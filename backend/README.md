# lembas-backend

This repository contains the source code for lembas' API server.

## Configuration

The server can be configured with the following environment variables:

- `DATABASE_URL`: Required - the URL of a Postgres database to use for data persistence.
- `IDP_URL`: Required - the URL of an OAuth IDP against which to verify JWTs. This needs to be the same IDP as the frontend
- `IDP_DISABLED`: Allows explicit disabling of security features for development when set to `FALSE`

## Building

This project uses `cargo`, and can be built with `cargo build` - use `cargo run --release` to build and run the server. Builds require

To build a cross platform Docker image, run `docker build -t lembas .` To run the container, use:

```bash
sudo docker run \
 -p 3000:3000 \
 -e DATABASE_URL=$DATABASE_URL \
 -e IDP_URL=$IDP_URL \
 --rm \
 -d\
 --name lembas \
 --network="host" \
 lembas
```

## Development Environment

A local database instance can be started by running `docker compose up -d` in the `db` folder. Data is
persisted in a local volume called `pgdata`, which will need to be created with `docker volume create pgdata` if it doesn't already exist.

The best way to connect a frontend is using `ngrok`. Run `ngrok http 3000` to start a tunnelling session,
then start the backend. Use the provided public URL to connect.

The server has a feature called `dev` which can be enabled via the command line using `cargo run --release --features dev`.
This will clear the database on server startup and run any commands in `/db/devdata.sql`.

## Testing

The server has a suite of unit and integration tests which hit both the database access methods and top level API calls. The model responses against which the API is tested can be found in tests/responses. The test data against which the test suite is run is defined in tests/fixtures. Run the test suite with `cargo test`

## Deployment

The server is deployed automatically from `main` by a GitLab runner. The infrastructure and configuration for this deployment is defined in `template.yml`.
