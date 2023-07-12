include(FetchContent)
FetchContent_Declare(
	zlib_content
	GIT_REPOSITORY	https://github.com/madler/zlib.git
	GIT_TAG			v1.2.13
)
FetchContent_MakeAvailable(zlib_content)

set_property(GLOBAL PROPERTY ZLIB_LIB_DIR "${zlib_content_BINARY_DIR}")
