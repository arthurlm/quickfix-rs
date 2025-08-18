cmake_minimum_required(VERSION 3.10)
project(QuickFixBind)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_CURRENT_SOURCE_DIR}/libquickfix/cmake/")

option(HAVE_SSL "Build with SSL" OFF)
option(HAVE_MYSQL "Build with MySQL" OFF)
option(HAVE_POSTGRESQL "Build with PostgreSQL" OFF)

option(QUICKFIX_BIND_EXAMPLES "Build quickfix C binding examples" ON)

if (HAVE_SSL)
    find_package(OpenSSL REQUIRED)
    include_directories(${OPENSSL_INCLUDE_DIR})
    message("-- Building with OpenSSL")
endif()

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

# Add main directory
add_subdirectory(quickfix-bind)

# Add example if asked
if(QUICKFIX_BIND_EXAMPLES)
add_subdirectory(examples)
endif()