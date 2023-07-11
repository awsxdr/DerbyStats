include(FetchContent)
FetchContent_Declare(
	zlib_content
	GIT_REPOSITORY	https://github.com/madler/zlib.git
	GIT_TAG			v1.2.13
)
FetchContent_MakeAvailable(zlib_content)
# file(GLOB_RECURSE SOURCES ${zlib_content_SOURCE_DIR}/*.c)
# add_library(ZLIB ${SOURCES})
# 
# set(GLOBAL	ZLIB_LIBRARY		"${zlib_content_BINARY_FILES}")
# set(GLOBAL	ZLIB_INCLUDE_DIR	"${zlib_content_SOURCE_FILES}" "${zlib_content_BINARY_FILES}")
set_property(GLOBAL PROPERTY ZLIB_LIB_DIR "${zlib_content_BINARY_DIR}")
