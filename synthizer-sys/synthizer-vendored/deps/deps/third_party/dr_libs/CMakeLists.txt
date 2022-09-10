cmake_minimum_required(VERSION 3.15.0)
project(syz_deps C CXX)

add_library(dr_libs INTERFACE)
target_include_directories(dr_libs INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")
