cmake_minimum_required(VERSION 3.0)
project(test-gerlib)

set(GERLIB_TEST ${PROJECT_NAME})

#########################################################################################
# Testing
#########################################################################################
add_executable(${GERLIB_TEST})
target_sources(${GERLIB_TEST}
  PRIVATE
  dummy.cc
)
target_link_libraries(${GERLIB_TEST}
  PRIVATE
  ${GERLIB_LIBRARY}
)
set_target_properties(${GERLIB_TEST} PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
