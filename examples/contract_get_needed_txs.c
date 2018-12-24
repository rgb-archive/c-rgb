#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#include <rgb.h>

void print_hex(const void *p, size_t len) {
    for (size_t i = 0; i < len; ++i)
        printf("%02hhx", *((uint8_t *) p + i));
}

int main() {
    struct rgb_bitcoin_outpoint issuance_utxo = {
            .txid = {{0xDE, 0xAD, 0xBE, 0xEF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF}},
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

    rgb_debug_print_contract(&contract);

    // TODO: needed_txs should be freed
    struct rgb_needed_tx *needed_txs = NULL;
    uint32_t len = rgb_contract_get_needed_txs(&contract, &needed_txs);

    printf("Number of elements: %u\n", len);

    for (size_t i = 0; i < len; ++i) {
        if (needed_txs[i].type == RGB_NEEDED_TX_TXID) {
            printf("FromTXID(\n\ttxid: ");
            print_hex((void *) &needed_txs[i].txid, 32);
            printf("\n)\n");

        } else if (needed_txs[i].type == RGB_NEEDED_TX_SPENDS_OUTPOINT) {
            printf("WhichSpendsOutPoint(\n\ttxid: ");
            print_hex((void *) &needed_txs[i].outpoint.txid, 32);
            printf("\n\tvout: %u\n)\n", needed_txs[i].outpoint.vout);

        } else {
            fprintf(stderr, "Unknown NeededTx type %u. Exiting now.\n", needed_txs->type);

            return EXIT_FAILURE;
        }
    }

    return EXIT_SUCCESS;
}