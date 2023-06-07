FROM rust as environment_setup
RUN apt-get update && \
    apt-get install -y libasound2-dev portaudio19-dev build-essential libpulse-dev libdbus-1-dev libudev-dev 

WORKDIR /usr/src/simple_collision
COPY . .

FROM environment_setup as build

CMD ["cargo", "build"]

FROM environment_setup as dev
RUN cargo install cargo-watch

CMD ["cargo-watch", "-x", "run"]
