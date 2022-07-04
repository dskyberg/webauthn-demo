# Rust Webauthn

## Redis
This app requires Redis for session mgt.  The default redis URI is used uness env vars are set.  The following defaults are used.  Either load them in the env in an OS appropriate manner, or create a .env in the workspace root.

```toml
REDIS_SCHEME = redis
REDIS_HOST = 127.0.0.1
REDIS_PORT = 6379
REDIS_USER = ""
REDIS_PASSWORD = ""
REDIS_DATABASE = ""
```

### Podman
I use Podman, rather than Docker.  But you can use whatever you like.  Or, just install Redis!  It totally doesn't matter.  If you want to use Podman, follow the Podman install guide to install and initialize the machine. If you are running on a Mac, Then just do the following.

Install Podman, and init the machine:

````zsh
> brew install podman
...
> podman machine init
````

Check podman, to ensure it installed correctly:

````zsh
> podman info
````

Start the Podman machine.  Do this whenever you need to start it: 

````zsh
> podman machine start
````

Start the Redis image without persistant store

````zsh
podman run  --rm -d \
  --name redis\
  -p 6379:6379 \
  redis --requirepass StrongPassword
````
### Using the services script:
You can use the ./service.sh script to bring the Podman containers up and down, and to clean up the database.

To start the containers:
```zsh
> ./services.sh up
```

To stop the containers:
```
> ./services.sh down
```

To reset the data volume:
```
> ./services.sh down
> ./services.sh clean
> ./services.sh up
```
## Server Layout
This server uses Actix for web server support. Actix is a typical routers / handlers style service.  The routes for the webauthn endpoints are defined in [routes](./src/webauthn/routes.rs). You can see the linked handlers from there.

### The Model
The entire WebAuthn model is defined in [model](./src/webauthn/model).