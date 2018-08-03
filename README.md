# IOJCOREFER

A toy (Paste Bin) Application using Redis as Store


### Installing 

With Docker:

```
    git clone git@github.com:EarvinKayonga/iojcorefer.git
    cd iojcorefer
    docker-compose -f docker/docker-compose.yml up
```

From Source:

```
    git clone git@github.com:EarvinKayonga/iojcorefer.git
    cd iojcorefer
    cargo run --release -- -r redis://<socket of a running redis>
```