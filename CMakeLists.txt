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
set(CMAKE_CXX_FLAGS "-pg -fsanitize=undefined -Werror -Wall")
set(CMAKE_CXX_FLAGS_DEBUG "-g -ggdb")
set(CMAKE_CXX_FLAGS_RELEASE "-O2 -DNDEBUG")

#########################################################################################
# Global Defines
#########################################################################################
set(GER_INCLUDE_DIR ${CMAKE_SOURCE_DIR}/include)
set(GER_LIBRARY "gerlib")
set(GER_BINARY "ger")
set(GER_TEST "ger-test")

set(GER_CAPNP_SOURCE_DIR ${CMAKE_SOURCE_DIR}/capnp)
set(GER_CAPNP_OUTPUT_DIR ${CMAKE_BINARY_DIR}/capnp)
set(GER_CAPNP_LIB ger_capnp)

#########################################################################################
# Dependencies
#########################################################################################
# Conan
include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()
include_directories(${CONAN_INCLUDE_DIRS})
add_definitions(${CONAN_DEFINES})
link_libraries(${CONAN_LIBS})
# Docopt
add_subdirectory(third_party/docopt)
include_directories(third_party/docopt)
link_libraries(docopt)
# enum_t
include_directories(third_party/enum_t/include)
# FMT
add_subdirectory(third_party/fmt)
include_directories(third_party/fmt/include)
link_libraries(fmt)

#########################################################################################
# Project
#########################################################################################
# Cap'n Proto
include_directories(${GER_CAPNP_SOURCE_DIR})
include_directories(${GER_CAPNP_OUTPUT_DIR})
add_subdirectory(capnp)
link_libraries(${GER_CAPNP_LIB})
# Source code
include_directories(${GER_INCLUDE_DIR})
add_subdirectory(src)

#########################################################################################
# Testing
#########################################################################################
if(BUILD_TESTING)
  add_subdirectory(test)
endif()
