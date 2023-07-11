# https://stackoverflow.com/questions/75777485/integrating-uwebsockets-into-a-cmake-project

cmake_minimum_required(VERSION 3.24)
include(FetchContent)
FetchContent_Declare(
  uwebsockets_content
  GIT_REPOSITORY	https://github.com/uNetworking/uWebSockets.git
  GIT_TAG			v20.37.0
  GIT_SHALLOW		ON
  GIT_SUBMODULES	""
)
FetchContent_MakeAvailable(uwebsockets_content)
# find_package(ZLIB REQUIRED)
add_library(uWebSockets INTERFACE)
target_include_directories(uWebSockets INTERFACE ${uwebsockets_content_SOURCE_DIR}/src/)
target_link_libraries(uWebSockets INTERFACE uSockets ${zlib_LIBRARIES})