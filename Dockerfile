FROM rust:1.35 as build

COPY ./ ./

RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/RESTful-image /build-out/

FROM alpine:3.9

RUN apk update && apk add postgresql=11.3-r0

COPY --from=build /build-out/RESTful-image /

RUN sudo service postgresql start

CMD /RESTful-image
