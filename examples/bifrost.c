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
	    .vout = 0
    };

    const char proof_hex[] = "01deadbeefff0000000000000000000000000000000000000000000000000000000300000000011edb367299711fcb52ca19f3c6ef92996466ae7413fb9db0e36c02388aa9260ee8030000010000000001084d79205469746c65deadbeefff00000000000000000000000000000000000000000000000000000003000000deadbeefff000000000000000000000000000000000000000000000000000000030000000b110907e8030000";
    uint8_t *proof_bytes = (uint8_t *) malloc(sizeof(proof_hex) / 2);
    decode_hex(proof_hex, proof_bytes, sizeof(proof_hex) / 2);

    const char server[] = "localhost:3000";

    struct rgb_proof *proof = rgb_proof_deserialize(proof_bytes, sizeof(proof_hex) / 2);

    uint8_t result = rgb_bifrost_upload_proofs(server, proof, utxo.txid);
    printf("Upload result: %d\n", result);

    struct rgb_allocated_array_rgb_proof downloaded = rgb_bifrost_get_proofs_for(server, &utxo);
    printf("Downloaded %ld proofs:\n", downloaded.size);

    for (size_t i = 0; i < downloaded.size; i++) {
	rgb_debug_print_proof(&downloaded.ptr[0]);
    }

    return EXIT_SUCCESS;
}