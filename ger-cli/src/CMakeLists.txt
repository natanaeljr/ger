cmake_minimum_required(VERSION 3.0)

#########################################################################################
# Library
#########################################################################################
add_library(${GERCLI_LIBRARY} STATIC
  change_cmd.cc
  gercli.cc
  config.cc
)

target_link_libraries(${GERCLI_LIBRARY}
  ${GERLIB_LIBRARY}
)

add_dependencies(${GERCLI_LIBRARY}
  ${GERLIB_LIBRARY}
)

set_target_properties(${GERCLI_LIBRARY} PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
  LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

#########################################################################################
# Executable
#########################################################################################
add_executable(${GERCLI_BINARY}
  main.cc
)

target_link_libraries(${GERCLI_BINARY}
  ${GERCLI_LIBRARY}
)

add_dependencies(${GERCLI_BINARY}
  ${GERCLI_LIBRARY}
)

set_target_properties(${GERCLI_BINARY} PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})