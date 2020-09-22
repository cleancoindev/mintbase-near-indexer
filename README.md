# Mintbase indexer
From Scratch, run a VM. It takes a long time to sync, half a day to download the headers and another half to catch up, doing this on a local machine will drive you insane. Everytime you turn off your machine the blocks will fly by and if it gets lost for a day, you have to start over  

What separates a production grade app on blockchain is the use of a proper indexer like what we use on Ethereum [TheGraph](https://thegraph.com/explorer/subgraph/nategeier/mintbase), so dapps can pull readable data from an easily quieriable state. Relying on the blockchain itself ends up causing the programming to add loops on top of loops on top of loops of needless code adding more render time and frustration to the developer. 


## VM used: 
e2-standard-4 (4 vCPUs, 16 GB memory) 


## Install essentials, git, dev tools, librocksdb, rust

```
sudo apt install git-all
sudo apt-get install build-essential
sudo apt-get install llvm clang librocksdb-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
## Close Terminal and reboot 
`cargo` and `rustup` should now be in your path

## Add git ssh keys to your github profile
```
$ ssh-keygen -t rsa -b 4096 -C "nate@mintbase.io"
$ cat ~/.ssh/id_rsa.pub
```

## Init the indexer core

```
$ mkdir ~/.near && mkdir ~/.near/testnet
$ git clone git@github.com:nearprotocol/nearcore.git
$ cd nearcore/tools/indexer/example
$ cargo run --release -- --home-dir ~/.near/testnet init --chain-id testnet --download
$ rm -rf ~/near/.testnet/config.json
```


```
$ git clone https://github.com/Mintbase/mintbase-near-indexer
$ mv ~/mintbase-near-inder/config.json ~/.ssh/testnet/
```

## Start The indexer 
```
$ ulimit -n 2100
$ cd mintbase-near-indexer/indexer/
$ cd mintbase-near-inder
```



## Running the docker postgres 

Running the postgress db:

```bash
bash scripts/start-docker.sh
```

This will create a local copy of the Mintbase Indexer PostgreSQL database and run a REST api server on port **:3001**