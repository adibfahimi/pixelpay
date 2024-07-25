package mine

import (
	"context"
	"log"
	"time"

	pb "github.com/adibfahimi/pixelpay/api"
	"github.com/adibfahimi/pixelpay/utils"
)

func MineSolo(client pb.BlockchainServiceClient, nodeAddress string, address string) {
	ctx := context.Background()

	log.Println("Mining block ...")

	for {
		log.Println("Getting block ...")
		res, err := client.GetBlockToMine(ctx, &pb.Empty{})
		if err != nil {
			log.Printf("Error getting block: %v\n", err)
			time.Sleep(15 * time.Second)
			continue
		}

		var block = res.Block
		var reward = res.Reward

		log.Println("Block received.")

		coinBaseTx := pb.Tx{
			FromAddress: "",
			ToAddress:   address,
			Signature:   "",
			Hash:        "",
			Amount:      reward,
			Timestamp:   time.Now().Unix(),
		}

		coinBaseTx.Hash = utils.CalculateTxHash(&coinBaseTx)
		block.Txs = append(block.Txs, &coinBaseTx)
		block.MerkleRoot = utils.CalculateMerkleRoot(block)

		log.Println("Mining block ...")

		if err := utils.MineBlock(block); err != nil {
			log.Printf("Error mining block: %v\n", err)
			continue
		}

		log.Println("Block mined. With nonce:", block.Nonce)

		if _, err := client.AddBlock(ctx, block); err != nil {
			log.Printf("Error sending block: %v\n", err)
			continue
		}

		log.Println("Block sent. Mining next block ...")
	}
}
