FROM rust:1-alpine as build
ENV TZ=Asia/Tokyo

WORKDIR /opt/app
COPY . .

RUN cargo build --release


FROM gcr.io/distroless/base:nonroot AS runner
ENV TZ=Asia/Tokyo

COPY --from=build --chown=nonroot:nonroot /opt/app/target/release/virtual-natsumatsuri /bin/server

EXPOSE 8080
USER nonroot
ENTRYPOINT [ "/bin/server" ]
