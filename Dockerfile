FROM debian:bullseye-slim
WORKDIR /app
ADD target/debug/actix-todos .
CMD ["/app/actix-todos"]
