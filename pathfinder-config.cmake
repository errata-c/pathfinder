
include_guard(DIRECTORY)
include(CMakeFindDependencyMacro)

####### Expanded from @PACKAGE_INIT@ by configure_package_config_file() #######
####### Any changes to this file will be overwritten by the next CMake run ####
####### The input file was ConfigTemplate.cmake.in                            ########

get_filename_component(PACKAGE_PREFIX_DIR "${CMAKE_CURRENT_LIST_DIR}/../../" ABSOLUTE)

####################################################################################

set(PACK_NAME pathfinder)
set(PACK_VERSION 1.0.0)
set(PACK_ROOT ${PACKAGE_PREFIX_DIR})
set(PACK_CONFIG_DIR share/pathfinder)

set(pathfinder_VERSION ${PACK_VERSION})
set(pathfinder_PREFIX_DIR ${PACK_ROOT})
set(pathfinder_ROOT_DIR ${PACK_ROOT})
set(pathfinder_CONFIG_DIR ${PACK_CONFIG_DIR})

mark_as_advanced(pathfinder_VERSION pathfinder_PREFIX_DIR pathfinder_ROOT_DIR pathfinder_CONFIG_DIR)

# Create imported target ez-window
add_library(pathfinder SHARED IMPORTED)

set_target_properties(pathfinder PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${PACKAGE_PREFIX_DIR}/include"
)

set_property(TARGET pathfinder APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(pathfinder PROPERTIES
  IMPORTED_LOCATION_RELEASE "${PACKAGE_PREFIX_DIR}/bin/pathfinder.dll"
  IMPORTED_IMPLIB_RELEASE "${PACKAGE_PREFIX_DIR}/lib/pathfinderd.lib"
  )
  
set_property(TARGET pathfinder APPEND PROPERTY IMPORTED_CONFIGURATIONS DEBUG)
set_target_properties(pathfinder PROPERTIES
  IMPORTED_LOCATION_DEBUG "${PACKAGE_PREFIX_DIR}/bin/pathfinderd.dll"
  IMPORTED_IMPLIB_DEBUG "${PACKAGE_PREFIX_DIR}/lib/pathfinderd.lib"
  )

unset(PACK_NAME)
unset(PACK_VERSION)
unset(PACK_ROOT)
unset(PACK_CONFIG_DIR)


