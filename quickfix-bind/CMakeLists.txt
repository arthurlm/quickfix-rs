include_directories(include)

## Uncomment / edit bellow lines to your need.
# include_directories($ENV{HOME}/.local/include)
# link_directories($ENV{HOME}/.local/lib)

add_library(quickfixbind STATIC
    src/quickfix_bind.cpp
)
target_link_libraries(quickfixbind PRIVATE quickfix)

if(QUICKFIX_BIND_EXAMPLES)
add_executable(demo_basic_binding
    examples/demo_basic_binding.c
)
target_link_libraries(demo_basic_binding PRIVATE quickfixbind)
endif()

install(TARGETS quickfixbind DESTINATION lib)
