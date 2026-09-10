# nameless

Uhh... the bot?

> I will write a proper README soon:tm:

## If you had configured everything based on `config.rs`

```sh
docker run \
    --env-file .env \              # optional, but will make your life less miserable.
    --name nameless \
    --mount type=bind,src=./nameless.db,dst=<to-what-you-configured>,bind-create-src \
    --rm -it \
    ghcr.io/swyrin/nameless:latest # pick a specific version, unless you love edging.
```

## If you had configured only `TOKEN`

```sh
docker run \
    --env-file .env \              # optional, but will make your life less miserable.
    --name nameless \
    --mount type=bind,src=./nameless.db,dst=/nameless.db,bind-create-src \
    --rm -it \
    ghcr.io/swyrin/nameless:latest # pick a specific version, unless you love edging.
```
