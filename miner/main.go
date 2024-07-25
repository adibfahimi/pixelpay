package main

import (
	"flag"
	"fmt"
	"log"

	pb "github.com/adibfahimi/pixelpay/api"
	"github.com/adibfahimi/pixelpay/miner/mine"
	"github.com/adibfahimi/pixelpay/utils"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func main() {
	conn, err := grpc.NewClient("localhost:3000", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := pb.NewBlockchainServiceClient(conn)

	nodeAddress := flag.String("node", "", "set node address for solo mining")
	poolAddress := flag.String("pool", "", "set pool address for pool mining")
	address := flag.String("address", "", "set wallet address")

	flag.Parse()

	if *address == "" {
		fmt.Println("no wallet address provided")
		return
	}

	if !utils.IsValidAddress(*address) {
		fmt.Println("invalid wallet address")
		return
	}

	if *nodeAddress != "" {
		fmt.Println("start solo mining")
		mine.MineSolo(client, *nodeAddress, *address)
	} else if *poolAddress != "" {
		fmt.Println("start pool mining")
		// mine.MinePool(*poolAddress, *address)
	} else {
		fmt.Println("no mode selected. For more information, use --help")
	}
}
