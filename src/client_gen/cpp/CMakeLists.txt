cmake_minimum_required(VERSION 3.5)
project(project_name)

# OS and compiler checks.
if(WIN32)
    add_definitions(-DWIN32)
    SET(PROJECT_LIBS Ws2_32.lib)
endif()

file(GLOB HEADERS "*.hpp" "model/*.hpp")
SET_SOURCE_FILES_PROPERTIES(${HEADERS} PROPERTIES HEADER_FILE_ONLY TRUE)
file(GLOB SRC "*.cpp" "model/*.cpp")
add_executable(project_name ${HEADERS} ${SRC})
TARGET_LINK_LIBRARIES(project_name ${PROJECT_LIBS})