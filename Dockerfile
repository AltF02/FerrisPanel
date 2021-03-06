FROM alpine AS builder

RUN apk add --update --no-cache gcc git curl nodejs npm libc-dev && \
    mkdir /ferrispanel

ENV PATH=/root/.cargo/bin:$PATH
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /build/ferrispanel

COPY . .

RUN cargo build --release && mv ./target/release/ferrispanel /ferrispanel/ferrispanel
RUN cd client && \
    npm install && \
    npm run build

#COPY /build/ferrispanel/www /pufferpanel/www

FROM alpine
COPY --from=builder /ferrispanel /ferrispanel

WORKDIR /ferrispanel

ENTRYPOINT ["/ferrispanel/ferrispanel"]
CMD ["start"]

