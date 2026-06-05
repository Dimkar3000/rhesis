cmake_minimum_required(VERSION 3.28)

project(rhesis)

find_package(ECM 6.0 REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})
include(KDEInstallDirs)
include(ECMUninstallTarget)

include(ECMFindQmlModule)
find_package(KF6 REQUIRED COMPONENTS QQC2DesktopStyle)
find_package(Qt6 REQUIRED COMPONENTS Gui)


add_custom_target(kirigami_rust
    ALL
    COMMAND cargo build --target-dir ${CMAKE_CURRENT_BINARY_DIR}
)

install(
    PROGRAMS ${CMAKE_CURRENT_BINARY_DIR}/debug/rhesis
    DESTINATION ${KDE_INSTALL_BINDIR}
)

install(FILES org.kde.rhesis.desktop DESTINATION ${KDE_INSTALL_APPDIR})

