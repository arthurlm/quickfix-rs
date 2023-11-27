cmake_minimum_required(VERSION 3.10)
project(QuickFixBind)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_CURRENT_SOURCE_DIR}/libquickfix/cmake/")

option(HAVE_MYSQL "Build with MySQL" OFF)
option(HAVE_POSTGRESQL "Build with PostgreSQL" OFF)

option(QUICKFIX_BIND_EXAMPLES "Build quickfix C binding examples" ON)
option(WITH_PRINT_EX_STDOUT, "Enable exception printing to stdout" OFF)

if (HAVE_MYSQL)
    find_package(MySQL REQUIRED)
    include_directories(${MYSQL_INCLUDE_DIR})
    message("-- Building with MySQL")
endif()

if (HAVE_POSTGRESQL)
    find_package(PostgreSQL REQUIRED)
    include_directories(${PostgreSQL_INCLUDE_DIRS})
    message("-- Building with POSTGRESQL")
endif()

add_subdirectory(quickfix-bind)
