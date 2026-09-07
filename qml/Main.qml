import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls
import "components"

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
            color: "#DE0E1117"
            border.color: "#1FFFFFFF"
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
                        font.family: "Segoe UI, Inter, sans-serif"
                        font.pixelSize: 9
                        font.bold: true
                        color: "#94A3B8"
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
                    color: "#10FFFFFF"

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 10
                        spacing: 6

                        RowLayout {
                            Layout.fillWidth: true
                            Text { text: "Render Engine"; color: "#94A3B8"; font.pixelSize: 11 }
                            Item { Layout.fillWidth: true }
                            Text { text: "Qt 6 Quick / QML (GPU Native)"; color: "#38BDF8"; font.pixelSize: 11 }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Text { text: "Window State"; color: "#94A3B8"; font.pixelSize: 11 }
                            Item { Layout.fillWidth: true }
                            Text { text: "Frameless / Always-On-Top"; color: "#10B981"; font.pixelSize: 11 }
                        }
                    }
                }
            }
        }
    }
}
