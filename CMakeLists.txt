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
set(PROJECT_COPYRIGHT "2019, 2020, David Cattermole.")

add_subdirectory(cpp)
