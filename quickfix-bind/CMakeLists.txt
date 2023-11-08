include_directories(include)
include_directories($ENV{HOME}/.local/include)
link_directories($ENV{HOME}/.local/lib)

add_library(quickfixbind SHARED
    src/quickfix.cpp
)
target_link_libraries(quickfixbind PRIVATE quickfix)

add_executable(quickfix_bind_tester
    src/main.c
)
target_link_libraries(quickfix_bind_tester PRIVATE quickfixbind)
