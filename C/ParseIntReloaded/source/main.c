#include <stddef.h>
#include <assert.h>

#include "parse.h"

void test(long expected, char const *const number);

void test(char const *const number, long expected)
{
        assert(expected == parse_int(number));
}

int main(void)
{
        test("one", 1);
        test("twenty", 20);
        test("two hundred and forty-six", 246);
        test("seven hundred eighty-three thousand nine hundred and nineteen", 783919);
        test("one million", 1000000);
        return EXIT_SUCCESS;
}
