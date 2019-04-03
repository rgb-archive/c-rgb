#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#include <rgb.h>

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
    struct rgb_bitcoin_outpoint utxo = {
	    .txid = {{0xDE, 0xAD, 0xBE, 0xEF, 0xFF}},
	    .vout = 3
    };

    struct rgb_bitcoin_address myaddr = {
	    .str = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
    };

    const char proof_hex[] = "01deadbeefff0000000000000000000000000000000000000000000000000000000300000000011edb367299711fcb52ca19f3c6ef92996466ae7413fb9db0e36c02388aa9260ee8030000010000000001084d79205469746c65deadbeefff00000000000000000000000000000000000000000000000000000003000000deadbeefff000000000000000000000000000000000000000000000000000000030000000b110907e8030000";
    uint8_t *proof_bytes = (uint8_t *) malloc(sizeof(proof_hex) / 2);
    decode_hex(proof_hex, proof_bytes, sizeof(proof_hex) / 2);

    struct rgb_proof *input_proof = rgb_proof_deserialize(proof_bytes, sizeof(proof_hex) / 2);
    rgb_debug_print_proof(input_proof);

    struct rgb_kaleidoscope_outpoint op = {
	    .bitcoin_address = &myaddr,
	    .bitcoin_amount = 42,
	    .rgb_outputs = rgb_init_kaleidoscope_outpoint_map()
    };

    rgb_push_kaleidoscope_outpoint_map(op.rgb_outputs, input_proof->output[0].asset_id, 222);
    rgb_debug_print_kaleidoscope_outpoint(&op);

    struct rgb_proof_tx_pair spend_result = rgb_kaleidoscope_spend_proofs(1, input_proof, 1, &utxo, 1, &op);

    rgb_debug_print_proof(spend_result.proof);
    rgb_debug_print_serialized_tx(&spend_result.serialized_tx);

    return EXIT_SUCCESS;
}