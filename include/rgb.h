#ifndef RGBORG_RGB_H
#define RGBORG_RGB_H

#include <stdint.h>

static const uint32_t RGB_NETWORK_MAINNET = 0;
static const uint32_t RGB_NETWORK_TESTNET = 1;
static const uint32_t RGB_NETWORK_REGTEST = 2;

static const uint32_t RGB_NEEDED_TX_TXID = 0;
static const uint32_t RGB_NEEDED_TX_SPENDS_OUTPOINT = 1;

// Use this as a vout in an OutputEntry to burn that output
static const uint32_t RGB_BURN_ASSET = 0xFFFFFFFF;

struct rgb_sha256d {
    uint8_t val[32];
};

struct rgb_needed_tx_map {
};

#define rgb_as_ptr(x) (x).ptr

#define _declare_rgb_allocated_box_internal(type, optional_struct)        \
        struct rgb_allocated_ ## type {                                \
            optional_struct type *ptr;                                        \
            union {                                                        \
                size_t len;                                                \
                size_t size;                                                \
                size_t num_elements;                                        \
            };                                                                \
        };

#define declare_rgb_allocated_box_struct(type) _declare_rgb_allocated_box_internal(type, struct)
#define declare_rgb_allocated_box_native(type) _declare_rgb_allocated_box_internal(type,)

// Declare all the possible boxes being returned
declare_rgb_allocated_box_struct(rgb_contract)
declare_rgb_allocated_box_struct(rgb_proof)
declare_rgb_allocated_box_struct(rgb_needed_tx)
declare_rgb_allocated_box_struct(rgb_needed_tx_map)
declare_rgb_allocated_box_struct(rgb_sha256d)
declare_rgb_allocated_box_native(uint8_t)

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

struct rgb_bitcoin_serialized_tx {
    uint32_t size;
    uint8_t *payload;
};

// Contracts

struct rgb_contract {
    char title[256];
    struct rgb_bitcoin_outpoint issuance_utxo;
    struct rgb_bitcoin_outpoint initial_owner_utxo;
    uint32_t network;
    uint32_t total_supply;
};

struct rgb_allocated_rgb_sha256d rgb_contract_get_asset_id(const struct rgb_contract *contract);

struct rgb_allocated_rgb_needed_tx rgb_contract_get_needed_txs(const struct rgb_contract *contract);

struct rgb_allocated_uint8_t rgb_contract_get_expected_script(const struct rgb_contract *contract);

struct rgb_allocated_uint8_t rgb_contract_serialize(const struct rgb_contract *contract);

struct rgb_allocated_rgb_contract rgb_contract_deserialize(const uint8_t *buffer, uint32_t len);

uint8_t rgb_contract_verify(const struct rgb_contract *contract, const struct rgb_needed_tx_map *map);

struct rgb_allocated_rgb_needed_tx_map rgb_init_needed_tx_map();

void rgb_push_needed_tx_map(struct rgb_needed_tx_map *map, const struct rgb_needed_tx *key,
                            const struct rgb_bitcoin_serialized_tx *val);

// Proofs

struct rgb_proof {
    uint32_t bind_to_count;
    struct rgb_bitcoin_outpoint *bind_to;
    uint32_t input_count;
    struct rgb_proof *input;
    uint32_t output_count;
    struct rgb_output_entry *output;
    struct rgb_contract *contract;
};

uint8_t rgb_proof_is_root_proof(const struct rgb_proof *proof);

struct rgb_allocated_rgb_needed_tx rgb_proof_get_needed_txs(const struct rgb_proof *proof);

struct rgb_allocated_uint8_t rgb_proof_get_expected_script(const struct rgb_proof *proof);

struct rgb_allocated_uint8_t rgb_proof_serialize(const struct rgb_proof *proof);

struct rgb_allocated_rgb_proof rgb_proof_deserialize(const uint8_t *buffer, uint32_t len);

uint8_t rgb_proof_verify(const struct rgb_proof *proof, const struct rgb_needed_tx_map *map);

struct rgb_allocated_rgb_contract
rgb_proof_get_contract_for(const struct rgb_proof *proof, const struct rgb_sha256d *asset_id); // TODO!

// Debug functions

void rgb_debug_sha256d(const struct rgb_sha256d *hash);

void rgb_debug_print_contract(const struct rgb_contract *contract);

void rgb_debug_print_needed_tx(const struct rgb_needed_tx *e);

void rgb_debug_print_serialized_tx(const struct rgb_bitcoin_serialized_tx *tx);

void rgb_debug_print_needed_tx_map(const struct rgb_needed_tx_map *map);

void rgb_debug_print_proof(const struct rgb_proof *proof);

#endif
