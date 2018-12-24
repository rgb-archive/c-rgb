#include <stdlib.h>

#include <rgb.h>

int main() {
    struct rgb_bitcoin_outpoint random_outpoint = {
            .txid = {{0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F}},
            .vout = 42
    };

    const struct rgb_needed_tx enum_txid = {
            .type = RGB_NEEDED_TX_TXID,
            .txid = random_outpoint.txid
    };

    const struct rgb_needed_tx enum_outpoint = {
            .type = RGB_NEEDED_TX_SPENDS_OUTPOINT,
            .outpoint = random_outpoint
    };

    rgb_debug_print_needed_tx(&enum_txid);
    rgb_debug_print_needed_tx(&enum_outpoint);

    return EXIT_SUCCESS;
}