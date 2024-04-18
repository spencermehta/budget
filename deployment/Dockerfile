FROM rust:1.77.2
WORKDIR /usr/src/budget
COPY ../ .
RUN cargo build --release
EXPOSE 80
CMD target/release/budget
