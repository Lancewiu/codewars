cmake_minimum_required(VERSION 3.16.2)

project(IsMyFriendCheating C)

if (MSVC) # Enable warnings
  add_compile_options(/W4)
elseif(CMAKE_C_COMPILER_ID STREQUAL "GNU")
  add_compile_options(-Wall -Wextra -Wpedantic -Wstrict-prototypes -Wmissing-prototypes -Wformat=2)
endif()

add_subdirectory(source)
