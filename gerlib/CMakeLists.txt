cmake_minimum_required(VERSION 3.0)
project(gerlib LANGUAGES CXX)

#########################################################################################
# Cap'n Proto
#########################################################################################
add_subdirectory(capnp)

#########################################################################################
# Library
#########################################################################################
add_library(${GERLIB_LIBRARY})
target_include_directories(${GERLIB_LIBRARY} PUBLIC include)
target_link_libraries(${GERLIB_LIBRARY}
  PUBLIC
  ${GERCAPNP_LIBRARY}
)
set_target_properties(${GERLIB_LIBRARY} PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib
  LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

#########################################################################################
# Sources
#########################################################################################
add_subdirectory(src)

#########################################################################################
# Testing
#########################################################################################
if(BUILD_TESTING)
  add_subdirectory(test)
endif()