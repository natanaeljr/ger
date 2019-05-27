cmake_minimum_required(VERSION 3.0)

#########################################################################################
# Testing
#########################################################################################
add_executable(${GER_TEST}
  dummy.cc
  # command.cc
)

target_link_libraries(${GER_TEST}
  ${GER_LIBRARY}
)

add_dependencies(${GER_TEST}
  ${GER_LIBRARY}
)

set_target_properties(${GER_TEST} PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})