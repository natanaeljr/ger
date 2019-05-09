cmake_minimum_required(VERSION 3.0)

include(${CMAKE_SOURCE_DIR}/cmake/GoogleTest.cmake)
include_directories(${GOOGLETEST_INCLUDE_DIRS})

add_definitions(${GOOGLETEST_DEFINITIONS})

add_executable(run-test
    test.cc
)

target_link_libraries(run-test
    ${GERRITC_LIBRARY}
    ${GOOGLETEST_MAIN_LIBRARIES}
    ${GOOGLETEST_LIBRARIES}
    pthread
)

set(RUNTEST_DEPENDENCIES
    ${GOOGLETEST_TARGET}
)

add_dependencies(run-test 
    ${GERRITC_LIBRARY}
    ${RUNTEST_DEPENDENCIES}
)

set_target_properties(run-test PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
