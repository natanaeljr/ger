cmake_minimum_required(VERSION 3.0)
project(ger-capnp LANGUAGES CXX)

##################################################################################################
# Generate Cap'n Proto files
##################################################################################################
set(GER_CAPNP_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set(GER_CAPNP_OUTPUT_DIR ${CMAKE_CURRENT_BINARY_DIR})
set(GER_CAPNP_FILES
    gerrit/accounts
    gerrit/changes
    util/listmap
)

foreach(capnp_file ${GER_CAPNP_FILES})
    add_custom_command(
            OUTPUT ${GER_CAPNP_OUTPUT_DIR}/${capnp_file}.capnp.h
            OUTPUT ${GER_CAPNP_OUTPUT_DIR}/${capnp_file}.capnp.c++
            COMMAND capnp compile -I${CMAKE_CURRENT_SOURCE_DIR}
                --output=c++:${GER_CAPNP_OUTPUT_DIR}
                ${CMAKE_CURRENT_SOURCE_DIR}/${capnp_file}.capnp
            DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/${capnp_file}.capnp
    )

    set(GER_CAPNP_HEADERS ${GER_CAPNP_HEADERS} ${GER_CAPNP_OUTPUT_DIR}/${capnp_file}.capnp.h)
    set(GER_CAPNP_SRCS ${GER_CAPNP_SRCS} ${GER_CAPNP_OUTPUT_DIR}/${capnp_file}.capnp.c++)
endforeach(capnp_file)

add_custom_target(ger_capnp_files ALL DEPENDS ${GER_CAPNP_HEADERS} ${GER_CAPNP_SRCS})

set_source_files_properties(${GER_CAPNP_SRCS} PROPERTIES COMPILE_FLAGS "-Wno-shadow -Wno-unused-parameter")

##################################################################################################
# Generate Library from capnp files
##################################################################################################
add_library (${GERCAPNP_LIBRARY} STATIC )
target_sources(${GERCAPNP_LIBRARY}
    PRIVATE
    ${GER_CAPNP_SRCS}
)
target_include_directories(${GERCAPNP_LIBRARY}
    PUBLIC
    ${GER_CAPNP_OUTPUT_DIR}
    ${GER_CAPNP_SOURCE_DIR}
)
target_link_libraries(${GERCAPNP_LIBRARY}
    PRIVATE
    capnp-json
    capnp-rpc
    capnp
    kj
)
add_dependencies (${GERCAPNP_LIBRARY} ger_capnp_files)
