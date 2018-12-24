#include <stdint.h>

static const uint32_t RGB_NETWORK_MAINNET = 0;
static const uint32_t RGB_NETWORK_TESTNET = 1;
static const uint32_t RGB_NETWORK_REGTEST = 2;

static const uint32_t RGB_NEEDED_TX_TXID = 0;
static const uint32_t RGB_NEEDED_TX_SPENDS_OUTPOINT = 1;

struct rgb_sha256d {
    uint8_t val[32];
};

struct rgb_bitcoin_outpoint {
    struct rgb_sha256d txid;
    uint32_t vout;
};

struct rgb_needed_tx {
    uint32_t type;
    union {
        struct {
            struct rgb_sha256d txid;
            uint32_t dummy; // This makes both elements of the union the same length. It's easier to iterate over them.
        };
        struct rgb_bitcoin_outpoint outpoint;
    };
};

struct rgb_output_entry {
    struct rgb_sha256d asset_id;
    uint32_t amount;
    uint32_t vout;
};

// Contracts

struct rgb_contract {
    char title[256];
    struct rgb_bitcoin_outpoint issuance_utxo;
    struct rgb_bitcoin_outpoint initial_owner_utxo;
    uint32_t network;
    uint32_t total_supply;
};

void rgb_contract_get_asset_id(const struct rgb_contract *contract, struct rgb_sha256d *hash_buffer);

uint32_t rgb_contract_get_needed_txs(const struct rgb_contract *contract, struct rgb_needed_tx **needed_txs);

uint32_t rgb_contract_get_expected_script(const struct rgb_contract *contract, uint8_t **buffer);

uint32_t rgb_contract_serialize(const struct rgb_contract *contract, uint8_t **buffer);

void rgb_contract_deserialize(const uint8_t *buffer, uint32_t len, struct rgb_contract *contract);

// Debug functions

void rgb_debug_sha256d(const struct rgb_sha256d *hash);

void rgb_debug_print_contract(const struct rgb_contract *contract);

void rgb_debug_print_needed_tx(const struct rgb_needed_tx *e);