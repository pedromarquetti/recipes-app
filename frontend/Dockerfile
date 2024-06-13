FROM rustlang/rust:nightly


RUN apt update && apt -y upgrade

# installing wasm target
RUN rustup target add wasm32-unknown-unknown
# installing trunk
RUN cargo install trunk

WORKDIR /app

# copying the entire project because Trunk.toml requires a "backend" and "frontend" directories
COPY . .
WORKDIR /app/frontend

# todo: trunk serve should not be used... the app needs to be built and serverd 
CMD [ "trunk","serve" ]