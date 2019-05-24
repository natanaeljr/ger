cmake_minimum_required(VERSION 3.0)

#########################################################################################
# Library
#########################################################################################
add_library(${GER_LIBRARY} STATIC
  cli/ger_cli.cc
  cli/change_cmd.cc
)

set_target_properties(${GER_LIBRARY} PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
  LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

#########################################################################################
# Executable
#########################################################################################
add_executable(${GER_BINARY}
  cli/main.cc
)

target_link_libraries(${GER_BINARY}
  ${GER_LIBRARY}
)

add_dependencies(${GER_BINARY}
  ${GER_LIBRARY}
)

set_target_properties(${GER_BINARY} PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})