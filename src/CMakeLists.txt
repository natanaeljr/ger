cmake_minimum_required(VERSION 3.0)

#########################################################################################
# Libraries
#########################################################################################
#add_library(${GERRITC_LIBRARY} STATIC
#)
#
#add_dependencies(${GERRITC_LIBRARY}
#)
#
#target_link_libraries(${GERRITC_LIBRARY}
#)

#########################################################################################
# Executable
#########################################################################################
add_executable(${GERRITC_BINARY} main.cc)
#add_dependencies(${GERRITC_BINARY} ${GERRITC_LIBRARY})
#target_link_libraries(${GERRITC_BINARY} ${GERRITC_LIBRARY})
set_target_properties(${GERRITC_BINARY} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
