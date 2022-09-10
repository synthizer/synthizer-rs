cmake_minimum_required(VERSION 3.15.0)
project(wdl C CXX)

# We disable all of these until they're needed for build size and speed.

SET(WDL_FILES
  #audiobuffercontainer.cpp
  #blowfish.c
  #convoengine.cpp
  #des.cpp
  #fft.c
  #projectcontext.cpp
  resample.cpp
  #rfb_client.cpp
  #rng.cpp
  #scsrc.cpp
  #sha.cpp
  #shm_connection.cpp
  #timing.c
  #win32_utf8.c
)

list(TRANSFORM WDL_FILES PREPEND WDL/)

add_library(wdl_objlib OBJECT ${WDL_FILES})
target_compile_definitions(wdl_objlib PRIVATE
  WDL_RESAMPLE_TYPE=float
)
target_include_directories(wdl_objlib INTERFACE "${CMAKE_CURRENT_SOURCE_DIR}")
