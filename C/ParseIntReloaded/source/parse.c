#include "parse.h"

#define __STDC_WANT_LIB_EXT1__ 1
#include <string.h> //strtok_s, strnlen_s
#include <stdbool.h> //bool

static struct SizedNames {
        char const **name;
        size_t *length;
        size_t num_names;
};

static struct SizeSpec {
        struct SizedNames names;
        long *multiplier;
};

static struct ValueSpec {
        struct SizedNames names;
        long *value;
};

static struct SizeSpec SIZE_SPECIFIER_TOKEN = {
        .names = {
                .name = {
                        "million",
                        "thousand",
                        "hundred"
                },
                .length = {
                        7,
                        8,
                        7
                },
                .num_names = 3
        },
#define POW_TEN_LONG(exp) (long)pow(10, exp)
        .multiplier = {
                POW_TEN_LONG(6),
                POW_TEN_LONG(3),
                POW_TEN_LONG(2)
        }
#undef POW_TEN_LONG
};

static struct ValueSpec TENS_DIGIT_TOKEN = {
        .names = {
                .name = {
                        "twenty",
                        "thirty",
                        "forty",
                        "fifty",
                        "sixty",
                        "seventy",
                        "eighty",
                        "ninety"
                },
                .length = {
                        6,
                        6,
                        5,
                        5,
                        5,
                        7,
                        6,
                        6
                },
                .num_names = 8
        },
        .value = {
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9
        },
};

static struct ValueSpec TEENS_TOKEN = {
        .names = {
                .name = {
                        "ten",
                        "eleven",
                        "twelve",
                        "thirteen",
                        "fourteen",
                        "fifteen",
                        "sixteen",
                        "seventeen",
                        "eighteen",
                        "nineteen"
                },
                .length = {
                        3,
                        6,
                        6,
                        8,
                        8,
                        7,
                        7,
                        9,
                        8,
                        8
                },
                num_names = 10
        },
        .value = {
                10,
                11,
                12,
                13,
                14,
                15,
                16,
                17,
                18,
                19
        }
};

static struct ValueSpec ONES_TOKEN = {
        .names = {
                .name = {
                        "one",
                        "two",
                        "three",
                        "four",
                        "five",
                        "six",
                        "seven",
                        "eight",
                        "nine",
                },
                .length = {
                        3,
                        3,
                        5,
                        4,
                        4,
                        3,
                        5,
                        5,
                        4
                },
                .num_names = 9
        },
        .value = {
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9
        }
};

static bool sizednames_contains(
                SizedNames *names,
                size_t str_len,
                const char str[const str_len]);

static long process_token(long value, const char *const token);

static long process_size_spec(
                long value,
                SizeSpec const *const spec,
                size_t index);

static long process_value(
                long value,
                ValueSpec *spec,
                size_t index);

static bool sizednames_contains(
                SizedNames *names,
                size_t str_len,
                const char str[const str_len])
{
        for (size_t i = 0; i < list_len; ++i)
                if strncmp
        return true;
}

static long process_token(long value, const char *token);
{
        // zero can only be used as a single value.
        if (0 == strncmp("zero", token, 4))
                return 0;

        if (0 == strncmp())

        return value;
}

long parse_int(const char *number)
{
        rsize_t strmax;
        char *tok_state;
        char *token = strtok_s(number, &strmax, " ", &tok_state);
        long result = 0;
        while (NULL != token) {
                result = process_token(result, token);
                token = strtok_s(NULL, &strmax, " ", &tok_state);
        }
        return result;
}
