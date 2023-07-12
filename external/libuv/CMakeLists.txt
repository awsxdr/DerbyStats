include(FetchContent)
FetchContent_Declare(
	libuv_content
	GIT_REPOSITORY	https://github.com/libuv/libuv.git
	GIT_TAG			v1.46.0
)
FetchContent_MakeAvailable(libuv_content)

# file(GLOB_RECURSE SOURCES ${libuv_content_SOURCE_DIR}/src/*.c)
# add_library(libuv INTERFACE ${SOURCES})
# 
# target_include_directories(libuv INTERFACE ${libuv_content_SOURCE_DIR}/include/)

set_property(GLOBAL PROPERTY LIBUV_LIB_DIR "${libuv_content_BINARY_DIR}")
