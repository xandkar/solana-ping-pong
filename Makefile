PROGRAM_KEYPAIR := target/deploy/program-keypair.json
PROGRAM_BIN     := target/deploy/program.so
CLIENT_BIN      := target/debug/client

# local | dev | test
CLUSTER ?= local

CLUSTER_URL_LOCAL   := http://127.0.0.1:8899
CLUSTER_URL_DEVNET  := https://api.devnet.solana.com
CLUSTER_URL_TESTNET := https://api.testnet.solana.com

# XXX solana lib wants a URL always, while
#     solana bin wants a URL only for local, but a named net otherwise.
ifeq ($(CLUSTER),local)
    CLUSTER_URL := $(CLUSTER_URL_LOCAL)
    NET         := $(CLUSTER_URL_LOCAL)
    SOL         := 5
else ifeq ($(CLUSTER),dev)
    CLUSTER_URL := $(CLUSTER_URL_DEVNET)
    NET         := devnet
    SOL         := 2
else ifeq ($(CLUSTER),test)
    CLUSTER_URL := $(CLUSTER_URL_TESTNET)
    NET         := testnet
    SOL         := 1
else
    $(error Invalid cluster name: "$(CLUSTER)")
endif

.PHONY: all
all:
	@$(MAKE) --no-print-directory build
	@$(MAKE) --no-print-directory deploy
	@$(MAKE) --no-print-directory run

.PHONY: build
build:
	cargo build
	cargo build-bpf

.PHONY: deploy
deploy: airdrop # airdrop just to make sure we have enough to deploy
	# deploy sometimes fails on testnet with:
	#     Error: Custom: Invalid blockhash
	# in which case the solution seems to be to wait and retry.
	solana program deploy $(PROGRAM_BIN) --url $(NET)

.PHONY: airdrop
airdrop:
	solana airdrop $(SOL) --url $(NET)

.PHONY: run
run:
	./$(CLIENT_BIN) \
	    $(PROGRAM_KEYPAIR) \
	    $(CLUSTER_URL)

.PHONY: logs
logs:
	solana logs --url $(NET) | grep -i $(shell solana-keygen pubkey $(PROGRAM_KEYPAIR))
