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

add_subdirectory(external)

# Add source to this project's executable.
add_executable (DerbyStats "DerbyStats.cpp" "DerbyStats.h")
message("Build Type: ${CMAKE_BUILD_TYPE}")
target_link_libraries(DerbyStats uWebSockets)

if (WIN32)
	target_link_libraries(DerbyStats libuv)
	if (CMAKE_BUILD_TYPE STREQUAL "Debug")
		target_link_libraries(DerbyStats "zlibstaticd.lib")
	else (CMAKE_BUILD_TYPE STREQUAL "Debug")
		target_link_libraries(DerbyStats "zlibstatic.lib")
	endif (CMAKE_BUILD_TYPE STREQUAL "Debug")
else (WIN32)
	target_link_libraries(DerbyStats z uv "libz.a" "libuv.a")
endif (WIN32)

get_property(libuv_dir GLOBAL PROPERTY LIBUV_LIB_DIR)
get_property(zlib_dir GLOBAL PROPERTY ZLIB_LIB_DIR)

target_link_directories(DerbyStats PUBLIC "${libuv_dir}" "${zlib_dir}")

if(WIN32)
	target_link_libraries(DerbyStats "Ws2_32.lib" "Dbghelp.lib" "Iphlpapi.lib" "Userenv.lib")
endif (WIN32)

include_directories(
	"${libuv_SOURCE_DIR}/include"
	"${zlib_SOURCE_DIR}"
	"${zlib_BINARY_DIR}"
)

file(COPY "ui/dist/" DESTINATION "${CMAKE_CURRENT_BINARY_DIR}/ui")

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET DerbyStats PROPERTY CXX_STANDARD 20)
endif()

# TODO: Add tests and install targets if needed.
