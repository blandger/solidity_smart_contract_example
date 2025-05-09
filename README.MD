Simple exercise to use Ethereum blockchain by creating wallets, deploying smart contract and calling it via console client and server.

### Run client in debug mode

> cargo run -p client -- create-wallet user-1

### Run server in debug mode

> cargo run -p server

## Client commands list

### Create wallet

> cargo run -p client -- create-wallet your-wallet-name

### Get wallet balance

> cargo run -p client -- get-balance your-wallet-name

### Transfer from one wallet to another

> cargo run -p client -- transfer from-wallet-name to-wallet-name amount


# Flow to run included smart contract example
Below I explain how to run included smart contract example. What steps are needed to run it.

## Prepare two or three wallets with test ETH tokens

### Create first, main wallet

> cargo run -p client -- create-wallet main-wallet
