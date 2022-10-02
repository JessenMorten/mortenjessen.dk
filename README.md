# mortenjessen.dk

## TODO

- https://swimburger.net/blog/dotnet/how-to-deploy-aspnet-blazor-webassembly-to-github-pages

## How to develop

### Install WebAssembly target

`rustup target add wasm32-unknown-unknown`

### Install trunk

`cargo install trunk`

### Start development server

`trunk serve`

### Build in release mode

`trunk build --release`
