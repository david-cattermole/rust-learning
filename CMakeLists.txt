# Rust Learning project
#
cmake_minimum_required(VERSION 2.8)

# Project configuration.
project(rustlearning)
set(PROJECT_VERSION_MAJOR 0)
set(PROJECT_VERSION_MINOR 1)
set(PROJECT_VERSION_PATCH 0)
set(PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")
set(PROJECT_HOMEPAGE_URL "https://github.com/david-cattermole/rust-learning")
set(PROJECT_AUTHOR "David Cattermole")
set(PROJECT_COPYRIGHT "2019, David Cattermole.")

# Source
set(SOURCE_FILES
         cpp/main.cpp
        )


# 'rustlearning' maya plugin library
add_library(rustlearning SHARED ${SOURCE_FILES})
target_include_directories(rustlearning
  PRIVATE cpp
  )

# TODO: Use cmake 'find_library' so we don't hard-code a path.
set(RUST_LIBRARY "C:/Users/catte/dev/rust-learning_windows/target/release/rustlearning.dll")
target_link_libraries(
  rustlearning
  ${RUST_LIBRARY}
  )
