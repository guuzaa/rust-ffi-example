cmake_minimum_required(VERSION 3.15)
project(ffi_lib)

# Set C standard
set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Create shared library
add_library(ffi_lib SHARED
    lib.c
)

# Set output directory
set_target_properties(ffi_lib PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
)

# Set include directories
target_include_directories(ffi_lib PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
) 