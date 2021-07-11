FROM ekidd/rust-musl-builder as builder

# RUN  apt-get update \
#     && apt-get install -y wget \
#     && rm -rf /var/lib/apt/lists/*

# RUN wget http://prdownloads.sourceforge.net/ta-lib/ta-lib-0.4.0-src.tar.gz
# RUN tar xvzf ta-lib-0.4.0-src.tar.gz
# RUN rm ta-lib-0.4.0-src.tar.gz
# RUN cd ta-lib
# RUN ./configure
# RUN make
# RUN make install




WORKDIR /home/rust/

COPY . .
RUN cargo test
RUN cargo build --release

ENTRYPOINT ["./target/x86_64-unknown-linux-musl/release/trading-bot"]

FROM scratch
WORKDIR /home/rust/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/trading-bot .
ENTRYPOINT ["./trading-bot"]