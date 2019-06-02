cmake_minimum_required(VERSION 3.0)

#########################################################################################
# Library
#########################################################################################
add_library(${GERLIB_LIBRARY} STATIC
  change.cc
)

set_target_properties(${GERLIB_LIBRARY} PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
  LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)