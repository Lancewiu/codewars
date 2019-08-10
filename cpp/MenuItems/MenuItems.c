#define _CRT_SECURE_NO_WARNINGS
#include "Windows.h"

#include <string.h>
#include <stdint.h>
#include <stdlib.h>
#include <errno.h>
#include <stdio.h>

#define NUM_MENU_ITEMS 8
#define MAX_MENU_ITEM_LENGTH 10

const char* const MENU_ITEMS_UPPER[] = {
  "Burger",
  "Fries",
  "Chicken",
  "Pizza",
  "Sandwich",
  "Onionrings",
  "Milkshake",
  "Coke"
};

const char* const MENU_ITEMS_LOWER[] = {
  "burger",
  "fries",
  "chicken",
  "pizza",
  "sandwich",
  "onionrings",
  "milkshake",
  "coke"
};

const size_t MENU_ITEMS_LENGTHS[NUM_MENU_ITEMS] = {
  6,
  5,
  7,
  5,
  8,
  10,
  9,
  4
};

int build_hist(const char* input, uint32_t* hist)
{
	if (!input || !hist)
		return EINVAL;

	for (size_t i = 0; i < NUM_MENU_ITEMS; i++) {
		char* cursor = input;
		while (cursor = strstr(cursor, MENU_ITEMS_LOWER[i])) {
			hist[i]++;
			cursor++;
		}
	}
	return 0;
}

int format_orders(const uint32_t* hist, char* orders)
{
	if (!hist || !orders)
		return EINVAL;

	char* writer = orders;
	for (size_t i = 0; i < NUM_MENU_ITEMS; i++) {
		const size_t item_sz = sizeof(char) * MENU_ITEMS_LENGTHS[i];
		for (size_t rep = 0; rep < hist[i]; rep++) {
			strncpy(writer, MENU_ITEMS_UPPER[i], item_sz);
			writer += MENU_ITEMS_LENGTHS[i];
			*writer = ' ';
			writer++;
		}
	}

	if (writer != orders)
		* (writer - 1) = '\0';

	return 0;
}

char* get_order(const char* input)
{
	if (!input)
		return NULL;

	uint32_t* hist = (uint32_t*)calloc(NUM_MENU_ITEMS, sizeof(uint32_t));
	if (!hist || 0 != build_hist(input, hist))
		return NULL;

	const size_t upper_bound = 2 * strlen(input);
	char* orders = (char*)malloc(sizeof(char) * upper_bound);
	if (!orders || 0 != format_orders(hist, orders))
		return NULL;

	free(hist);
	hist = NULL;
	return orders;
}

int main(void)
{
	OutputDebugStringA(get_order("milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza"));
}