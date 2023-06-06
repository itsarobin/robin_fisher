FROM rust
RUN apt-get update && \
    apt-get install -y libasound2-dev portaudio19-dev build-essential libpulse-dev libdbus-1-dev libudev-dev 

WORKDIR /usr/src/bevy_simple_collision
COPY . .

RUN cargo install cargo-watch
RUN cargo install --path .

CMD ["cargo-watch", "-x", "run"]