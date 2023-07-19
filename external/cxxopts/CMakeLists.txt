include(FetchContent)
FetchContent_Declare(
	cxxopts_content
	GIT_REPOSITORY	https://github.com/jarro2783/cxxopts.git
	GIT_TAG			v3.1.1
)
FetchContent_MakeAvailable(cxxopts_content)

include_directories("${cxxopts_SOURCE_DIR}/include")