cmake_minimum_required(VERSION 3.0)
project(ger-cli LANGUAGES CXX)

#########################################################################################
# Project
#########################################################################################
include_directories(${GERCLI_INCLUDE_DIR})
include_directories(${GERLIB_INCLUDE_DIR})
add_subdirectory(src)

#########################################################################################
# Testing
#########################################################################################
if(BUILD_TESTING)
  add_subdirectory(test)
endif()