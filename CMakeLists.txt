# CMakeList.txt : CMake project for DerbyStats, include source and define
# project specific logic here.
#
cmake_minimum_required (VERSION 3.8)

# Enable Hot Reload for MSVC compilers if supported.
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

project ("DerbyStats" LANGUAGES C CXX)

# Add source to this project's executable.
add_executable (DerbyStats "DerbyStats.cpp" "DerbyStats.h"  "FileServe.h" "api/GamesController.h" "ScoreboardConnector.h" "ScoreboardConnector.cpp" "Exceptions.h" "state_stores/GameStateStore.cpp" "api/GamesController.cpp" "api/ApiController.h" "state_stores/GameStateStore.h" "api/RostersController.h" "api/RostersController.cpp" "state_stores/RostersStateStore.h" "state_stores/RostersStateStore.cpp")

set_property(TARGET DerbyStats PROPERTY CXX_STANDARD 20)

add_subdirectory(external)

target_link_libraries(DerbyStats uWebSockets asio)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
	add_compile_definitions("DEBUG")
endif()

if (WIN32)
	target_link_libraries(DerbyStats libuv)
	if (CMAKE_BUILD_TYPE STREQUAL "Debug")
		target_link_libraries(DerbyStats "zlibstaticd.lib")
	else ()
		target_link_libraries(DerbyStats "zlibstatic.lib")
	endif ()
	set_property(TARGET DerbyStats PROPERTY COMPILE_OPTIONS "/Zc:__cplusplus")
else ()
	target_link_libraries(DerbyStats z uv "libz.a" "libuv.a")
endif ()

get_property(libuv_dir GLOBAL PROPERTY LIBUV_LIB_DIR)
get_property(zlib_dir GLOBAL PROPERTY ZLIB_LIB_DIR)
get_property(asio_dir GLOBAL PROPERTY ASIO_INCLUDE_DIR)
get_property(json_dir GLOBAL PROPERTY JSON_SOURCE_DIR)

target_link_directories(DerbyStats PUBLIC "${libuv_dir}" "${zlib_dir}")

if(WIN32)
	target_link_libraries(DerbyStats "Ws2_32.lib" "Dbghelp.lib" "Iphlpapi.lib" "Userenv.lib")
endif (WIN32)

include_directories(
	websocketpp
	"${libuv_SOURCE_DIR}/include"
	"${zlib_SOURCE_DIR}"
	"${zlib_BINARY_DIR}"
	"${websocketpp_SOURCE_DIR}"
	"${asio_dir}/asio/include"
	"${cxxopts_SOURCE_DIR}/include"
	"${json_dir}/include"
)

# TODO: Add tests and install targets if needed.
