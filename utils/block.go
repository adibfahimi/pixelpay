package utils

import (
	"errors"
	"fmt"
	"math/big"

	pb "github.com/adibfahimi/pixelpay/api"
	"github.com/adibfahimi/pixelpay/config"
	"github.com/ethereum/go-ethereum/crypto"
)

func CalculateBlockHash(b *pb.Block) string {
	b.MerkleRoot = CalculateMerkleRoot(b)

	return crypto.Keccak256Hash([]byte(fmt.Sprintf("%v%v%v%v", b.PreviousHash, b.MerkleRoot, b.Timestamp, b.Nonce))).Hex()
}

func CalculateMerkleRoot(b *pb.Block) string {
	var hashes []string

	if len(b.Txs) == 0 {
		return ""
	}

	for _, tx := range b.Txs {
		hashes = append(hashes, CalculateTxHash(tx))
	}

	for len(hashes) > 1 {
		var levelHashes []string

		if len(hashes)%2 != 0 {
			hashes = append(hashes, hashes[len(hashes)-1])
		}

		for i := 0; i < len(hashes); i += 2 {
			combined := fmt.Sprintf("%s%s", hashes[i], hashes[i+1])
			hash := fmt.Sprintf("%x", crypto.Keccak256([]byte(combined)))
			levelHashes = append(levelHashes, hash)
		}

		hashes = levelHashes
	}

	return hashes[0]
}

func IsValidBlock(block *pb.Block) error {
	if block.PreviousHash == "" {
		return fmt.Errorf("invalid block: previous hash is empty")
	}

	if block.Nonce == 0 {
		return fmt.Errorf("invalid block: nonce is invalid")
	}

	if block.Timestamp <= 0 {
		return fmt.Errorf("invalid block: timestamp is invalid")
	}

	if block.Hash == "" {
		return fmt.Errorf("invalid block: hash is empty")
	}

	if block.Hash != CalculateBlockHash(block) {
		return fmt.Errorf("invalid block: hash is invalid")
	}

	for _, tx := range block.Txs {
		if err := IsValidTx(tx); err != nil {
			return fmt.Errorf("invalid block: invalid transaction: %v", err)
		}
	}

	return nil
}

func MineBlock(b *pb.Block) error {
	target := new(big.Int)
	target.Exp(big.NewInt(2), new(big.Int).Sub(big.NewInt(256), new(big.Int).SetInt64(b.Difficulty)), nil)

	for {
		hash := CalculateBlockHash(b)
		hashInt := new(big.Int)
		hashInt.SetString(hash[2:], 16)

		if hashInt.Cmp(target) == -1 {
			b.Hash = hash
			return nil
		}

		b.Nonce++

		if b.Nonce > config.MAX_NONCE {
			return errors.New("block mining failed")
		}
	}
}
