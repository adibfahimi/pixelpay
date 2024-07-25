package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"time"

	pb "github.com/adibfahimi/pixelpay/api"
	"github.com/adibfahimi/pixelpay/utils"
	"github.com/adibfahimi/pixelpay/wallet/wallet"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func SendTransaction(ctx context.Context, client pb.BlockchainServiceClient, wallet wallet.Wallet, amount int64, receiver string) {
	if amount > wallet.Balance {
		fmt.Println("you don't have enough balance")
		return
	}

	tx := pb.Tx{
		FromAddress: wallet.Address,
		ToAddress:   receiver,
		Signature:   "",
		Hash:        "",
		Amount:      amount,
		Timestamp:   time.Now().Unix(),
	}

	tx.Hash = utils.CalculateTxHash(&tx)

	if err := wallet.SignTx(&tx); err != nil {
		log.Fatalf("an error occured while signing tx: %v", err)
	}

	res, err := client.SendTx(ctx, &tx)
	if err != nil {
		log.Fatalf("an error occured while sending tx: %v", err)
	}

	if res.Status == pb.TxResponse_PENDING {
		fmt.Printf("Transaction sent successfully! tx hash: %s\n", res.TxHash)
	} else {
		fmt.Printf("Transaction failed! tx hash: %s\n", res.TxHash)
	}
}

func main() {
	isSend := flag.Bool("send", false, "Send command")
	isBalance := flag.Bool("balance", false, "balance command")
	receiver := flag.String("receiver", "", "Set receiver address")
	amount := flag.Int64("amount", 0, "Set amount to send")
	nodeAddress := flag.String("node", "http://localhost:3000", "Set node address")

	flag.Parse()

	wallet := wallet.LoadWallet(*nodeAddress)

	conn, err := grpc.NewClient("localhost:3000", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := pb.NewBlockchainServiceClient(conn)
	ctx := context.Background()

	balanceRes, err := client.GetBalance(ctx, &pb.AddressRequest{
		Address: wallet.Address,
	})

	if err != nil {
		log.Fatalf("an error occured while getting balance: %v", err)
		return
	}

	wallet.Balance = balanceRes.Balance

	if *isBalance {
		fmt.Printf("Your balance is: %d PXL\n", wallet.Balance)
	} else if *isSend {
		SendTransaction(ctx, client, wallet, *amount, *receiver)
	} else {
		fmt.Println("Command not found :(")
	}
}
