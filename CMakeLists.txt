cmake_minimum_required(VERSION 3.0)
project(ger VERSION 0.1 LANGUAGES CXX C)

#########################################################################################
# Configs
#########################################################################################
set(CMAKE_VERBOSE_MAKEFILE OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_FLAGS_DEBUG "-g -ggdb -pg -fsanitize=undefined -Werror -Wall")
set(CMAKE_CXX_FLAGS_RELEASE "-O2 -DNDEBUG")

#########################################################################################
# Global Defines
#########################################################################################
set(GER_INCLUDE_DIRS "include")
set(GER_LIBRARY "gerlib")
set(GER_BINARY "ger")
set(GER_TEST "ger-test")

#########################################################################################
# Dependencies
#########################################################################################
include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()
include_directories(${CONAN_INCLUDE_DIRS})
add_definitions(${CONAN_DEFINES})
link_libraries(${CONAN_LIBS})

#########################################################################################
# Project
#########################################################################################
add_subdirectory(src)

#########################################################################################
# Testing
#########################################################################################
if(BUILD_TESTING)
  add_subdirectory(test)
endif()
