import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls
import "components"
import "theme"

Window {
    id: mainWindow
    visible: true
    title: "CodeHalo"

    // Milestone 1: Frameless, transparent, always-on-top overlay
    flags: Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint | Qt.Tool
    color: "transparent"

    width: 380
    height: isExpanded ? 240 : 46

    Behavior on height {
        NumberAnimation {
            duration: 220
            easing.type: Easing.OutCubic
        }
    }

    property bool isExpanded: false
    property string currentEdge: "top"

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 4
        spacing: 8

        // Collapsed HUD Pill
        Hud {
            Layout.alignment: Qt.AlignHCenter
            edgeTag: mainWindow.currentEdge.toUpperCase()
            onClicked: {
                mainWindow.isExpanded = !mainWindow.isExpanded
            }
        }

        // Expanded Card
        Rectangle {
            id: expandedDrawer
            visible: mainWindow.isExpanded
            opacity: mainWindow.isExpanded ? 1.0 : 0.0
            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: 12
            color: Colors.backgroundHud
            border.color: Colors.borderHud
            border.width: 1

            Behavior on opacity {
                NumberAnimation { duration: 180 }
            }

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 14
                spacing: 12

                // Header
                RowLayout {
                    Layout.fillWidth: true
                    Text {
                        text: "CODEHALO DISPLAY & EDGE"
                        font.family: Typography.fontFamily
                        font.pixelSize: Typography.fontBadge
                        font.bold: true
                        color: Colors.textMuted
                    }
                    Item { Layout.fillWidth: true }
                    Button {
                        text: "✕"
                        flat: true
                        onClicked: mainWindow.isExpanded = false
                    }
                }

                // Edge selector
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6

                    Repeater {
                        model: ["top", "bottom", "left", "right"]
                        Button {
                            id: edgeBtn
                            text: modelData.toUpperCase()
                            Layout.fillWidth: true
                            highlighted: mainWindow.currentEdge === modelData
                            onClicked: mainWindow.currentEdge = modelData
                        }
                    }
                }

                // Status info
                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    radius: 8
                    color: Colors.surfaceInput

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 10
                        spacing: 6

                        RowLayout {
                            Layout.fillWidth: true
                            Text { text: "Render Engine"; color: Colors.textMuted; font.pixelSize: Typography.fontBody }
                            Item { Layout.fillWidth: true }
                            Text { text: "Qt 6 Quick / QML (GPU Native)"; color: Colors.accentCyan; font.pixelSize: Typography.fontBody }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Text { text: "Window State"; color: Colors.textMuted; font.pixelSize: Typography.fontBody }
                            Item { Layout.fillWidth: true }
                            Text { text: "Frameless / Always-On-Top"; color: Colors.accentGreen; font.pixelSize: Typography.fontBody }
                        }
                    }
                }
            }
        }
    }
}
