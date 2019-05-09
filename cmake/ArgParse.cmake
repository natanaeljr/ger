cmake_minimum_required(VERSION 3.0)
include(ExternalProject)

#########################################################################################
# ARGPARSE
#########################################################################################
option(DOWNLOAD_ARGPARSE "Download ArgParse v1.3" OFF)

# Try using ArgParse headers from system
if(NOT DOWNLOAD_ARGPARSE)
    find_path(ARGPARSE_INCLUDE_DIR "argparse.hpp")
    if (ARGPARSE_INCLUDE_DIR)
        message(STATUS "Looking for ArgParse headers: ${ARGPARSE_INCLUDE_DIR} - found")
    else()
        message(STATUS "Looking for ArgParse headers - not found")
        set(DOWNLOAD_ARGPARSE ON)
    endif()
endif(NOT DOWNLOAD_ARGPARSE)

# Download ArgParse locally into the binary directory
if(DOWNLOAD_ARGPARSE)
    message(STATUS "ArgParse will be downloaded")
    set(ARGPARSE_TARGET argparse)
    set(ARGPARSE_PREFIX ${CMAKE_BINARY_DIR}/${ARGPARSE_TARGET})
    set(ARGPARSE_INCLUDE_DIR ${ARGPARSE_PREFIX}/include)

    # Download and build
    ExternalProject_Add(${ARGPARSE_TARGET}
        GIT_REPOSITORY "https://github.com/p-ranav/argparse"
        GIT_TAG "v1.3"
        PREFIX ${ARGPARSE_PREFIX}
        CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${ARGPARSE_PREFIX}
        BUILD_COMMAND ""
        UPDATE_COMMAND ""
        TEST_COMMAND "")
endif(DOWNLOAD_ARGPARSE)

# Export Variables
set(ARGPARSE_INCLUDE_DIRS ${ARGPARSE_INCLUDE_DIR} CACHE STRING "ArgParse Include directories")
set(ARGPARSE_DEFINITIONS "" CACHE STRING "ArgParse Definitions")
if(ARGPARSE_TARGET)
set(ARGPARSE_TARGET ${ARGPARSE_TARGET} CACHE STRING "ArgParse Target to add as dependency")
endif()
