# Add 'rustlearning' executable
set(source_files
  src/main.cpp
  )
add_executable(${PROJECT_NAME} ${source_files})

# Add Headers
target_include_directories(${PROJECT_NAME}
  PUBLIC include
  )

# Rust directory variables
string(TOLOWER ${CMAKE_BUILD_TYPE} rust_build_type)
set(rust_build_location ${CMAKE_SOURCE_DIR}/rust/target/${rust_build_type})

# Add Rust library
set(rust_dll_name ${PROJECT_NAME}.dll)
find_library(RUST_LIBRARY ${rust_dll_name}
  HINTS
      ${rust_build_location}
  PATHS
      ${rust_build_location}
      )
target_link_libraries(${PROJECT_NAME} ${RUST_LIBRARY})

# Get the Rust Library DLL (for windows).
find_path(RUST_LIBRARY_DLL ${rust_dll_name}
  HINTS
      ${rust_build_location}
  PATHS
      ${rust_build_location}
  )
if(EXISTS ${RUST_LIBRARY_DLL})
    set(RUST_LIBRARY_DLL ${RUST_LIBRARY_DLL}/${rust_dll_name})
endif()

# Install Rust-compiled Executable
set(rust_exe_name ${PROJECT_NAME}rs.exe)
find_program(RUST_EXE ${rust_exe_name}
  HINTS
      ${rust_build_location}
  PATHS
      ${rust_build_location}
      )
target_link_libraries(${PROJECT_NAME} ${RUST_LIBRARY})

# Include Rust 'cbindgen' headers in the installation
set(header_files
  ${CMAKE_SOURCE_DIR}/rust/include/lib.h
  )
set_target_properties(${PROJECT_NAME} PROPERTIES PUBLIC_HEADER ${header_files})

# Install
install(FILES
  ${RUST_LIBRARY_DLL}
  DESTINATION "bin")
install(FILES
  ${RUST_LIBRARY}
  DESTINATION "bin")
install(FILES
  ${RUST_EXE}
  DESTINATION "bin")
install(TARGETS ${PROJECT_NAME}
  PUBLIC_HEADER DESTINATION "include"
  LIBRARY DESTINATION "lib"
  RUNTIME DESTINATION "bin")
