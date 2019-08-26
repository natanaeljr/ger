cmake_minimum_required(VERSION 3.0)

cmake_policy(SET CMP0076 NEW)

#########################################################################################
# Library Sources
#########################################################################################
target_sources(${GERLIB_LIBRARY}
  PRIVATE
  json.cc
)
