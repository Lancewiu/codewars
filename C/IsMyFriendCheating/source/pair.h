#ifndef PAIR_H
#define PAIR_H

#include <stdlib.h>

typedef struct Pair {
        long long first;
        long long snd;
} Pair;

typedef struct PairList {
        Pair **buf;
        size_t length;
        size_t max_len;
} PairList;

void init_pairlist(size_t max_len, PairList *output);
int pairlist_add_pair(PairList *self, Pair *p);

#endif
