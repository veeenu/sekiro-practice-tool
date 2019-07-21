cmake_minimum_required(VERSION 3.9)

if (WIN32)
  set (CMAKE_GENERATOR_PLATFORM "x64")
  set (CMAKE_CXX_FLAGS "/Ox /EHsc /Zi /Zo /std:c++17")
endif (WIN32)

project(dx11hook C CXX)
set( CMAKE_VERBOSE_MAKEFILE on )

include_directories(dx11hook PUBLIC include)
link_directories(dx11hook PUBLIC lib)
file(GLOB_RECURSE dx11hook_SRC src/*.h src/*.cpp)

add_library(dx11hook MODULE ${dx11hook_SRC})

target_link_libraries(dx11hook libMinHook-x64-v141-mt.lib)
target_link_libraries(dx11hook shlwapi.lib)
set_target_properties(dx11hook PROPERTIES PREFIX "")

add_custom_command(TARGET dx11hook POST_BUILD
  COMMAND "${CMAKE_COMMAND}" -E copy "$<TARGET_FILE:dx11hook>" "D:\\Steam\\steamapps\\common\\Sekiro\\uclib\\jdsd-practice-tool.dll"
  COMMENT "Copying...")