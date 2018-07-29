# IOJCOREFER

A toy (Paste Bin) Application using Redis as Store


### Installing 

With Docker:

```
    git clone git@github.com:EarvinKayonga/hashing.git
    cd hashing
    docker-compose -f docker/docker-compose.yml up
```

From Source:

```
    git clone git@github.com:EarvinKayonga/hashing.git
    cd hashing
    cargo run --release -- -r redis://<socket of a running redis>
```