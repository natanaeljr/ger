cmake_minimum_required(VERSION 3.0)
project(gerritc VERSION 0.1 LANGUAGES CXX C)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/Modules/")

#########################################################################################
# General
#########################################################################################
option(BUILD_DEPS "Build project dependencies" OFF)

set(CMAKE_VERBOSE_MAKEFILE OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_FLAGS_DEBUG "-g -ggdb -pg -fsanitize=undefined -Werror -Wall")
set(CMAKE_CXX_FLAGS_RELEASE "-O2 -DNDEBUG")

#########################################################################################
# Project
#########################################################################################
set(GERRITC_INCLUDE_DIR "include")
set(GERRITC_LIBRARY "gerritc")
set(GERRITC_BINARY "gerrit")

#########################################################################################
# Dependencies
#########################################################################################
if(BUILD_DEPS)
    message(STATUS "Building project dependencies enabled")
    set(DOWNLOAD_GSL ON CACHE BOOL "")
    set(DOWNLOAD_GOOGLETEST ON CACHE BOOL "")
else()
    message(STATUS "Trying to use dependencies from system")
endif()

include(cmake/GSL.cmake)
add_definitions(${GSL_DEFINITIONS})

#########################################################################################
# Headers
#########################################################################################
include_directories(${GERRITC_INCLUDE_DIR})
include_directories(${GSL_INCLUDE_DIRS})

#########################################################################################
# Sub-directories
#########################################################################################
add_subdirectory(src)

#########################################################################################
# Testing
#########################################################################################
if(BUILD_TESTING)
    add_subdirectory(test)
endif(BUILD_TESTING)

#########################################################################################
# Install
#########################################################################################
install(FILES "src/${GERRITC_BINARY}" DESTINATION "bin")
