cmake_minimum_required(VERSION 3.28)

project(rhesis
    VERSION 1.0.0
)

# --- Options ---
set(RHESIS_BINARY_NAME "rhesis" CACHE STRING "Name of the executable")

# --- Dependencies ---
find_package(ECM 6.0 REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})
include(KDEInstallDirs)
include(ECMUninstallTarget)

include(ECMFindQmlModule)
find_package(Qt6 REQUIRED COMPONENTS Gui)
find_package(Qt6 COMPONENTS QmlTools QUIET)

# --- Build the Rust binary ---
add_custom_target(rhesis_build ALL
    COMMAND cargo build --target-dir ${CMAKE_CURRENT_BINARY_DIR}
              $<IF:$<CONFIG:Debug>,,--release>
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building rhesis with cargo"
)

# Determine binary output location
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RHESIS_BINARY_PATH "${CMAKE_CURRENT_BINARY_DIR}/debug/${RHESIS_BINARY_NAME}${CMAKE_EXECUTABLE_SUFFIX}")
else()
    set(RHESIS_BINARY_PATH "${CMAKE_CURRENT_BINARY_DIR}/release/${RHESIS_BINARY_NAME}${CMAKE_EXECUTABLE_SUFFIX}")
endif()

# --- Install rules ---
install(
    PROGRAMS ${RHESIS_BINARY_PATH}
    DESTINATION ${KDE_INSTALL_BINDIR}
    COMPONENT Runtime
)

install(
    FILES ${CMAKE_CURRENT_SOURCE_DIR}/org.dimkar.rhesis.desktop
    DESTINATION ${KDE_INSTALL_APPDIR}
    COMPONENT Runtime
)

# --- Windows deployment ---
if(WIN32)
    # Find windeployqt
    get_target_property(QT_PATH Qt6::qmake IMPORTED_LOCATION)
    get_filename_component(QT_BIN_DIR "${QT_PATH}" DIRECTORY)
    find_program(WINDEPLOYQT
        NAMES windeployqt
        HINTS "${QT_BIN_DIR}"
        REQUIRED
    )

    # Deploy Qt and KDE dependencies alongside the binary
    add_custom_target(rhesis_deploy ALL
        COMMAND ${CMAKE_COMMAND} -E echo "Deploying Qt and KDE dependencies..."
        COMMAND ${WINDEPLOYQT} --no-translations --no-system-d3d-compiler
                --plugindir runtime/plugins
                --qmldir runtime/qml
                --libdir runtime
                $<TARGET_FILE_DIR:Qt6::QGuiApplication>
                ${RHESIS_BINARY_PATH}
        COMMAND ${CMAKE_COMMAND} -E copy
                ${RHESIS_BINARY_PATH}
                ${CMAKE_CURRENT_BINARY_DIR}/runtime/
        COMMENT "Running windeployqt"
    )

    # KDE deployment (optional, requires craft or craft-bin tools)
    find_program(KDEDEPLOY NAMES kde-deploy-windows QUIET)
    if(KDEDEPLOY)
        add_custom_command(TARGET rhesis_deploy POST_BUILD
            COMMAND ${KDEDEPLOY} --install-dir ${CMAKE_CURRENT_BINARY_DIR}/runtime
                    ${RHESIS_BINARY_PATH}
            COMMENT "Deploying KDE frameworks"
        )
    endif()

    # Install the deployed runtime
    install(
        DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/runtime/
        DESTINATION ${CMAKE_INSTALL_PREFIX}
        COMPONENT Runtime
    )

    # --- CPack (NSIS installer) ---
    set(CPACK_PACKAGE_NAME "Rhesis")
    set(CPACK_PACKAGE_VERSION "${PROJECT_VERSION}")
    set(CPACK_PACKAGE_VENDOR "dimkar")
    set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "Rhesis - LanguageTool client")
    set(CPACK_PACKAGE_EXECUTABLES "${RHESIS_BINARY_NAME}" "Rhesis")
    set(CPACK_CREATE_DESKTOP_LINKS "${RHESIS_BINARY_NAME}")
    set(CPACK_NSIS_DISPLAY_NAME "Rhesis")
    set(CPACK_NSIS_INSTALL_ROOT "$PROGRAMFILES64")
    set(CPACK_NSIS_CREATE_ICONS_EXTRA
        "CreateShortCut '$SMPROGRAMS\\\\$STARTMENU_FOLDER\\\\Rhesis.lnk' '$INSTDIR\\\\bin\\\\${RHESIS_BINARY_NAME}${CMAKE_EXECUTABLE_SUFFIX}'"
    )
    set(CPACK_NSIS_DELETE_ICONS_EXTRA
        "Delete '$SMPROGRAMS\\\\$START_MENU\\\\Rhesis.lnk'"
    )

    set(CPACK_COMPONENTS_ALL Runtime)
    set(CPACK_ALL_INSTALL_TYPES Full)

    include(CPack)
endif()
