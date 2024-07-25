package main

import (
	"context"
	"log"
	"net"
	"time"

	pb "github.com/adibfahimi/pixelpay/api"
	"github.com/adibfahimi/pixelpay/utils"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type server struct {
	pb.UnimplementedBlockchainServiceServer

	difficulty   int64
	chain        []*pb.Block
	pendingBlock *pb.Block
	miningReward int64
}

func (s *server) getLatestBlock() *pb.Block {
	return s.chain[len(s.chain)-1]
}

func (s *server) GetAllBlocks(ctx context.Context, req *pb.Empty) (*pb.BlocksResponse, error) {
	return &pb.BlocksResponse{
		Blocks: s.chain,
	}, nil
}

func (s *server) GetBlockByHash(ctx context.Context, req *pb.HashRequest) (*pb.BlockResponse, error) {
	for _, block := range s.chain {
		if block.Hash == req.Hash {
			return &pb.BlockResponse{
				Block: block,
			}, nil
		}
	}

	return nil, status.Error(codes.NotFound, "Block not found")
}

func (s *server) AddBlock(ctx context.Context, req *pb.Block) (*pb.BlockResponse, error) {
	if len(s.pendingBlock.Txs) == 0 {
		return nil, status.Error(codes.InvalidArgument, "No transactions to add")
	}

	latestBlock := s.getLatestBlock()

	if latestBlock.Hash != req.PreviousHash {
		return nil, status.Error(codes.InvalidArgument, "Invalid previous hash")
	}

	if err := utils.IsValidBlock(req); err != nil {
		return nil, status.Error(codes.InvalidArgument, err.Error())
	}

	s.chain = append(s.chain, req)
	s.pendingBlock = &pb.Block{
		Hash:         "",
		PreviousHash: latestBlock.Hash,
		MerkleRoot:   "",
		Txs:          []*pb.Tx{},
		Timestamp:    time.Now().Unix(),
		Nonce:        0,
		Difficulty:   s.difficulty,
	}

	return &pb.BlockResponse{
		Block: req,
	}, nil
}

func (s *server) GetTxByHash(ctx context.Context, req *pb.HashRequest) (*pb.Tx, error) {
	for _, block := range s.chain {
		for _, tx := range block.Txs {
			if tx.Hash == req.Hash {
				return tx, nil
			}
		}
	}

	return nil, status.Error(codes.NotFound, "Transaction not found")
}

func (s *server) GetBlockToMine(ctx context.Context, req *pb.Empty) (*pb.BlockToMineResponse, error) {
	if len(s.pendingBlock.Txs) == 0 {
		return nil, status.Error(codes.InvalidArgument, "No transactions to add")
	}

	return &pb.BlockToMineResponse{
		Block:  s.pendingBlock,
		Reward: s.miningReward,
	}, nil
}

func (s *server) SendTx(ctx context.Context, req *pb.Tx) (*pb.TxResponse, error) {
	if err := utils.IsValidTx(req); err != nil {
		return nil, status.Error(codes.InvalidArgument, err.Error())
	}

	s.pendingBlock.Txs = append(s.pendingBlock.Txs, req)

	return &pb.TxResponse{
		TxHash: req.Hash,
		Status: pb.TxResponse_PENDING,
	}, nil
}

func (s *server) GetBalance(ctx context.Context, req *pb.AddressRequest) (*pb.BalanceResponse, error) {
	balance := int64(0)

	for _, block := range s.chain {
		for _, tx := range block.Txs {
			if tx.FromAddress == req.Address {
				balance -= tx.Amount
			}

			if tx.ToAddress == req.Address {
				balance += tx.Amount
			}
		}
	}

	return &pb.BalanceResponse{
		Balance: balance,
	}, nil
}

func main() {
	lis, err := net.Listen("tcp", ":3000")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	grpcServer := grpc.NewServer()

	pb.RegisterBlockchainServiceServer(grpcServer, &server{})

	log.Printf("server listening at %v", lis.Addr())
	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
