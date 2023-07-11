# https://stackoverflow.com/questions/75777485/integrating-uwebsockets-into-a-cmake-project

cmake_minimum_required(VERSION 3.24)
set(CMAKE_C_STANDARD 11)

include(FetchContent)
FetchContent_Declare(
	usockets_content
	GIT_REPOSITORY	https://github.com/uNetworking/uSockets.git
	GIT_TAG			v0.8.6
	GIT_SHALLOW		ON
	GIT_SUBMODULES	""
)
FetchContent_MakeAvailable(usockets_content)

file(GLOB_RECURSE SOURCES ${usockets_content_SOURCE_DIR}/src/*.c)
add_library(uSockets ${SOURCES})

include_directories("${libuv_SOURCE_DIR}/include")
target_include_directories(uSockets PUBLIC ${usockets_content_SOURCE_DIR}/src)
target_compile_definitions(uSockets PRIVATE LIBUS_NO_SSL)