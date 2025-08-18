include_directories(include)

# Add main library
add_library(quickfixbind STATIC
    src/quickfix_bind.cpp
)
target_link_libraries(quickfixbind ${MYSQL_CLIENT_LIBS} ${PostgreSQL_LIBRARIES} ${OPENSSL_LIBRARIES})

if (WIN32)
    target_link_libraries(quickfixbind debug quickfixd)
    target_link_libraries(quickfixbind optimized quickfix)
else()
    target_link_libraries(quickfixbind quickfix)
endif()

# Add option if asked
if (HAVE_SSL)
    target_compile_definitions(quickfixbind PRIVATE "HAVE_SSL=1")
endif()

if (HAVE_MYSQL)
    target_compile_definitions(quickfixbind PRIVATE "HAVE_MYSQL=1")
endif()

if (HAVE_POSTGRESQL)
    target_compile_definitions(quickfixbind PRIVATE "HAVE_POSTGRESQL=1")
endif()

# Configure install target
install(TARGETS quickfixbind DESTINATION lib)
