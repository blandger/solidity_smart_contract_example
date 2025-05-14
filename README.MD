# Project description

That is a simple exercise using Ethereum blockchain functionality. I didn't have a goal to make it production ready example It uses Sepolia test network. 

It consists of client CLI app and local Axum Server (running on 8080 port).

CLI is used for creating wallets, transferring ETH tokens from one to another wallet, deploying smart contract and calling it via console client and server. CLI creates and signs transactions locally using private key files first. Then it sends signed transaction to local server and then to blockchain. Wallet balance is checked in test network (Sepolia) before proceeding with transaction. The private key files do not leave CLI app. 

## Operation list

You should run server first and then client. You should understand the blockchain flow to properly run client commands.

### Run server (in debug mode)

It would be better for you to have your own, personal Infura development key so you can run server using it via env variable that way.

> INFURA_RPC_URL=https://sepolia.infura.io/v3/XXXXXXXX cargo run -p server

You can run server WITHOUT personal Infura key. But it works WORST.
> cargo run -p server

### Run client (in debug mode)

> cargo run -p client -- create-wallet user-1

CLI app runs one command (with parameters) one time and then exits.

## Client commands list

### Create wallet

> cargo run -p client -- create-wallet name

Creates wallet (two files with private and public keys) using specified name and stores it into current directory as local files.

### Get wallet balance

The 'wallet-name' is a wallet address in hex format. Server MUST be running to get balance.

> cargo run -p client -- get-balance 0x3ab1d9D1aA3d97181FdA1a1Ff747009474655263

OR
The 'wallet-name' can be a private key of wallet stored as local file in current directory. Just for testing purposes.

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

The smart contract example can be called for 'storing' new value ONLY by OWNER. It can be called for 'reading' value by ANYONE. Deployment is done by OWNER one time and later can be called using it hash.

> cargo run -p client -- deploy binary-smart-contract-file-name wallet-name

A 'binary-smart-contract-file-name' is compiled smart contract binary file name (without extension). 

A 'wallet-name' is local private key file (wallet-name.private) in current directory. That wallet becomes owner of deployed smart contract.

There is a simple smart contract example included in project. See contract text in /client/smart_contract.sol. It was compiled into binary files by locally running 'solc' compiler command. 

> cd client/

> ./solc-static-linux --bin --abi --overwrite -o ./ smart_contract.sol

You can find two binary files in /client directory: **MessageStorage.abi** and **MessageStorage.bin**. I've created them beforehand. Compiler for contract can be [downloaded by link](https://github.com/ethereum/solidity/releases/).


### Store value in Deployed smart contract

It can be called for 'storing' new value by OWNER ONLY (the wallet that deployed smart contract).

> cargo run -p client -- store wallet-name new-store-value


# Flow to run included smart contract example
Below I explain how to run included smart contract example. What steps are needed to run it.

## Prepare two or three wallets with test ETH tokens

### Create first, main wallet

> cargo run -p client -- create-wallet main-wallet
