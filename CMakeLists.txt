cmake_minimum_required(VERSION 3.22)

project(codehalo
    VERSION 0.1.0
    LANGUAGES CXX
    DESCRIPTION "CodeHalo: Your AI coding agents, at a glance."
)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Qt 6
find_package(Qt6 6.5 REQUIRED COMPONENTS
    Core
    Gui
    Qml
    Quick
    QuickControls2
)

qt_standard_project_setup()

# CXX-Qt integration
include(FetchContent)
FetchContent_Declare(
    cxx_qt
    GIT_REPOSITORY https://github.com/KDAB/cxx-qt.git
    GIT_TAG main
)
FetchContent_MakeAvailable(cxx_qt)

# Add QML module
qt_add_qml_module(codehalo_qml
    URI CodeHalo
    VERSION 1.0
    QML_FILES
        qml/Main.qml
        qml/theme/Colors.qml
        qml/theme/Typography.qml
        qml/components/Halo.qml
        qml/components/Hud.qml
    RESOURCES
        assets/icons/codehalo.png
)

# Rust bridge library
add_subdirectory(rust)

# Main target
qt_add_executable(codehalo
    rust/src/main.cpp
)

target_link_libraries(codehalo PRIVATE
    codehalo_rust
    codehalo_qml
    Qt6::Core
    Qt6::Gui
    Qt6::Qml
    Qt6::Quick
    Qt6::QuickControls2
)

set_target_properties(codehalo PROPERTIES
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)
