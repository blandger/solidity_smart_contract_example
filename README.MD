# Project description

That is a simple exercise using Ethereum blockchain functionality. I didn't have a goal to make it production ready example It uses Sepolia test network. 

It consists of client CLI app and local Axum Server (running on 8080 port).

CLI is used for creating wallets, transferring ETH tokens from one to another wallet, deploying smart contract and calling it via console client and server. CLI creates and signs transactions locally using private key files first. Then it sends signed transaction to local server and then to blockchain. Wallet balance is checked in test network (Sepolia) before proceeding with transaction. The private key files do not leave CLI app. 

## Operation list

You should run server first and then client. You should understand the blockchain flow to properly run client commands.

### Run server (in debug mode)

I STRONGLY RECOMMEND you to have your own, personal Infura development key so you can run server using it via env variable that way.

> INFURA_RPC_URL=https://sepolia.infura.io/v3/XXXXXXXX cargo run -p server

You can run server WITHOUT personal Infura key. But probably it WON'T work properly!.
> cargo run -p server

### Run client (in debug mode)

> cargo run -p client -- create-wallet user-1

I STRONGLY RECOMMEND you to have your own, personal Infura development key. There is a HIGH CHANCE the CLI examples won't work properly !

> INFURA_RPC_URL=https://sepolia.infura.io/v3/4826ee26xxxxxxx cargo run -p client -- ....

CLI app runs one command (with parameters) one time and then exits.

## Client commands list

### Create wallet
Create two wallet files with names like 'user1.private' and 'user1.public' in current directory.
> cargo run -p client -- create-wallet user1

The 'name' is a wallet name. It can be any string valid for local file name creation.
> cargo run -p client -- create-wallet name

### Get wallet balance

The 'wallet-name' is a wallet address in hex format. Server MUST be running to get balance.

> cargo run -p client -- get-balance 0x3ab1d9D1aA3d97181FdA1a1Ff747009xxxxxxxx

OR
The 'wallet-name' can be a private key of wallet stored as local file in current directory. Just for testing purposes.

> cargo run -p client -- get-balance user1

General usage is following:
> cargo run -p client -- get-balance wallet-name

Reads wallet file (wallet-name.private), gets balance from server and prints result.

### Transfer from one wallet to another

The 'from-name' is local private key file (from-name.private) in current directory. That waller should HAVE amount of ETH tokens to transfer from. You can 'mine' tokens in test network (Sepolia or other) by using 'faucet'. How to do that is out of scope of this project.

The 'to-name' can be
- a 'wallet-name' is a wallet address in hex format.
- a local private key file (to-name.private) in current directory. Just for testing purposes.

The 'amount' is ethereum token not negative amount as FLOAT/INT number. Examples are 0.00001, 0.1, 1. The specified amount (with transaction fee) available in wallet is checked before proceeding with transaction.

> cargo run -p client -- transfer from-name 0x3ab1d9D1aA........ 0.0001

OR 

> cargo run -p client -- transfer from-name to-name amount

### Deploy compiled smart contract
I've put already prepared and compiled smart contract files in /client directory.

The smart contract example can be called for 'storing' new value ONLY by OWNER. It can be called for 'reading' value by ANYONE. Deployment is done by OWNER one time and later can be called using it hash.

> cargo run -p client -- deploy binary-smart-contract-file-name wallet-name

A 'binary-smart-contract-file-name' is compiled smart contract binary file name (without extension). 

A 'wallet-name' is local private key file (wallet-name.private) in current directory. That wallet becomes owner of deployed smart contract.

There is a simple smart contract example included in project. See contract text in /client/smart_contract.sol. It was compiled into binary files by locally running 'solc' compiler command. 

> cd client/

> ./solc-static-linux --bin --abi --overwrite -o ./ smart_contract.sol

You can find two binary files in /client directory: **MessageStorage.abi** and **MessageStorage.bin**. I've created them beforehand. Compiler for contract can be [downloaded by link](https://github.com/ethereum/solidity/releases/).
There is a copy of compiled contract in /common/src/MessageStorage.abi


### Read value from Deployed smart contract

Contract's method for reading value can be called by ANY user (the existing wallet). The included contract doesn't have restriction to read value.

> cargo run -p client -- read-message contract-address

An example is following:
> cargo run -p client -- read-message 0x794dade1fDe57801ADD4D10D19521xxxxxxxxxxx

### Store value in Deployed smart contract

It can be called for 'storing' new value by OWNER ONLY (the wallet that deployed smart contract).

> cargo run -p client -- store-message owner-wallet-name contract-address "new stored value"


# Flow to run included smart contract example
Below I explain how to run included smart contract example. What steps are needed to run it.

## Prepare two or three wallets with test ETH tokens

### Create first, main wallet

> cargo run -p client -- create-wallet main-wallet


> INFURA_RPC_URL=https://sepolia.infura.io/v3/4826ee26xxxxxxx cargo run -p client -- store-message user1 0x794dade1fDe57801ADD4D10D1952140be57bBF9E "new contract message 1"


> INFURA_RPC_URL=https://sepolia.infura.io/v3/4826ee26xxxx cargo run -p client -- read-message 0x794dade1fDe57801ADD4D10D1952140be57bBF9E
