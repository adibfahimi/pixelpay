package wallet

import (
	pb "github.com/adibfahimi/pixelpay/api"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
)

func (w *Wallet) SignTx(tx *pb.Tx) error {
	privateKeyBytes, err := hexutil.Decode(w.PrivateKey)
	if err != nil {
		return err
	}

	privateKey, err := crypto.ToECDSA(privateKeyBytes)
	if err != nil {
		return err
	}

	hashBytes, err := hexutil.Decode(tx.Hash)
	if err != nil {
		return err
	}

	signature, err := crypto.Sign(hashBytes, privateKey)
	if err != nil {
		return err
	}

	tx.Signature = hexutil.Encode(signature)

	return nil
}
