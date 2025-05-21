##### Running server and client in release mode

As example let's do a release build. If you've built project using release mode then you can run project binaries using commands below :

Server can be run by
```bash
cd /target/release
INFURA_RPC_URL=https://sepolia.infura.io/v3/4826ee2650324xxxxxxxx ./server
```

Client can be run by
```bash
./client create-wallet user1
```

## An example for steps to run smart contract example

### 1. Create first, main wallet
We'll use it and initial money source filled with faucet test network. 

```bash
./client create-wallet main
```
You'll get two files: 'main.private' and 'main.public' in current directory. And you wallet address will be printed in console.

Let's say the address is '0x1ab1d9D1aAxxxxxx'

### 2. Mine some test ETH tokens using Faucet
Go to the following link
https://sepolia-faucet.pk910.de/
 and mine tokens (~ 0.1 ETH) to your main wallet address 0x1ab1d9D1aAxxxxxx

You will use 'main' for transferring money to other generated account.

### 3. Create second wallet
You will use it for contract deployment and storing new message to that contract.

```bash
./client create-wallet user1
```

Let's say the address is '0x2ab1d9D1aAxxxxxx'. See files 'user1.private' and 'user1.public' in current directory.

### 4. Transfer money to second wallet from main
Second wallet (user1) will be used for contract deployment and storing new message to that contract. So it will enough to need money for both operations.

```bash
./client transfer main 0x2ab1d9D1aAxxxxxx 0.08
```

### 5. Deploy smart contract by second wallet (it will be owner)

Current folder (e.g. /target/release/) should contain precompiled binary contract file = **MessageStorage.bin**. So you better to copy it. Otherwise, you'll see error like: _Error: Contract binary file 'MessageStorage.bin' is not found by path xxxx_

```bash
./client deploy-contract user1 MessageStorage
```
On success, you'll see message like: _Contract deployment SUCCESS! Contract address: "0xFCe007c135e90867a8c9143989BAbxxxxxxx"_
Keep contract address for further usage.

You can get error that you don't have enough money in your wallet to deploy contract, so you can repeat step 4.

### 6. Read data from smart contract
Try to read data from just deployed smart contract.

```bash
./client read-message 0xFCe007c135e90867a8c9143989BAbxxxxxxx
```

Most probably you'll get initial value from it: _Got message from contract: 'RetrieveMessageResponse { message: "Initial contract message", last_updated_block: None }'_

### 7. Store data in smart contract (by owner only)
Contract owner can store new message value into it.

```bash
INFURA_RPC_URL=https://sepolia.infura.io/v3/4826ee2650324xxxxxxxx ./client store-message user1 0xFCe007c135e90867a8c9143989BAbxxxxxxx "new contract message 1"
```

On success, you'll see message like: _New Value 'new contract message 1' successfully stored in contract by address: 0xFCe007c135e90867a8c9143989BAbxxxxxxx_

### 8. Check new message is stored in smart contract
See updated message from smart contract.

```bash
./client read-message 0xFCe007c135e90867a8c9143989BAbxxxxxxx
```

You should see message like: _Got message from contract: 'RetrieveMessageResponse { message: "new contract message 1", last_updated_block: None }'..._
