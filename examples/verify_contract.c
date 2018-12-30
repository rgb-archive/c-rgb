#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#include <rgb.h>

void print_hex(const void *p, size_t len) {
    for (size_t i = 0; i < len; ++i)
        printf("%02hhx", *((uint8_t *) p + i));

    printf("\n");
}

static inline uint8_t char_value(char c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'f') return c - 'a' + 0x0A;
    return c - 'A' + 0x0A;
}

void decode_hex(const char *c, uint8_t *buffer, size_t len) {
    for (size_t i = 0; i < len; ++i) {
        buffer[i] = (char_value(c[i * 2]) << 4) | char_value(c[i * 2 + 1]);
    }
}

int main() {
    struct rgb_bitcoin_outpoint issuance_utxo = {
            .txid = {{0xDE, 0xAD, 0xBE, 0xEF, 0xFF}},
            .vout = 3
    };

    struct rgb_bitcoin_outpoint initial_owner_utxo = {
            .txid = {{0x00, 0x01, 0x02, 0x04, 0x08, 0xFF}},
            .vout = 0
    };

    struct rgb_contract contract = {
            .title = "My Title",
            .issuance_utxo = issuance_utxo,
            .initial_owner_utxo = initial_owner_utxo,
            .network = RGB_NETWORK_TESTNET,
            .total_supply = 1000
    };

    // Random transaction found on the blockchain
    const char hex[] = "020000000001028016db51a8e063e2f3152bc8c5a6286710ecf06dca7be1b2fbf733223cbf2d5d0100000017160014f8ec378a30323d79d70ca49ff2f22a2b2dff89d7feffffff9ef6f1c014fcef9ae13242a3db13bdd154e43ecf16ca1e04f8fe3463db1521670a00000017160014f9aefcdf133eb15d330ebfd4ee7122787d7e919cfeffffff02434d0f000000000017a91400c6d6ca1aa02e070289112d98a2abcc59974dbc87303c0700000000001976a914ee5db4fec017002d60ef88af13840b5943e61e6088ac02473044022049a3d638ce4bd3d699eaeee7e0180783140d1fe925a7fbcf3075473cf7dbf9e202203538760c6106137f8a1c0a89b1a93842d11cab674923a72356fb5cfe6662a7b9012102ee963d119f20da7696dd7fbfb3d89a0325380120d8d5c7dff21590dbe980f46a02483045022100ecff7995fc42b6ea9582ac2d9115f1da3c8b3f0dacccd288d9b716b4464c44c202202520e7efa66da38aeb9fcba0e92cfc61604310a036462b7e52697453877eb666012103ea2b0b712415acabc0b6245594e8e45b3eb267a17e6ebb6abe4da40372c9605bb37c0800";

    struct rgb_bitcoin_serialized_tx tx = {
            .size = (sizeof(hex) / 2),
            .payload = (uint8_t *) malloc(sizeof(hex) / 2)
    };

    const struct rgb_needed_tx need = {
            .type = RGB_NEEDED_TX_SPENDS_OUTPOINT,
            .outpoint = issuance_utxo
    };

    decode_hex(hex, tx.payload, sizeof(hex) / 2);

    struct rgb_needed_tx_map *map;

    rgb_init_needed_tx_map(&map);
    rgb_push_needed_tx_map(map, &need, &tx);

    // The check will fail with "invalid commitment"
    uint8_t result = rgb_contract_verify(&contract, map);

    printf("Verification result: %u\n", result);

    return EXIT_SUCCESS;
}