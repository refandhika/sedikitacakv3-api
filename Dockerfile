FROM rust:1.79 as builder
WORKDIR /home/refandhika/project/sedikitacakv3-api
COPY . .
RUN cargo build --release
FROM rust:1.79-slim
RUN apt-get update && apt-get install -y libssl-dev
RUN apt-get update && apt-get install -y libpq-dev
WORKDIR /home/refandhika/local/bin
COPY --from=builder /home/refandhika/project/sedikitacakv3-api/target/release/sedikitacakv3-api .
EXPOSE 8080
CMD ["./sedikitacakv3-api"]
