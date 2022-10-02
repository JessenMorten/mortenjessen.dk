# mortenjessen.dk

[![Deploy](https://github.com/JessenMorten/mortenjessen.dk/actions/workflows/deploy.yml/badge.svg?branch=main)](https://github.com/JessenMorten/mortenjessen.dk/actions/workflows/deploy.yml)

## How to develop

### Install WebAssembly target

`rustup target add wasm32-unknown-unknown`

### Install trunk

`cargo install trunk`

### Start development server

`trunk serve`

### Build in release mode

`trunk build --release`
