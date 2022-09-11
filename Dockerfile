FROM node:10-alpine

ENV PATH="${PATH}:/root/.cargo/bin"

RUN apk --no-cache add \
    curl \
    build-base

RUN curl -sSf https://sh.rustup.rs/ | sh -s -- --default-toolchain=1.63.0 -y && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 

RUN mkdir /build/
WORKDIR /build/
COPY . /build/
RUN wasm-pack build && \
    cd www && \
    npm install && \
    npm run build && \
    rm dist/runserver
