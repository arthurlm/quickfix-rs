include_directories(include)

# Add main library
add_library(quickfixbind STATIC
    src/quickfix_bind.cpp
)
target_link_libraries(quickfixbind ${MYSQL_CLIENT_LIBS} ${PostgreSQL_LIBRARIES} quickfix)

# Add option if asked
if (WITH_PRINT_EX_STDOUT)
    target_compile_definitions(quickfixbind PRIVATE "PRINT_QUICKFIX_EX_STDOUT=1")
endif()

if (HAVE_MYSQL)
    target_compile_definitions(quickfixbind PRIVATE "HAVE_MYSQL=1")
endif()

if (HAVE_POSTGRESQL)
    target_compile_definitions(quickfixbind PRIVATE "HAVE_POSTGRESQL=1")
endif()

# Add example if asked
if(QUICKFIX_BIND_EXAMPLES)
add_executable(demo_basic_binding
    examples/demo_basic_binding.c
)
target_link_libraries(demo_basic_binding ${MYSQL_CLIENT_LIBS} ${PostgreSQL_LIBRARIES} quickfixbind)
endif()

# Configure install target
install(TARGETS quickfixbind DESTINATION lib)
