# Welcome to my recipes Fullstack app

![workflow status](https://github.com/pedromarquetti/recipes-app/actions/workflows/tests.yml/badge.svg)

This is still a work in progress, since i'm learning Rust/Yew/Warp/Diesel as I code.

## Local development setup and dependencies

### [Yew](https://yew.rs) setup

1. [Install WASM target](https://yew.rs/docs/getting-started/introduction#install-webassembly-target) with `rustup target add wasm32-unknown-unknown`

### Diesel CLI setup

1. Diesel cli requires
   `sudo apt install default-libmysqlclient-dev libsqlite3-dev libpq-dev`
1. Then run
   `cargo install diesel_cli`

### .env file

1. Create .env file

   `echo "DATABASE_URL=postgresql://dev:dev@localhost:5432" >> .env`

   `echo JWT_SECRET_KEY=secret >> .env`

   > Note: there's now an example .env file that can be used

   > Note: The postgresql://... URI might be different

### Autoreload with [systemfd](https://github.com/mitsuhiko/systemfd) for the backend

1. `cargo install systemfd cargo-watch`
1. change dir to "backend" dir then run
1. `systemfd --no-pid -s http::5555 -- cargo watch  -w . -w ../db -w ../frontend -x run`
   > Note on the above command: -w specifies which files will be watched by [cargo-watch](https://github.com/watchexec/cargo-watch)

### Frontend setup and dependencies

The frontend uses [Trunk](https://github.com/trunk-rs/trunk), that auto-reloads by default:

1. cargo install trunk

## Running with Docker

Now there's a docker-compose file ready to be ran, simply run docker-compose up after setting up the .env file (example env file supplied).

Both the front and backend will be exposed.

> Note: The default behaviour of the backend is to provide unsafe HTTP requests and cookies, set the DEV_ENV variable to false so the JWToken is set as Secure.

## Running manually

In the future I'll implement a Docker setup for easier deployment, for now, run

1. Frontend:

   - `TRUNK_SERVE_PROXY_BACKEND=<backend server:port/api> trunk serve`

     - Note:
       TRUNK_SERVE_PROXY_BACKEND env. variable is necessary because Trunk.toml [[proxy]] wasn't working well with Docker

       If you want to change the default proxy (defaults to localhost), pass do `trunk serve --proxy-backend <backend URL>`.

1. Backend:
   - `cargo run`
     > Note: backend is expecting a DATABASE_URL env. var with a valid postgresql link

# TODO

- [ ] improve API Response errors (example:`{
    "error": "duplicate key value violates unique constraint \"recipe_recipe_name_key\""
}` should be `{"error":"key <KEY> already exists"}`)
- [x] ~~Create a seperate 'user validation function', so there's no repeating code (\*\_route.rs files validate user credentials)~~ done

## Useful links

1. [Official Yew docs](https://yew.rs/docs/tutorial#fetching-data-using-external-rest-api)
1. [Tutorial/starting point (for me at least)](https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/)
