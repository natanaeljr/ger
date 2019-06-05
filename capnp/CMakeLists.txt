cmake_minimum_required(VERSION 3.0)
project(ger-capnp LANGUAGES CXX)

##################################################################################################
# Generate Cap'n Proto files
##################################################################################################
set(GER_CAPNP_FILES
    util/listmap
    gerrit/accounts
    gerrit/changes
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

##################################################################################################
# Generate Library from capnp files
##################################################################################################
set_source_files_properties(${GER_CAPNP_SRCS} PROPERTIES COMPILE_FLAGS "-Wno-shadow -Wno-unused-parameter")

add_library (${GER_CAPNP_LIB} STATIC ${GER_CAPNP_SRCS})
target_link_libraries(${GER_CAPNP_LIB}
    capnp-json
    capnp-rpc
    capnp
    kj
)

add_dependencies (${GER_CAPNP_LIB} ger_capnp_files)
