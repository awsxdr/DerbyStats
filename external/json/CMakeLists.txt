include(FetchContent)
FetchContent_Declare(
	json_content
	GIT_REPOSITORY	https://github.com/nlohmann/json.git
	GIT_TAG			v3.11.2
)
FetchContent_MakeAvailable(json_content)

set_property(GLOBAL PROPERTY JSON_SOURCE_DIR "${json_content_SOURCE_DIR}")
