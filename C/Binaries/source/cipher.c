#include "cipher.h"

#include <assert.h>
#include <stdbool.h>
#include <stdint.h>

void encode_digit(
                uint_fast8_t digit,
                size_t *restrict const output_len,
                char output[restrict static 2]);

enum CipherError decode_n_bits(
                char const input[restrict const static 2],
                uint_fast8_t *restrict const n_bits);

uint_fast8_t decode_digit(
                uint_fast8_t n_bits,
                char const encoded[restrict const n_bits]);

void encode_digit(
                uint_fast8_t digit,
                size_t *restrict const output_len,
                char output[restrict static 2])
{
        // digits larger than 4 bit width is clamped to 4;
        uint_fast8_t n_bits = 4;
        for (size_t i = 0; i < 4; ++i) {
                if (2 > digit >> i) {
                        n_bits = i + 1;
                        break;
                }
        }

        *output_len = (size_t)(2 * n_bits);

        for (uint_fast8_t i = 0; i < n_bits - 1; ++i)
                output[i] = '0';
        output[n_bits - 1] = '1';

        output += n_bits;
        size_t shift_offset;
        uint_fast8_t masked;
        for (uint_fast8_t i = 0; i < n_bits; ++i) {
                shift_offset = n_bits - i - 1;
                masked = 1 & (digit >> shift_offset);
                assert(masked == 0 || masked == 1);
                output[i] = '0' + (char)((digit >> shift_offset) & 1);
        }
}

enum CipherError code(
                size_t digit_len,
                char const digits[restrict const digit_len],
                size_t *restrict const output_len,
                char output[restrict const 2 * digit_len])
{
        if (NULL == digits || NULL == output || NULL == output_len)
                return ERR_NULL_ENCOUNTERED;

        size_t offset = 0;
        size_t length;
        char digit;
        for (size_t i = 0; i < digit_len; ++i) {
                digit = digits[i];
                if (digit < '0')
                        return ERR_INVALID_CHAR;
                encode_digit(
                                (uint_fast8_t)(digit - '0'),
                                &length,
                                output + offset);
                offset += length;
        }

        output[offset] = '\0';
        (*output_len) = offset;

        return NO_ERROR;
}

enum CipherError decode_n_bits(
                char const input[restrict const static 2],
                uint_fast8_t *restrict const n_bits)
{
        // 4 is the maximum valid encoded digit size (the value of 8 and 9).
        for (size_t i = 0; i < 4; ++i) {
                if (input[i] == '1') {
                        *n_bits = (uint_fast8_t)(i + 1);
                        return NO_ERROR;
                }
        }
        return ERR_IMPOSSIBLE_DIGIT_SEQUENCE;
}

uint_fast8_t decode_digit(
                uint_fast8_t n_bits,
                char const encoded[restrict const n_bits])
{
        uint_fast8_t decoded = 0;
        for (size_t i = 0; i < n_bits; ++i)
                if (encoded[i] == '1')
                        decoded |= 1 << (n_bits - i - 1);

        assert(decoded <= 9);
        return decoded;
}

enum CipherError decode(
                size_t encoded_len,
                char const encoded[restrict const encoded_len],
                size_t *restrict const output_len,
                char output[restrict encoded_len / 4])
{
        if (NULL == encoded || NULL == output_len || NULL == output)
                return ERR_NULL_ENCOUNTERED;

        uint_fast8_t n_bits;
        size_t r_offset = 0;
        size_t w_offset = 0;
        while (r_offset < encoded_len) {
                enum CipherError code = decode_n_bits(
                                encoded + r_offset,
                                &n_bits);
                if (NO_ERROR != code)
                        return code;
                r_offset += n_bits;

                char decoded = (char)decode_digit(n_bits, encoded + r_offset);
                r_offset += n_bits;

                if (decoded > '9')
                        return ERR_INVALID_CHAR;
                output[w_offset++] = '0' + decoded;
        }
        output[w_offset] = '\0';
        *output_len = w_offset;
        return NO_ERROR;
}
