include_directories(../quickfix-bind/include)

add_executable(demo_basic_binding demo_basic_binding.c)
target_link_libraries(demo_basic_binding ${MYSQL_CLIENT_LIBS} ${PostgreSQL_LIBRARIES} quickfixbind)
