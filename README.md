# Rust GraphQL Microservice

A containerized graphql microservice starter with rust

## Components

DB: [postgresql](https://www.postgresql.org/), [diesel](http://diesel.rs/)
backend: [juniper](https://github.com/graphql-rust/juniper), [actix-web](https://github.com/actix/actix-web)

## Setup

Install postgresql, [docker](https://docs.docker.com/v17.09/engine/installation/) and [docker-compose](https://docs.docker.com/compose/install/)

Clone this repository and run docker compose

```bash
git clone <git url>
cd rust-graphql-backend
docker-compose up
```

In browser, go to [http://localhost:5001/graphiql](http://localhost:3001/graphiql)

Then you will see this image.
![](https://i.imgur.com/PT2gy4f.png)

The `cargo-watch` package will be running in the container so that the server automatically updates after changes in the project directory including DB and backend.

---

## Backend development

## Monitoring

![](https://i.imgur.com/zLHT3l6.png)


Prometheus is available at [http://localhost:9090](http://localhost:9090)
Grafana is available at [http://localhost:3000](http://localhost:3000),
and the ID is `admin` and PW is `pass`.
For Grafana's plugins and server configuration, you can change it in [docker-compose.yml](../docker-compose.yml).

## Remove

Simply run `docker-compose down` in the project root directory.

## Future works

Make this into a progressive Rust framework for building efficient, reliable, and scalable server-side applications like [Nest](https://nestjs.com/).
