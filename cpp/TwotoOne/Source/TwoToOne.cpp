#include "TwoToOne.h"
#include <iterator>
#include <set>

std::string TwoToOne::longest(const std::string& s1, const std::string& s2)
{
	std::set<char> chars(s1.cbegin(), s1.cend());
	chars.insert(s2.cbegin(), s2.cend());
	return std::string(chars.cbegin(), chars.cend());
}
