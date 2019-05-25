#include "pch.h"
#include "CppUnitTest.h"
#include "TwoToOne.h"

#include <string>

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

void dotest(std::string a1, std::string a2, std::string expected)
{
	Assert::AreEqual(TwoToOne::longest(a1, a2), expected);
}

namespace TwoToOneUnitTests
{
	TEST_CLASS(TwoToOneUnitTests)
	{
	public:

		TEST_METHOD(Provided)
		{
			dotest("aretheyhere", "yestheyarehere", "aehrsty");
			dotest("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
		}

		TEST_METHOD(Examples)
		{
			dotest("xyaabbbccccdefww", "xxxxyyyyabklmopq", "abcdefklmopqwxy");
			dotest("abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz");
		}
	};
}
