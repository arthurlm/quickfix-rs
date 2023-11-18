cmake_minimum_required(VERSION 3.10)
project(QuickFixBind)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

option(QUICKFIX_BIND_EXAMPLES "Build quickfix C binding examples" ON)
option(WITH_PRINT_EX_STDOUT, "Enable exception printing to stdout" OFF)

add_subdirectory(quickfix-bind)
