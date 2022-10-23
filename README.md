# WebAuthn Demo
A simple WebAuthn FIDO2 Passkey demo written in Rust and React.  The UI leverage Chakra. 

Server persistence is managed via Redis and Mongodb.

WebAuthn is a [W3C standard][w3c] that enables web developers to replace passwords in their applications with [FIDO authentication][fido2]. Passkey is currently supported and tested on both Chrome and Safari.

## Dependencies
Be sure the following dependencies are met before continuing

### Rust
Rust (and Cargo) must be installed.  If you are running on Mac, just use Homebrew:

```bash
> brew install rustup
```
This will install the Rust toolchain manager - rustup. Now, refresh your terminal, and
run the following:

```bash
> rustup init
```
The above installs the latest version of the rust compiler and Cargo toolset.

### Node
Node must be installed.  I suggest using nvm to install node.  On Mac, run the following:

```bash
> brew install nvm
```
Once complete, the Node Version Manager will be available, for managing Node installs.  Now, run the following:

```bash
> nvm install 16
```
Once the above completes, you will have the latest stable Node v16 installed (v16.14.0 at the time of this writing).

### ContainerD
If you want to use the containerd support built into the demo script, you will need a
containerd manager.  You can use Docker.  I suggest using Lima, just to ensure you don't violate Docker licenses.

To install and use Lima, do the following:

```bash
> brew install lima
```

The script will look for both Docker and Lima, and use whatever it find.  If you don't want to use containerd services, you can install Mongodb and Redis locally. Just ignore the "services" and "volume" portions of the Demo scripts.

## Running the Demo
The demo is comprised of a Rust based server that runs on port 3001, and a React Javascript based client that runs on port 3000.  The demo also uses Redis.  I use Podman to launch the Redis container.  It's easy.  Don't be afraid. 

### Step 1: Launch the Services
The server leverages Redis to persist assertions and sessions. 
If you have podman installed, use the [demo](./demo) script to launch and manage the containers.

```bash
> ./demo services up
```

### Step 2: Build the client and server
You will need to run the following in separate terminal windows.  In the first window, build, and then run the server:

```bash
> ./demo server build
> ./demo server run

Note: You can just run the server.  Cargo will ensure the server is properly build before running.
To increase server log output run

```bash
> RUST_LOG=trace ./demo server run
```
Now build and run the client.  Note: unlike the server, the client MUST be built before running for the first time.

```bash
> ./demo client build
```
Once complete, run:

```bash
> ./demo client run
```

## Stopping the demo

### Stop the client and server
In each terminal window, just click CTRL-C to stop the client and server.

### Stop the services

To stop the containerd services:

```bash
> ./demo services down
```

### Clean up service volumes

If you want to get rid of all the volume containers for the services

```bash
> ./demo volume rm
```

If you want to get rid of just one volume container,

```bash
> ./demo volume rm [mongo, redis]
```

[w3c]: https://w3c.github.io/webauthn/
[fido2]: https://fidoalliance.org/fido2/
[create-react-app]: https://create-react-app.dev/