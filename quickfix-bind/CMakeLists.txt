include_directories(include)

# Add main library
add_library(quickfixbind STATIC
    src/quickfix_bind.cpp
)
target_link_libraries(quickfixbind PRIVATE quickfix)

# Add example if asked
if(QUICKFIX_BIND_EXAMPLES)
add_executable(demo_basic_binding
    examples/demo_basic_binding.c
)
target_link_libraries(demo_basic_binding PRIVATE quickfixbind)
endif()

# Configure install target
install(TARGETS quickfixbind DESTINATION lib)
