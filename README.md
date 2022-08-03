# decks-v2

This is a repo representing a uniswap like DEX (when finished), which runs as a substrate parachain.

I have used the `parachain-template-node`, and simply added a couple of pallets (pallet-dex and pallet-kitty) to it.

To test the `parachain-template-node` there needs to be a relay chain network to use as the parachain collator needs to register with the relay chain. 

Unfortunately, after everything compiled and I was able to build my project, I was unable to build my docker image which would let me bootstrap the relay chain environment with zombienet and test that it works correctly.

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

Zombienet lets you run the parachain on top of a relay chain testnet which you can deploy locally. I decided to run my entire environment in a local Kubernetes cluster using kind: https://github.com/kubernetes-sigs/kind 

There are some scripts located in `env/` which lets you spin up a local `kind` cluster with 1 master node, and 3 worker nodes.

Assuming you have `kind` installed, you can create the cluster how I did with:

```
env/ $ ./local-k8s.sh
```

This also creates a docker image registry which the worker nodes can pull images from if necessary.

Then you can deploy the Zombienet with

```
env/ $ ./local-parachain.sh
```

or use `zombienet-macos` directly:

```
env/ $ zombienet-macos spawn conf/zombienet.toml
```

note that it takes a `.toml` which specifies the network architecture.

### Zombienet Test Problems

I digged a bit found out this issue: https://github.com/paritytech/cumulus/issues/483

There seems to be on-going work in making using parachain-templates support `--dev` mode. That would have helped to evalaute the parachain I submitted as I had deleted the polkadot binaries, so could not start up the realy chain natively in time. 

If time allowed I would port it to the substrate-node-template directory, and build it there.