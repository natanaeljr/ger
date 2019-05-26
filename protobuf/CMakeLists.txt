cmake_minimum_required(VERSION 3.0)
project(ger-protobuf LANGUAGES CXX)

set(CMAKE_CXX_FLAGS_DEBUG "")
set(CMAKE_CXX_FLAGS_RELEASE "")

################################################################################
# Generate protobuf files and LIM L2 protobuf lib
################################################################################
set(GER_PROTOBUF_FILES
    gerrit/accounts
    gerrit/changes
)

foreach(proto_file ${GER_PROTOBUF_FILES})
    add_custom_command(
            OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${proto_file}.pb.h
            OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${proto_file}.pb.cc
            COMMAND protoc --proto_path=${CMAKE_CURRENT_SOURCE_DIR}/
                --cpp_out=${CMAKE_CURRENT_BINARY_DIR}/
                ${CMAKE_CURRENT_SOURCE_DIR}/${proto_file}.proto
            DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/${proto_file}.proto
    )

    set(GER_PROTOBUF_HEADERS ${GER_PROTOBUF_HEADERS} ${CMAKE_CURRENT_BINARY_DIR}/${proto_file}.pb.h)
    set(GER_PROTOBUF_SRC ${GER_PROTOBUF_SRC} ${CMAKE_CURRENT_BINARY_DIR}/${proto_file}.pb.cc)
endforeach(proto_file)

add_custom_target(ger_protobuf_files ALL DEPENDS ${GER_PROTOBUF_HEADERS} ${GER_PROTOBUF_SRC})

################################################################
# Generate Library from protbuf files
###############################################################
set_source_files_properties(${GER_PROTOBUF_SRC} PROPERTIES COMPILE_FLAGS "-Wno-shadow -Wno-unused-parameter")

add_library (ger_protobuf STATIC ${GER_PROTOBUF_SRC})

add_dependencies (ger_protobuf ger_protobuf_files)