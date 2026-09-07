# Compile Rust code into static library via Cargo
set(RUST_OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/cargo_out")

add_custom_target(cargo_build
    COMMAND cargo build --manifest-path "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml" --target-dir "${RUST_OUTPUT_DIR}"
    WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
    COMMENT "Building CodeHalo Rust engine and CXX-Qt bridge"
)

add_library(codehalo_rust INTERFACE)
add_dependencies(codehalo_rust cargo_build)

target_include_directories(codehalo_rust INTERFACE
    "${CMAKE_CURRENT_SOURCE_DIR}/src"
    "${RUST_OUTPUT_DIR}/cxx-qt-gen/include"
)

target_link_directories(codehalo_rust INTERFACE
    "${RUST_OUTPUT_DIR}/debug"
)

target_link_libraries(codehalo_rust INTERFACE
    codehalo_rust
)
