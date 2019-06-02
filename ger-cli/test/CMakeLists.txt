cmake_minimum_required(VERSION 3.0)
project(test-gercli)

set(GERCLI_TEST ${PROJECT_NAME})

#########################################################################################
# Testing
#########################################################################################
add_executable(${GERCLI_TEST}
  dummy.cc
)

target_link_libraries(${GERCLI_TEST}
  ${GERCLI_LIBRARY}
)

add_dependencies(${GERCLI_TEST}
  ${GERCLI_LIBRARY}
)

set_target_properties(${GERCLI_TEST} PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
