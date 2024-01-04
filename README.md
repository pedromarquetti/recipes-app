# Welcome to my recipes Fullstack app

This is still a work in progress, since i'm learning Rust/Yew/Warp/Diesel as I code.

# Running

In the future I'll implement a Docker setup for easier deployment, for now, run

1. Frontend:

   - `trunk serve`

     - Note:

       The frontend will proxy API requests to the `proxy backend` defined in Trunk.toml, and will panic if the server is not available.

       If you want to change the default proxy (defaults to localhost), pass do `trunk serve --proxy-backend <backend URL>`.

1. Backend:
   - `cargo run`
   - Note: backend is expecting a DATABASE_URL env. var with a valid postgresql link

# TODO

1. improve API Response errors (example:`{
    "error": "duplicate key value violates unique constraint \"recipe_recipe_name_key\""
}` should be `{"error":"key <KEY> already exists"}`)

## Useful links

1. [Official Yew docs](https://yew.rs/docs/tutorial#fetching-data-using-external-rest-api)
1. [Tutorial/starting point (for me at least)](https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/)
