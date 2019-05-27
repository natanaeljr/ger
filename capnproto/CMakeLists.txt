cmake_minimum_required(VERSION 3.0)
project(ger-capnproto LANGUAGES CXX)

set(CMAKE_CXX_FLAGS_DEBUG "")
set(CMAKE_CXX_FLAGS_RELEASE "")

################################################################################
# Generate capnproto files and LIM L2 capnproto lib
################################################################################
set(GER_CAPNPROTO_FILES
    gerrit/changes
)

foreach(capnp_file ${GER_CAPNPROTO_FILES})
    add_custom_command(
            OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${capnp_file}.capnp.h
            OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${capnp_file}.capnp.c++
            COMMAND capnp compile -I${CMAKE_CURRENT_SOURCE_DIR}
                --output=c++:${CMAKE_CURRENT_BINARY_DIR}
                ${CMAKE_CURRENT_SOURCE_DIR}/${capnp_file}.capnp
            DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/${capnp_file}.capnp
    )

    set(GER_CAPNPROTO_HEADERS ${GER_CAPNPROTO_HEADERS} ${CMAKE_CURRENT_BINARY_DIR}/${capnp_file}.capnp.h)
    set(GER_CAPNPROTO_SRC ${GER_CAPNPROTO_SRC} ${CMAKE_CURRENT_BINARY_DIR}/${capnp_file}.capnp.c++)
endforeach(capnp_file)

add_custom_target(ger_capnproto_files ALL DEPENDS ${GER_CAPNPROTO_HEADERS} ${GER_CAPNPROTO_SRC})

################################################################
# Generate Library from protbuf files
###############################################################
set_source_files_properties(${GER_CAPNPROTO_SRC} PROPERTIES COMPILE_FLAGS "-Wno-shadow -Wno-unused-parameter")

add_library (ger_capnproto STATIC ${GER_CAPNPROTO_SRC})
target_link_libraries(ger_capnproto
    capnp-json
    capnp-rpc
    capnp
    kj
)

add_dependencies (ger_capnproto ger_capnproto_files)