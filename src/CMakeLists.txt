cmake_minimum_required(VERSION 3.0)

#########################################################################################
# Library
#########################################################################################
add_library(${GERRITC_LIBRARY} STATIC
    gerrit.cc
)

target_link_libraries(${GERRITC_LIBRARY}
    ${LIBNOTIFY_LIBRARIES}
)

set(GERRITC_LIBRARY_DEPS
    ${GSL_TARGET}
    ${ARGPARSE_TARGET}
)

if(GERRITC_LIBRARY_DEPS)
add_dependencies(${GERRITC_LIBRARY} ${GERRITC_LIBRARY_DEPS})
endif()

#########################################################################################
# Executable
#########################################################################################
add_executable(${GERRITC_BINARY} main.cc)

add_dependencies(${GERRITC_BINARY} ${GERRITC_LIBRARY})

target_link_libraries(${GERRITC_BINARY} ${GERRITC_LIBRARY})

set_target_properties(${GERRITC_BINARY} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
