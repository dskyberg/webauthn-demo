# WebAuthn Demo

WebAuthn is a [W3C standard][w3c] that enables web developers to replace passwords in their applications with [FIDO authentication][fido2]. This repository implements a demo built with Rust and React.


## Launch Redis
The server leverages Redis to persist assertions and sessions.
If you have podman installed, use the [services](./server/services.sh) script to launch and manage the container.

## Run the server
The server is written in Rust and tested on MacOS with Rust 1.6.2. The server runs on localhost:3001 by default. To build and launch, simply run

```sh
> cd server
> cargo run
```
To increase server log output run

```sh
> RUST_LOG=trace cargo run
```

## Run the client
The client is a React SPA style app, developed with [create-react-app]. The client runs on localhost:3000 and proxies localhost:3001 (set in [package.json](package.json)) All the normal behaviors for building and launching apply. To run

```sh
> npm i
> npm start
```


[w3c]: https://w3c.github.io/webauthn/
[fido2]: https://fidoalliance.org/fido2/
[create-react-app]: https://create-react-app.dev/