cmake_minimum_required(VERSION 3.0)

#########################################################################################
# Library
#########################################################################################
add_library(${GER_LIBRARY} STATIC
  ger.cc
)

target_link_libraries(${GER_LIBRARY}
  ${LIBSSH_LIBRARIES}
  ${FMT_LIBRARIES}
  absl_strings
)

add_dependencies(${GER_LIBRARY}
  ${GSL}
  ${FMT}
  ${ABSEIL}
)

#########################################################################################
# Executable
#########################################################################################
add_executable(${GER_BINARY}
  main.cc
)

target_link_libraries(${GER_BINARY}
  ${GER_LIBRARY}
)

add_dependencies(${GER_BINARY}
  ${GER_LIBRARY}
)