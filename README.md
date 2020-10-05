# Mintbase indexer

From Scratch, run a VM. It takes a long time to sync, half a day to download the headers and another half to catch up, doing this on a local machine will drive you insane. Everytime you turn off your machine the blocks will fly by and if it gets lost for a day, you have to start over

What separates a production grade app on blockchain is the use of a proper indexer like what we use on Ethereum [TheGraph](https://thegraph.com/explorer/subgraph/nategeier/mintbase), so dapps can pull readable data from an easily quieriable state. Relying on the blockchain itself ends up causing the programming to add loops on top of loops on top of loops of needless code adding more render time and frustration to the developer.

## VM used:

e2-standard-4 (4 vCPUs, 16 GB memory)

## Install essentials, git, dev tools, librocksdb, rust

```
sudo apt update && sudo apt upgrade
sudo apt install git-all screen build-essential
sudo apt-get install llvm clang librocksdb-dev libpq-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Close Terminal and reboot

`cargo` and `rustup` should now be in your path so running `cargo --version` should show something

## Add git ssh keys to your github profile

```
$ ssh-keygen -t rsa -b 4096 -C "nate@mintbase.io"
$ cat ~/.ssh/id_rsa.pub
```

## Init the Indexer Core

```
$ mkdir ~/.near && mkdir ~/.near/testnet
$ git clone git@github.com:nearprotocol/nearcore.git
$ cd nearcore/tools/indexer/example
$ cargo run --release -- --home-dir ~/.near/testnet init --chain-id testnet --download
```

## Clone the Mintbase indexer

What you'll edit

```
$ cd ~
$ git clone https://github.com/Mintbase/mintbase-near-indexer
```

### Istall Docker:

[Install Docker on Debian](https://docs.docker.com/engine/install/debian/)
[INstall Docker compose](https://docs.docker.com/compose/install/)

### Permantly set the ulimit

[Increase the number of files open](https://medium.com/@muhammadtriwibowo/set-permanently-ulimit-n-open-files-in-ubuntu-4d61064429a)

## Move the config file over to `.near/testnet`

Look at this file a bit, once you start the indexer below the config only gets read once, notice the shard number and block horizon

```
$ rm -rf ~/near/.testnet/config.json
$ mv ~/mintbase-near-indexer/config.json ~/.near/testnet/
```

## Increase space limits and Start The indexer

```
$ cd mintbase-near-indexer/indexer/
$ cargo run --release -- --home-dir ~/.near/testnet run
```

## Running the docker postgres

Install docker and docker-compose
Running the postgress db:

```bash
bash scripts/start-docker.sh
```

This will create a local copy of the Mintbase Indexer PostgreSQL database and run a REST api server on port **:3001**

## Helpful Tools

- 3.23.154.30
- sudo du -sh ~/.near/testnet/data/

## Keep Screen Running

```cd
$ screen
//  Ctrl + A, and then Ctrl + D
$ screen -ls | grep pts | cut -d. -f1 | awk '{print $1}' | xargs kill

```
