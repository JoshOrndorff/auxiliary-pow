# Auxiliary Proof of Work

A Substrate pallet (and adjascent tools) to use Proof of Work as a probabilistic finality gadget.

## Abstract

Classic Proof of Work blockchains like Bitcoin, Monero, Ethereum 1.0, use the PoW for two purposes. Firstly, for the right to produce the next block sometimes called "authorship" or "leader election". Secondly, as a probabilistic finality gadget, a metric for determining which chain is the "real" chain.

By contrast, classic PoS chains like Cosmos, Polkadot, and Ethereum 2.0, use a PoS for leader election and for a deterministic finality gadget. For example Polkadot uses Babe and Grandpa.

Some designs, such as [EIP-1011](https://eips.ethereum.org/EIPS/eip-1011) propose using PoW for leader election only, and installing a separate deterministic PoS finality gadget. Following this line of thinking, the Substrate framework separates the roles of leader election and finality cleanly through dedicated traits for each. The Substrate recipes even has an [example of PoW/PoS hybrid](https://substrate.dev/recipes/hybrid-consensus).

AFAIK, nobody has suggested using PoW for probabilistic finality only. The Auxiliary PoW pallet allows exactly that.

## Applications

### PoW/PoS hybrid

The Auxiliary PoW pallet allows EIP-1011-style hybrids in reverse. That is to say using a Proof of Stake leader election mechanism, coupled with Proof of Work finality.

### Dual PoW

The Auxiliary PoW pallet enables a pure PoW chain where the leader election PoW is entirely orthogonal to the finality PoW.Using a separate PoW for finality has several advantages over recycling the leader election PoW.

Firstly, miners who exceed a minimum threshold hashrate are reliably able to contribute work to the chain's finality. This avoids the wasted work that is orphaned off in classical PoW chains, as well as the centralization around mining pools.

Secondly, finality flows in gradually as work accumulates in the transaction pool. This helps miners converge on the "best" chain to mine before the next block is authored.

## Run the Demo

docker run TODO

## Pallet Implementation

The

## Use the pallet in your project

```toml

```

## Build this repo

Setup Substrate development environment (link devhub)
C
