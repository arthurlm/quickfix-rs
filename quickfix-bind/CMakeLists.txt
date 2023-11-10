include_directories(include)
include_directories($ENV{HOME}/.local/include)
link_directories($ENV{HOME}/.local/lib)

add_library(quickfixbind SHARED
    src/quickfix_bind.cpp
)
target_link_libraries(quickfixbind PRIVATE quickfix)

add_executable(demo_basic_binding
    examples/demo_basic_binding.c
)
target_link_libraries(demo_basic_binding PRIVATE quickfixbind)
