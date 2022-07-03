# Rust Webauthn

## Redis
This app requires Redis for session mgt.  

### Podman
Make sure Podman is installed.  If not, install with Homebrew
````zsh
> brew install podman
...
> podman machine init
````

Check podman
````zsh
> podman info
````

Start Podman 
````zsh
> podman machine start
````

Start the MongoDB image with persistant store
````zsh
podman run  --rm -d \
  --name mongo \
  -v $PWD/mongo-data:/data/db  \
  -p 27017:27017 \
  mongo:latest
````

Start the Redis image without persistant store

````zsh
podman run  --rm -d \
  --name redis\
  -p 6379:6379 \
  redis --requirepass StrongPassword
````