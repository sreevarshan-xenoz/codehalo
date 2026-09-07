import QtQuick
import QtQuick.Layouts

// Expanded card that drops below the notch pill on click
Rectangle {
    id: card

    signal close()

    implicitWidth: 300
    implicitHeight: contentCol.implicitHeight + 24

    color: "#0D0D0D"
    radius: 14
    border.color: "#1AFFFFFF"
    border.width: 1
    antialiasing: true

    // Subtle top-left/right corner square off (matches the notch pill bottom)
    Rectangle {
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: parent.radius
        color: "#0D0D0D"
        z: 1
    }

    property var providers: []

    ColumnLayout {
        id: contentCol
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: 14
        spacing: 10
        z: 2

        // Header
        RowLayout {
            Layout.fillWidth: true

            Text {
                text: "CODEHALO"
                font.pixelSize: 9
                font.bold: true
                font.letterSpacing: 1.5
                font.family: "Segoe UI, Inter, sans-serif"
                color: "#666666"
                renderType: Text.NativeRendering
            }

            Item { Layout.fillWidth: true }

            Text {
                text: "✕"
                font.pixelSize: 11
                color: "#555555"
                renderType: Text.NativeRendering
                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: card.close()
                }
            }
        }

        // Provider rows rendered dynamically from model
        Repeater {
            model: card.providers
            ProviderRow {
                providerName: modelData.name
                label: modelData.label
                ringColor: modelData.color
                usedFraction: modelData.usedFraction
                statusText: modelData.status
                isActive: modelData.isActive
                isBlocked: modelData.isBlocked
            }
        }

        // Divider
        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: "#1AFFFFFF"
        }

        // Footer status
        RowLayout {
            Layout.fillWidth: true
            Text {
                text: "Refreshed just now"
                font.pixelSize: 10
                color: "#444444"
                font.family: "Segoe UI, Inter, sans-serif"
                renderType: Text.NativeRendering
            }
            Item { Layout.fillWidth: true }
            Text {
                text: "Settings"
                font.pixelSize: 10
                color: "#38BDF8"
                font.family: "Segoe UI, Inter, sans-serif"
                renderType: Text.NativeRendering
                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                }
            }
        }
    }
}
