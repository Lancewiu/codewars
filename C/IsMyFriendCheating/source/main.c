#include <stddef.h>
#include <stdbool.h>
#include <assert.h>

#include "pair.h"
#include "remove_n.h"

void test(long long n, size_t n_results, long long result[const n_results]);
bool pair_contains(Pair *pair, long long element);

bool pair_contains(Pair *pair, long long element)
{
        if (NULL == pair)
                return false;
        return pair->first == element || pair->snd == element;
}

void test(long long n, size_t n_results, long long results[const n_results])
{
        int pairs_len;
        Pair **pairs = removNb(n, &pairs_len);
        assert((size_t)pairs_len == n_results);
        for (int i = 0; i < pairs_len; ++i)
                for (size_t j = 0; j < n_results; ++j)
                        assert(pair_contains(pairs[i], results[j]));
}

int main(void)
{
#define LL_ARRAY_SIZE(long_array) sizeof(long_array) / sizeof(long long)
        {
                long long results[] = {15, 21};
                test(26, LL_ARRAY_SIZE(results), results);
        }

        test(100, 0, NULL);

        {
                long long results[] = {21, 31};
                test(37, LL_ARRAY_SIZE(results), results);
        }

        {
                long long results[] = {55, 91};
                test(101, LL_ARRAY_SIZE(results), results);
        }
#undef LL_ARRAY_SIZE
        return EXIT_SUCCESS;
}
