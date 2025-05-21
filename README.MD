# Project description

That is an exercise project for learning how user can interact with Ethereum blockchain functionality. I didn't have a goal to make it production ready example. It uses Sepolia test network. 

It consists of **client CLI** app and local **Axum Server**.

CLI is used for creating wallets, transferring ETH tokens from one to another wallet, deploying smart contract and calling it via console client and server. CLI creates and signs transactions locally using private key files first. Then it sends signed transaction to local server and then to blockchain. Wallet balance is checked in test network (Sepolia) before proceeding with transaction. **The private key files do not leave CLI app**. 

## Module list
* /server - binary Axum server (running on 8080 port) should be started first for client CLI interaction
* /client - binary CLI app works with local server (and with Ethereum test network)
* /common - common modules shared between client and server

You should run server first and then client binary with command. You should understand the blockchain ideas and flow to properly run client commands.

### Run server (in debug mode)

I STRONGLY RECOMMEND you to have your own, personal Infura development key so you can run client and server using it via env variable that way:

```bash
INFURA_RPC_URL=https://sepolia.infura.io/v3/XXXXXXXX cargo run -p server
```
You can run server WITHOUT personal Infura key. But probably code WON'T work properly!
```bash
cargo run -p server
```

### Run client (in debug mode)

```bash
cargo run -p client -- create-wallet user1
```

I STRONGLY RECOMMEND you to have your own, personal Infura development key. There is a HIGH CHANCE the CLI examples won't work properly !

```bash
INFURA_RPC_URL=https://sepolia.infura.io/v3/4826ee26xxxxxxx cargo run -p client -- rest_of_commands
```

CLI app runs one command (with parameters) one time and then exits.

## Operation list (client commands)

### Create wallet
Generate two wallet files with names like 'user1.private' and 'user1.public' in current directory.
```bash
cargo run -p client -- create-wallet user1
```

The 'name' is a wallet name. It can be any string valid for local file name creation.
OR general usage
```bash
cargo run -p client -- create-wallet name
```

### Get wallet balance

The 'wallet-name' is a wallet address in hex format. Local server MUST be running to get balance.

```bash
cargo run -p client -- get-balance 0x3ab1d9D1aA3d97181FdA1a1Ff747009xxxxxxxx
```

The 'wallet-name' can be a wallet private key (name) stored as local file in current directory. JUST FOR TESTING PURPOSES.

```bash
cargo run -p client -- get-balance user1
```

OR general usage is following:
```bash
cargo run -p client -- get-balance wallet-name
```

Reads wallet file (wallet-name.private), gets balance from server and prints result.

### Transfer money from one wallet to another address

The 'sender-name' is local private key file (from-name.private) in current directory. That waller should HAVE amount of ETH tokens to transfer from. You can 'mine' tokens in test network (Sepolia or other) by using 'faucet'. How to do that is out of scope of this project.

The 'receiver-address' can be
- a 'wallet-name' is a wallet address in hex format.
- a local private key file (user1.private) in current directory. JUST FOR TESTING PURPOSES.

The 'amount' is ethereum token not negative amount as FLOAT/INT number. Examples are 0.00001, 0.1, 1. The specified amount (with transaction fee) available in wallet is checked before proceeding with transaction.

```bash
cargo run -p client -- transfer user1 0x3ab1d9D1aAxxxxxx 0.0001
```

OR general usage
```bash
cargo run -p client -- transfer sender-name receiver-address amount
```

### Deploy compiled smart contract
I've put already prepared and compiled smart contract files in /client directory.

The smart contract example can be called for 'storing' new value ONLY by OWNER. It can be called for 'reading' value by ANYONE. Deployment is done by OWNER one time and later can be called using it hash.

```bash
cargo run -p client -- deploy-contract user1 MessageStorage
```

OR general usage
```bash
cargo run -p client -- deploy binary-smart-contract-file-name wallet-name
```

A 'binary-smart-contract-file-name' is compiled smart contract binary file name (without extension). We have a precompiled binary file in /client/MessageStorage.bin

A 'wallet-name' is local private key file (wallet-name.private) in current directory. That wallet becomes owner of deployed smart contract.

#### How compile a smart contract from source

There is a simple smart contract example precompiled and included into project. See contract content in /client/smart_contract.sol. It was compiled into binary files by locally running 'solc' compiler command below. 
```bash
./solc-static-linux --bin --abi --overwrite -o ./client ./client/smart_contract.sol
```

You can find two prepared files in /client directory: **MessageStorage.abi** and **MessageStorage.bin**. I've created them beforehand. 

Compiler for contract can be [downloaded by link from GitHub repository](https://github.com/ethereum/solidity/releases/). I've used 'solc-static-linux' binary for Linux.
There is a copy of compiled contract in /common/src/MessageStorage.abi


### Read value from Deployed smart contract

Contract's method for reading value is public. The included contract doesn't have restriction to read that value.

An example is following:
```bashx
cargo run -p client -- read-message 0x794dade1fDe57801ADD4D10D19521xxxxxxxxxxx
```bash```

OR general usage
```bash
cargo run -p client -- read-message contract-address
```

### Store value in Deployed smart contract

It can be called for 'storing' new value by OWNER ONLY (the wallet that deployed smart contract). I STRONGLY RECOMMEND you to have your own, personal Infura development key. There is a HIGH CHANCE the example won't work properly !

```bash
INFURA_RPC_URL=https://sepolia.infura.io/v3/4826ee26503249dxxxxxx cargo run -p client -- store-message user1 0x794dade1fDe57801ADD4D10D1952140be57bBF9E "new contract message 1"
```

OR general usage
```bash
cargo run -p client -- store-message owner-wallet-name contract-address "new stored value"
```

# The scenario flow to run smart contract example
See documentation in [docs/README.MD](docs/README.MD)
It explains a possible scenarios to run smart contract example. What steps are needed see there.
