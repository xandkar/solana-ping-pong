#! /bin/sh

CLIENT_KEYPAIR_PATH="$HOME/.config/solana/id.json"
PROGRAM_KEYPAIR_PATH='./target/deploy/program-keypair.json'
CLUSTER_URL='http://127.0.0.1:8899'

./target/debug/client \
    "$CLIENT_KEYPAIR_PATH" \
    "$PROGRAM_KEYPAIR_PATH" \
    "$CLUSTER_URL"
