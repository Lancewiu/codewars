#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "cipher.h"

void assert_encode(
                char const str[const],
                char const expected[const]);

void assert_decode(
                char const encoded[const],
                char const expected[const]);

void assert_encode(
                char const str[const],
                char const expected[const])
{
        // see code() in strcode.h for explanation on expected size
        printf("String:  \"%s\"\r\n", str);

        size_t str_len = strlen(str);

        size_t encoded_len;
        char *encoded = (char *)malloc(sizeof(char) * (8 * (str_len - 1) + 1));
        enum CipherError err_code = code(
                        str_len,
                        str,
                        &encoded_len,
                        encoded);
        if (NO_ERROR != err_code) {
                printf("ERROR: %d\r\n", err_code);
                goto Finally;
        }

        printf("Encoded: \"%s\"", encoded);

        size_t expected_len = strlen(expected);
        assert(expected_len == encoded_len);
        assert(0 == strncmp(encoded, expected, encoded_len));
        printf(" -- PASS\r\n");

Finally:
        free(encoded);
        encoded = NULL;
}

void assert_decode(
                char const encoded[const],
                char const expected[const])
{
        printf("Encoded: \"%s\"\r\n", encoded);

        size_t encoded_len = strlen(encoded);

        size_t decoded_len;
        char *decoded = (char *)malloc(sizeof(char) * encoded_len);
        enum CipherError err_code = decode(
                        encoded_len,
                        encoded,
                        &decoded_len,
                        decoded);
        if (NO_ERROR != err_code) {
                printf("ERROR: %d\r\n", err_code);
                goto Finally;
        }

        printf("Decoded: \"%s\"", decoded);
        assert(strlen(expected) == decoded_len);
        assert(0 == strncmp(decoded, expected, decoded_len));
        printf(" -- PASS\r\n");

Finally:
        free(decoded);
        decoded = NULL;
}

int main(void)
{
        assert_encode("77338855", "001111001111011101110001100000011000001101001101");
        assert_encode("77338", "0011110011110111011100011000");
        assert_encode("0011121314", "1010111111011011011111001100");
        assert_encode("62", "0011100110");
        assert_encode("55337700", "001101001101011101110011110011111010");
        assert_encode("1119441933000055", "1111110001100100110000110011000110010111011110101010001101001101");
        assert_encode("69", "00111000011001");
        assert_encode("86", "00011000001110");
        //assert_encode("07", "10001111");

        assert_decode("001111001111011101110001100000011000001101001101", "77338855");
        assert_decode("0011110011110111011100011000", "77338");
        assert_decode("1010111111011011011111001100", "0011121314");
        assert_decode("10001111", "07");
        assert_decode("001100001100001100001110001110001110011101110111001110001110001110001111001111001111001100001100001100", "444666333666777444");
        assert_decode("01110111110001100100011000000110000011110011110111011100110000110001100110", "33198877334422");
        assert_decode("0011010011010011011010101111110011000011000011000011100011100011100011100011100011100001100100011001000110011100011001001111001111001111001111001111001111", "55500011144466666699919777777");
        assert_decode("01110111011111000110010011110011110011110011110011110011110111011101110110011001100110011001101111111010101100011001000110000001100000011000", "3331977777733322222211100019888");
        //assert_decode("10001111", "07");

        return EXIT_SUCCESS;
}
