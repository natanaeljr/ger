cmake_minimum_required(VERSION 3.0)

cmake_policy(SET CMP0076 NEW)

#########################################################################################
# Library Sources
#########################################################################################
target_sources(${GERCLI_OBJECT}
  PRIVATE
  change_cmd.cc
  gercli.cc
  config.cc
)

#########################################################################################
# Executable Sources
#########################################################################################
target_sources(${GERCLI_BINARY}
  PRIVATE
  main.cc
)