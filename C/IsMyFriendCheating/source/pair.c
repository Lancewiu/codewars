#include "pair.h"

#include <stdlib.h>

void init_pairlist(size_t max_len, PairList *output)
{
        output->buf = malloc(sizeof(Pair *) * max_len);
        output->length = 0;
        output->max_len = max_len;
}

int pairlist_add_pair(PairList *self, Pair *p)
{
        if (NULL == self || NULL == p || self->length > self->max_len)
                return -1;
        self->buf[self->length++] = p;
        return 0;
}
