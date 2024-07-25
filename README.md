# Pixelpay

Pixelpay is a simple blockchain project built with gRPC.

## Project Components

The project consists of three main components:

1. Node
2. Wallet
3. Miner

## Building the Project

To build the project components, use the following commands:

```bash
go build -o pixelpay-node ./node/main.go
go build -o pixelpay-wallet ./wallet/main.go
go build -o pixelpay-miner ./miner/main.go
```

## Running with Docker

Alternatively, you can run the node using Docker:

```bash
git clone https://github.com/adibfahimi/pixelpay
cd pixelpay
docker build -t pixelpay-node ./node
docker run -p 3000:3000 pixelpay-node
```

## Using the Wallet and Miner

After setting up the node, you can use with the wallet or miner. Use the following commands to see available options:

```bash
./pixelpay-wallet -help
./pixelpay-miner -help
```
