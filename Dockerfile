FROM rust:alpine

RUN apk add --update --no-cache gcc git curl nodejs npm && \
    mkdir /ferrispanel

WORKDIR /build/ferrispanel
COPY . .

RUN cargo build --release && \
    cp ./target/release/ferrispanel /ferrispanel/ferrispanel && \
    cd client && \
    npm install && \
    npm run build && \
    mv ../www /pufferpanel/www/

FROM alpine
COPY --from=rust /ferrispanel /ferrispanel

WORKDIR /ferrispanel

ENTRYPOINT ["/ferrispanel/ferrispanel"]
CMD ["start"]

