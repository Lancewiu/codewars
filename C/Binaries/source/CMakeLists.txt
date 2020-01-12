add_library(Cipher STATIC cipher.c)
target_include_directories(Cipher PRIVATE ${CMAKE_CURRENT_LIST_DIR})
set_property(TARGET Cipher PROPERTY C_STANDARD 99)

add_executable(Binaries main.c)
target_link_libraries(Binaries PRIVATE Cipher)
set_property(TARGET Binaries PROPERTY C_STANDARD 99)
