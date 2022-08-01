# Local Parachain network

Run:

- local-k8s.sh to create local kubernetes cluster (kind) with integrated registry
- local-parachain.sh to use `zombienet-macos` to create a test cluster of
	- 2 relay chain nodes
	- 2 parachain collator node 

For the local parachain network, check out `conf/zombienet.toml`.
