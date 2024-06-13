FROM rustlang/rust:nightly

RUN apt update && apt -y upgrade

WORKDIR /app

# copying the entire project (backend depends on 'db')
COPY . .

# installing the backend
RUN cargo install --path ./backend

CMD [ "backend" ]

