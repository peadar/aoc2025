# Library sources
add_library(aoc2025
    aoc-main.cc
    timeit.cc
    unicode.cc
)

# Public include dir for consumers of the library
target_include_directories(aoc2025 PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

# Optional: library-specific compile flags / definitions
# target_compile_definitions(mylib PRIVATE SOME_DEFINE=1)

