# Static build information.
set(target_exe_name "rustlearningcpp")
set(target_dynlib_name "rustlearningcpplib")
set(target_staticlib_name "rustlearningcppstatic")
set(rust_lib_name "rustlearningrs")
set(rust_exe_name "rustlearningrs")


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


# Find Rust-compiled Executable
set(rust_exe_name ${rust_exe_name}.exe)
find_program(RUST_EXE ${rust_exe_name}
  HINTS
  ${rust_build_location}
  PATHS
  ${rust_build_location}
  )


# Add C++ 'rustlearning' library
set(source_files
  src/lib.cpp
  )
add_library(${target_dynlib_name} SHARED ${source_files})
set_target_properties(${target_dynlib_name}
  PROPERTIES
  OUTPUT_NAME ${target_exe_name})
add_library(${target_staticlib_name} STATIC ${source_files})
set_target_properties(${target_staticlib_name}
  PROPERTIES
  OUTPUT_NAME ${target_exe_name})
target_link_libraries(${target_dynlib_name} ${RUST_LIBRARY})
target_link_libraries(${target_staticlib_name} ${RUST_LIBRARY})


# Add C++ 'rustlearning' executable
set(source_files
  src/main.cpp
  )
add_executable(${target_exe_name} ${source_files})
target_link_libraries(${target_exe_name} ${RUST_LIBRARY} ${target_staticlib_name})


# Add Headers From C++
target_include_directories(${target_exe_name}
  PUBLIC include
  PRIVATE src 
  )
target_include_directories(${target_dynlib_name}
  PUBLIC include
  PRIVATE src 
  )
target_include_directories(${target_staticlib_name}
  PUBLIC include
  PRIVATE src 
  )


# Add Headers From Rust
set(rust_header_dir ${CMAKE_SOURCE_DIR}/rust/include/)
target_include_directories(${target_exe_name}
  PUBLIC ${rust_header_dir}
  )
target_include_directories(${target_dynlib_name}
  PUBLIC ${rust_header_dir}
  )
target_include_directories(${target_staticlib_name}
  PUBLIC ${rust_header_dir}
  )


# Include public header files for installation
set(header_files
  # C++ public header files
  include/cpplib.h

  # Include Rust 'cbindgen' headers
  ${rust_header_dir}/rustlib.h
  )


# Install
install(FILES
  ${RUST_LIBRARY_DLL}
  DESTINATION "bin")
install(FILES
  ${RUST_EXE}
  DESTINATION "bin")
install(FILES
  ${header_files}
  DESTINATION "include")
install(
  TARGETS ${target_exe_name} ${target_dynlib_name} ${target_staticlib_name}
  ARCHIVE DESTINATION "lib"
  LIBRARY DESTINATION "lib"
  RUNTIME DESTINATION "bin")
