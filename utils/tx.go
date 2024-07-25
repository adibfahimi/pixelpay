package utils

import (
	"bytes"
	"fmt"
	"regexp"

	pb "github.com/adibfahimi/pixelpay/api"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
)

func CalculateTxHash(tx *pb.Tx) string {
	return crypto.Keccak256Hash([]byte(fmt.Sprintf("%s%s%d%d", tx.FromAddress, tx.ToAddress, tx.Amount, tx.Timestamp))).Hex()
}

func IsValidTx(tx *pb.Tx) error {
	if tx.FromAddress == "" {
		return fmt.Errorf("invalid transaction: from address is empty")
	}

	if tx.ToAddress == "" {
		return fmt.Errorf("invalid transaction: to address is empty")
	}

	if tx.Amount <= 0 {
		return fmt.Errorf("invalid transaction: amount is invalid")
	}

	if tx.Signature == "" {
		return fmt.Errorf("invalid transaction: signature is empty")
	}

	if tx.Hash == "" {
		return fmt.Errorf("invalid transaction: hash is empty")
	}

	if tx.Timestamp <= 0 {
		return fmt.Errorf("invalid transaction: timestamp is invalid")
	}

	if tx.Hash != CalculateTxHash(tx) {
		return fmt.Errorf("invalid transaction: hash is invalid")
	}

	return nil
}

func VerifyTxSignature(tx *pb.Tx) error {
	pubKeyBytes, err := hexutil.Decode(tx.FromAddress)
	if err != nil {
		return fmt.Errorf("failed to decode: %v", err)
	}

	signatureBytes, err := hexutil.Decode(tx.Signature)
	if err != nil {
		return fmt.Errorf("failed to decode: %v", err)
	}

	hashBytes, err := hexutil.Decode(tx.Hash)
	if err != nil {
		return fmt.Errorf("failed to decode: %v", err)
	}

	realPubKey, err := crypto.SigToPub(hashBytes, signatureBytes)
	if err != nil {
		return fmt.Errorf("failed to recover public key: %v", err)
	}

	realPubKeyBytes := crypto.CompressPubkey(realPubKey)

	if bytes.Equal(pubKeyBytes, realPubKeyBytes) {
		return fmt.Errorf("invalid public key")
	}

	if !crypto.VerifySignature(realPubKeyBytes, hashBytes, signatureBytes[:64]) {
		return fmt.Errorf("invalid signature")
	}

	return nil
}

func IsValidAddress(address string) bool {
	pattern := "^0x[0-9a-fA-F]{40}$"
	regex := regexp.MustCompile(pattern)
	return regex.MatchString(address)
}
