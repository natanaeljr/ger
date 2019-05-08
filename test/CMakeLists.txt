cmake_minimum_required(VERSION 3.0)

include(${CMAKE_SOURCE_DIR}/cmake/GoogleTest.cmake)
include_directories(${GOOGLETEST_INCLUDE_DIRS})

add_definitions(${GOOGLETEST_DEFINITIONS})

add_executable(run-test
    main.cc
)

target_link_libraries(run-test
    ${GOOGLETEST_MAIN_LIBRARIES}
    ${GOOGLETEST_LIBRARIES}
    pthread
)

if(GOOGLETEST_TARGET)
add_dependencies(run-test ${GOOGLETEST_TARGET})
endif()

set_target_properties(run-test PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
