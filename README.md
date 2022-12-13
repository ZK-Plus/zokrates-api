# Setup 

## Docker
If you have Docker installed in your machine you can simply run:
```bash
$ docker-compose up
```

## locally:
```bash
$ RUST_MIN_STACK=1000000 cargo +nightly run
```
Or with hot-loading:
```bash
$ RUST_MIN_STACK=1000000 cargo +nightly watch -x run
```

# Run production container

1 - build image of Dockerfile.prod
2 - run container:
```sh
docker run -p 8000:8000 blockchainpreprocessor-prod:latest 
```

# testing

Run tests:
```bash
$ cargo test
```

# Additionally
Rust format:
```bash
$ cargo fmt
```

Rust lint:
```bash
$ cargo clippy
```