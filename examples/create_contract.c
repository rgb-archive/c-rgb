#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#include <rgb.h>

void print_hex(const void *p, size_t len) {
    for (size_t i = 0; i < len; ++i)
        printf("%02hhx", *((uint8_t *) p + i));

    printf("\n");
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

    rgb_debug_print_contract(&contract);

    struct rgb_sha256d asset_id;

    rgb_contract_get_asset_id(&contract, &asset_id);

    printf("From Rust: ");
    fflush(stdout); // without this the order of prints would likely be messed up
    rgb_debug_sha256d(&asset_id);

    printf("From C:    ");
    print_hex((const void *) &asset_id, 32);

    printf("\n");

    // Let's ask Rust for the commitment script
    uint8_t *script_buffer = NULL;
    uint32_t script_len = rgb_contract_get_expected_script(&contract, &script_buffer);

    printf("Expected commitment output: ");
    print_hex((const void *) script_buffer, script_len);

    printf("\n");

    // And now we serialize it
    uint8_t *serialized_buffer = NULL;
    uint32_t serialized_len = rgb_contract_serialize(&contract, &serialized_buffer);

    printf("Serialized contract: ");
    print_hex((const void *) serialized_buffer, serialized_len);

    printf("\n");

    // And deserialize it
    struct rgb_contract deserialized_contract;
    rgb_contract_deserialize(serialized_buffer, serialized_len, &deserialized_contract);

    rgb_debug_print_contract(&deserialized_contract);

    return EXIT_SUCCESS;
}