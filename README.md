# decks-v2

This is a repo representing a uniswap like DEX (when finished), which runs as a substrate parachain.

I have used the parachain-template-node, and simply added a couple of pallets (pallet-dex and pallet-kitty) to it.

## Instructions

You can run the node in different ways:

1. Standalone native
2. Standalone in Docker
3. In local rococo testnet

### Standalone

Simply compile the parachain node:

```
$ cd parachain/
$ cargo build --release
```

then start the node:

```
$ ./target/release/parachain-node --help 
```

### Standalone in Docker

**NB**

Should have Docker installed for this. I have only tested on an Intel based macBook Pro.

First you have to build the Docker image so it has the parachain binary in it.
This can be done in two ways:

- compile inside the docker image, as defined in `decks-v2/parachain/Dockerfile`
- inject a pre-compiled binary into a new image, as defined in `decks-v2/parachain/Dockerfile.injected`

It is faster to compile outside of the docker container, hence do this:

```
$ cd parachain
$ cargo build --release

# Now to build the image
$ docker build -f Dockerfile.injected -t localhost:5001/parachain-node:latest .
```

This will build the image, and you should see it with:

```
$ docker images
REPOSITORY                          TAG       IMAGE ID       CREATED         SIZE
localhost:5001/parachain-node      latest    d3dd0426c869   4 days ago      220MB
```

Then you can run it like this:

```
$ docker run localhost:5001/parachain-node --version
```

### In a local parachain testnet environment

This can be done using Zombienet: https://github.com/paritytech/zombienet.

To use zombienet, make sure you have it installed, there are releases availble here: https://github.com/paritytech/zombienet/releases

#### Zombienet

Zombienet lets you run the parachain on top of a relay chain testnet. As I am used to Kubernetes tooling, I decided to run my entire environment in Kubernetes. There are some scripts located in `env/` which lets you do this as well if you are interested.
