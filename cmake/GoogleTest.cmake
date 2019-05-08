cmake_minimum_required(VERSION 3.0)
include(ExternalProject)

#########################################################################################
# GoogleTest
#########################################################################################
option(DOWNLOAD_GOOGLETEST "Download GoogleTest release-1.8.1" OFF)

# Try using GTest/GMock headers from system
if(NOT DOWNLOAD_GOOGLETEST)
    find_package(GTest QUIET)
    if(GTEST_FOUND)
        find_path(GTEST_INCLUDE_DIR "gtest/gtest.h")
        find_path(GMOCK_INCLUDE_DIR "gmock/gmock.h")
        if (GTEST_INCLUDE_DIR)
            message(STATUS "Looking for GTest headers: ${GTEST_INCLUDE_DIR} - found")
        else()
            message(STATUS "Looking for GTest headers - not found")
            set(DOWNLOAD_GOOGLETEST ON)
        endif()
        if (GMOCK_INCLUDE_DIR)
            message(STATUS "Looking for GMock headers: ${GMOCK_INCLUDE_DIR} - found")
        else()
            message(STATUS "Looking for GMock headers - not found")
            set(DOWNLOAD_GOOGLETEST ON)
        endif()
    else(GTEST_FOUND)
        message(STATUS "GTest package - not found")
        set(DOWNLOAD_GOOGLETEST ON)
    endif(GTEST_FOUND)
endif(NOT DOWNLOAD_GOOGLETEST)

# Download GoogleTest locally into the binary directory
if(DOWNLOAD_GOOGLETEST)
    message(STATUS "GoogleTest will be downloaded")
    set(GOOGLETEST_TARGET googletest)
    set(GOOGLETEST_PREFIX ${CMAKE_BINARY_DIR}/${GOOGLETEST_TARGET})

    set(GTEST_INCLUDE_DIRS ${GOOGLETEST_PREFIX}/include)
    set(GMOCK_INCLUDE_DIRS ${GOOGLETEST_PREFIX}/include)
    set(GTEST_LIBRARIES ${GOOGLETEST_PREFIX}/lib/libgtest.a)
    set(GMOCK_LIBRARIES ${GOOGLETEST_PREFIX}/lib/libgmock.a)
    set(GTEST_MAIN_LIBRARIES ${GOOGLETEST_PREFIX}/lib/libgmock_main.a)

    # Download and build
    ExternalProject_Add(${GOOGLETEST_TARGET}
        GIT_REPOSITORY "https://github.com/google/googletest"
        GIT_TAG "release-1.8.1"
        PREFIX ${GOOGLETEST_PREFIX}
        CMAKE_ARGS -DCMAKE_INSTALL_PREFIX=${GOOGLETEST_PREFIX}
        BUILD_COMMAND ""
        UPDATE_COMMAND ""
        TEST_COMMAND "")

endif(DOWNLOAD_GOOGLETEST)

# Export Variables
set(GOOGLETEST_INCLUDE_DIRS ${GTEST_INCLUDE_DIRS} ${GMOCK_INCLUDE_DIRS} CACHE STRING "GTest/GMock Include directories")
set(GOOGLETEST_LIBRARIES ${GTEST_LIBRARIES} ${GMOCK_LIBRARIES})
set(GOOGLETEST_MAIN_LIBRARIES ${GTEST_MAIN_LIBRARIES})
set(GOOGLETEST_DEFINITIONS "" CACHE STRING "GTest/GMock Definitions")
if(GOOGLETEST_TARGET)
    set(GOOGLETEST_TARGET ${GOOGLETEST_TARGET} CACHE STRING "GoogleTest target dependency")
endif(GOOGLETEST_TARGET)