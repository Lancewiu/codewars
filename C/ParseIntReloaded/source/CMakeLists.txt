function(add_module target source)
  add_library(${target} STATIC ${source})
  target_include_directories(${target} PRIVATE ${CMAKE_CURRENT_LIST_DIR})
  set_property(TARGET ${target} PROPERTY C_STANDARD 11)
endfunction()

add_module(Parse parse.c)

add_executable(${CMAKE_PROJECT_NAME} main.c)
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE Parse)
set_property(TARGET ${CMAKE_PROJECT_NAME} PROPERTY C_STANDARD 11)
