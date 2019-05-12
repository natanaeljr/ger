cmake_minimum_required(VERSION 3.0)

#########################################################################################
# Testing
#########################################################################################
add_executable(${GER_TEST}
  command.cc
)

target_link_libraries(${GER_TEST}
  ${GER_LIBRARY}
  ${GTEST_MAIN_LIBRARIES}
  ${GTEST_LIBRARIES}
  pthread
)

add_dependencies(${GER_TEST}
  ${GER_LIBRARY}
  ${GTEST}
)
