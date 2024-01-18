FROM rust:1.72.0-slim-buster 

RUN apt-get update \
  && apt-get upgrade --no-install-recommends -y --force-yes \
  && apt-get install -y --no-install-recommends \
  curl \
  openssh-client \
  make \
  gcc \
  default-libmysqlclient-dev \
  libxrender1 \
  pkg-config \
  libssl-dev \
  libpq-dev \
  cmake \
  g++ \
  libsasl2-dev \
  libsasl2-2 \
  libsasl2-modules

RUN rustup component add rust-analyzer rustfmt clippy \
  && cargo install sqlx-cli cargo-watch cargo-nextest \
  && rustup update nightly \
  && rustup component add rustc-codegen-cranelift-preview --toolchain nightly

RUN curl -o mold.tar.gz -L https://github.com/rui314/mold/releases/download/v2.4.0/mold-2.4.0-aarch64-linux.tar.gz \
  && mkdir /usr/local/mold \
  && tar -xzf mold.tar.gz -C /usr/local/mold --strip-components=1

CMD [ "cargo", "watch", "-x", "run" ]
