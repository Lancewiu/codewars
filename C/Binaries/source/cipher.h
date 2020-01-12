#ifndef CIPHER_H
#define CIPHER_H

#include <stddef.h> // size_t

enum CipherError {
        NO_ERROR                      =  0,
        ERR_INVALID_CHAR              = -1,
        ERR_IMPOSSIBLE_DIGIT_SEQUENCE = -2,
        ERR_NULL_ENCOUNTERED          = -3
};

/* 
 * Since we operate on digits, the maximum size of
 * any allocated string must be 8 times per character
 * That is, in the event of "9999988888".
 */
enum CipherError code(
                size_t digit_len,
                char const digits[restrict const digit_len],
                size_t *restrict const output_len,
                char output[restrict const 2 * digit_len]);

/*
 * The inverse of the above is true but for cases of 
 * "00000111111" in which the largest buffer you will need is 
 * at least half that of the encoded value.
 */
enum CipherError decode(
                size_t encoded_len,
                char const encoded[restrict const encoded_len],
                size_t *restrict const output_len,
                char output[restrict encoded_len / 4]);

#endif
