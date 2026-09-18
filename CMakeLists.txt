cmake_minimum_required(VERSION 3.20)
project(teaclave-sgx-sdk LANGUAGES C CXX)

include(GNUInstallDirs)

# =============================================================================
# Options
# =============================================================================
option(BUILD_SYSROOT "Build the SGX sysroot (trusted Rust rlibs)" ON)
option(BUILD_URTS    "Build sgx_urts (untrusted runtime)"         ON)

set(SGX_TARGET "x86_64-unknown-linux-sgx" CACHE STRING
    "Rust target triple for the SGX enclave")
set(SGX_SYSROOT_FEATURES "untrusted_fs;untrusted_time;net;thread" CACHE STRING
    "Semicolon-separated std features to enable in the sysroot build")
set(SGX_SYSROOT_OUTPUT "" CACHE PATH
    "Where to install the sysroot (default: <source>/sysroot)")

# =============================================================================
# Derived paths
# =============================================================================
set(SDK_ROOT ${CMAKE_CURRENT_SOURCE_DIR})
set(TARGET_JSON ${SDK_ROOT}/rustlib/${SGX_TARGET}.json)

if(SGX_SYSROOT_OUTPUT STREQUAL "")
    set(SGX_SYSROOT_OUTPUT ${SDK_ROOT}/sysroot)
endif()

if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CARGO_PROFILE_FLAG "--release")
    set(CARGO_OUT_DIR "release")
else()
    set(CARGO_PROFILE_FLAG "")
    set(CARGO_OUT_DIR "debug")
endif()

# Convert semicolon-separated list to comma-separated for cargo --features
string(REPLACE ";" "," SGX_SYSROOT_FEATURES_CSV "${SGX_SYSROOT_FEATURES}")

# =============================================================================
# Target: sgx_sysroot — builds trusted Rust rlibs for x86_64-unknown-linux-sgx
# =============================================================================
if(BUILD_SYSROOT)
    set(SYSROOT_STAMP ${CMAKE_CURRENT_BINARY_DIR}/sgx_sysroot.stamp)
    set(SYSROOT_LIB_DIR ${SGX_SYSROOT_OUTPUT}/lib/rustlib/${SGX_TARGET}/lib)

    add_custom_command(
        OUTPUT ${SYSROOT_STAMP}
        COMMENT "Building SGX sysroot (${CARGO_OUT_DIR})..."
        COMMAND cargo build ${CARGO_PROFILE_FLAG}
            -Zbuild-std=core,alloc
            -Zjson-target-spec
            --target ${TARGET_JSON}
            --features "${SGX_SYSROOT_FEATURES_CSV}"
        COMMAND ${CMAKE_COMMAND} -E rm -rf ${SGX_SYSROOT_OUTPUT}
        COMMAND ${CMAKE_COMMAND} -E make_directory ${SYSROOT_LIB_DIR}
        COMMAND ${CMAKE_COMMAND} -E copy_directory
            ${SDK_ROOT}/rustlib/std/target/${SGX_TARGET}/${CARGO_OUT_DIR}/deps
            ${SYSROOT_LIB_DIR}
        COMMAND ${CMAKE_COMMAND} -E touch ${SYSROOT_STAMP}
        WORKING_DIRECTORY ${SDK_ROOT}/rustlib/std
        VERBATIM
    )

    add_custom_target(sgx_sysroot
        DEPENDS ${SYSROOT_STAMP}
    )

    # Convenience: expose the sysroot path so consumers can reference it
    set(SGX_SYSROOT_PATH ${SGX_SYSROOT_OUTPUT} CACHE INTERNAL
        "Path to the installed SGX sysroot")
endif()

# =============================================================================
# Target: sgx_urts_rust — builds untrusted runtime
# =============================================================================
if(BUILD_URTS)
    message(STATUS "sgx_urts_rust")

    set(SGX_URTS_OUTPUT ${SDK_ROOT}/sgx_urts/target/release/libsgx_urts.a)
    add_custom_command(
        OUTPUT ${SGX_URTS_OUTPUT}
        COMMAND cargo build --release
        WORKING_DIRECTORY ${SDK_ROOT}/sgx_urts
    )

    add_custom_target(sgx_urts_target
        DEPENDS ${SGX_URTS_OUTPUT}
    )

    add_library(sgx_urts_rust STATIC IMPORTED GLOBAL)
    add_dependencies(sgx_urts_rust sgx_urts_target)

    set_target_properties(sgx_urts_rust
        PROPERTIES
        IMPORTED_LOCATION "${SGX_URTS_OUTPUT}"
    )

    # IMPORTED libraries cannot be installed, so we just copy the files
    install(FILES ${SGX_URTS_OUTPUT} DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif()
