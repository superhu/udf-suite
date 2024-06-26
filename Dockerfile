# Dockerfile to build the udf-examples crate and load it. Usage:
#
# ```
# # build image
# docker build . --tag mdb-udf-suite
# # Run image
# docker run --rm -e MARIADB_ROOT_PASSWORD=example --name mdb-udf-suite-c mdb-udf-suite
# # Open a shell
# docker exec -it mdb-udf-suite-c mariadb -pexample
# ```

FROM rust:1.76 AS build

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release \
    && mkdir /output \
    && cp target/release/*.so /output

FROM mariadb:11.1

COPY --from=build /output/* /usr/lib/mysql/plugin/
