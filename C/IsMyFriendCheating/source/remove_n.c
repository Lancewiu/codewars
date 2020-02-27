#include "remove_n.h"
#include "pair.h"

#include <stdlib.h>
#include <stdint.h>

long long sum_to(long long upper_bound);

long long sum_to(long long upper_bound)
{
        uintmax_t ub_large = (uintmax_t)upper_bound;
        // (n * (n + 1)) / 2
        return (long long)((ub_large * (ub_large + 1)) / 2);
}

Pair **removNb(long long n, int *length)
{
        *length = 0;

        if (n < 3)
                return NULL;

        long long sum_n = sum_to(n);
        
        PairList pairlist;
        init_pairlist(2*n, &pairlist);
        if (NULL == pairlist.buf)
                return NULL;

        /*
         * skip the first case as it's not possible that 1 * b is
         * larger than (sum(1:b) - b - 1)
         */
        long long j;
        for (long long i = 2; i < n; ++i) {
                if (0 != (sum_n - i) % (i + 1))
                        continue;
                // b = (sum_n - a) / (a + 1) where a * b == (sum_n - a - b)
                j = (sum_n - i) / (i + 1);
                if (j > n)
                        continue;
                Pair *pair = malloc(sizeof(Pair));
                if (NULL == pair)
                        return NULL;
                pair->first = i;
                pair->snd = j;
                pairlist_add_pair(&pairlist, pair);
        }
        
        (*length) = (int)pairlist.length;
        return pairlist.buf;
}
