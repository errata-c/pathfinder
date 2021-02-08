cmake_minimum_required(VERSION 3.15)

project(pathfinder
VERSION 1.0.0)

include(GNUInstallDirs)

execute_process(COMMAND "cargo" "build"
	WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)

execute_process(COMMAND "cargo" "build" "--release"
	WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)

execute_process(
	COMMAND "cbindgen" "--config" "cbindgen.toml" "--lockfile" "../Cargo.lock" "-o" "include/pathfinder.h"
	WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/c"
)

set(PATHFINDER_RELEASE_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/target/release")
set(PATHFINDER_DEBUG_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/target/debug")

if(${CMAKE_SYSTEM_NAME} STREQUAL "Windows")
	set(CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}/pathfinder")
endif()

install(
	FILES "${CMAKE_CURRENT_SOURCE_DIR}/c/include/pathfinder.h"
	TYPE INCLUDE
)
install(DIRECTORY
	"${CMAKE_CURRENT_SOURCE_DIR}/resources/debug-fonts"
	"${CMAKE_CURRENT_SOURCE_DIR}/resources/fonts"
	"${CMAKE_CURRENT_SOURCE_DIR}/resources/shaders"
	"${CMAKE_CURRENT_SOURCE_DIR}/resources/textures"
	DESTINATION "share/pathfinder/resources"
)
install(FILES
	"${CMAKE_CURRENT_SOURCE_DIR}/resources/MANIFEST"
	DESTINATION "share/pathfinder/resources"
)
install(FILES
	"${CMAKE_CURRENT_SOURCE_DIR}/pathfinder-config.cmake"
	DESTINATION "share/pathfinder"
)

# if windows, include exp and pdb, else just the .so files
if(${CMAKE_SYSTEM_NAME} STREQUAL "Windows")
	install(FILES
		"${PATHFINDER_RELEASE_ROOT}/pathfinder.dll"
		"${PATHFINDER_RELEASE_ROOT}/pathfinder.dll.exp"
		"${PATHFINDER_RELEASE_ROOT}/pathfinder.pdb"
		DESTINATION "bin/release"
	)
	install(FILES
		"${PATHFINDER_RELEASE_ROOT}/pathfinder.dll.lib"
		DESTINATION "lib/release"
	)

	install(FILES
		"${PATHFINDER_DEBUG_ROOT}/pathfinder.dll"
		"${PATHFINDER_DEBUG_ROOT}/pathfinder.dll.exp"
		"${PATHFINDER_DEBUG_ROOT}/pathfinder.pdb"
		DESTINATION "bin/debug"
	)

	install(FILES
		"${PATHFINDER_DEBUG_ROOT}/pathfinder.dll.lib"
		DESTINATION "lib/debug"
	)
elseif(${CMAKE_SYSTEM_NAME} STREQUAL "Linux")
	install(FILES
		"${PATHFINDER_RELEASE_ROOT}/pathfinder.so"
		DESTINATION "bin/release"
	)
	install(FILES
		"${PATHFINDER_DEBUG_ROOT}/pathfinder.so"
		DESTINATION "bin/debug"
	)
endif()







