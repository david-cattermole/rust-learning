# Static build information.
set(target_exe_name "rustlearningcpp")
set(target_lib_name "rustlearningcpplib")
set(rust_lib_name "rustlearningrs")
set(rust_exe_name "rustlearningrs")


# Add C++ 'rustlearning' executable
set(source_files
  src/main.cpp
  )
add_executable(${target_exe_name} ${source_files})

# Add C++ 'rustlearning' library
set(source_files
  src/lib.cpp
  )
add_library(${target_lib_name} SHARED ${source_files})

# Add Headers From C++
target_include_directories(${target_exe_name}
  PUBLIC include
  )
target_include_directories(${target_lib_name}
  PUBLIC include
  )
target_include_directories(${target_exe_name}
  PRIVATE src
  )
target_include_directories(${target_lib_name}
  PRIVATE src
  )

# Add Headers From Rust
set(rust_header_dir ${CMAKE_SOURCE_DIR}/rust/include/)
target_include_directories(${target_exe_name}
  PUBLIC ${rust_header_dir}
  )
target_include_directories(${target_lib_name}
  PUBLIC ${rust_header_dir}
  )

# Rust directory variables
string(TOLOWER ${CMAKE_BUILD_TYPE} rust_build_type)
set(rust_build_location ${CMAKE_SOURCE_DIR}/rust/target/${rust_build_type})

# Add Rust library
set(rust_dll_name ${rust_lib_name}.dll)
find_library(RUST_LIBRARY ${rust_dll_name}
  HINTS
      ${rust_build_location}
  PATHS
      ${rust_build_location}
  )
target_link_libraries(${target_exe_name} ${RUST_LIBRARY})
target_link_libraries(${target_lib_name} ${RUST_LIBRARY})

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
set(rust_exe_name ${rust_exe_name}.exe)
find_program(RUST_EXE ${rust_exe_name}
  HINTS
      ${rust_build_location}
  PATHS
      ${rust_build_location}
  )
target_link_libraries(${target_exe_name} ${RUST_LIBRARY})
target_link_libraries(${target_lib_name} ${RUST_LIBRARY})

# Include Rust 'cbindgen' headers in the installation
set(rust_header_files
  ${rust_header_dir}/rustlib.h
  )

set_target_properties(${target_exe_name} PROPERTIES PUBLIC_HEADER ${rust_header_files})
set_target_properties(${target_lib_name} PROPERTIES PUBLIC_HEADER ${rust_header_files})

# Include C++ public header files, for installation
set(cpp_header_files
  include/cpplib.h
  )
set_target_properties(${target_exe_name} PROPERTIES PUBLIC_HEADER ${cpp_header_files})
set_target_properties(${target_lib_name} PROPERTIES PUBLIC_HEADER ${cpp_header_files})

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
install(TARGETS ${target_exe_name}
  PUBLIC_HEADER DESTINATION "include"
  LIBRARY DESTINATION "lib"
  RUNTIME DESTINATION "bin")
install(TARGETS ${target_lib_name}
  PUBLIC_HEADER DESTINATION "include"
  LIBRARY DESTINATION "lib"
  ARCHIVE DESTINATION "lib"
  RUNTIME DESTINATION "bin")
