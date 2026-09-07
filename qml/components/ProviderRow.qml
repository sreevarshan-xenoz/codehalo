import QtQuick
import QtQuick.Layouts

// A single provider row in the expanded card — ring + name + status
RowLayout {
    id: row

    property string providerName: "Claude Code"
    property color ringColor: "#DA7756"
    property real usedFraction: 0.0
    property string statusText: ""
    property bool isActive: false
    property bool isBlocked: false

    Layout.fillWidth: true
    spacing: 12

    // Ring
    ProviderRing {
        width: 32
        height: 32
        label: providerName.substring(0, 1)
        ringColor: row.ringColor
        usedFraction: row.usedFraction
        isActive: row.isActive
        Layout.alignment: Qt.AlignVCenter
    }

    // Text info
    ColumnLayout {
        Layout.fillWidth: true
        spacing: 2

        Text {
            text: providerName
            font.pixelSize: 12
            font.bold: true
            font.family: "Segoe UI, Inter, sans-serif"
            color: "#EEEEEE"
        }

        Text {
            text: statusText
            font.pixelSize: 10
            font.family: "Segoe UI, Inter, sans-serif"
            color: row.isBlocked ? "#F59E0B" : "#666666"
        }
    }

    // Percentage
    Text {
        text: Math.round(usedFraction * 100) + "%"
        font.pixelSize: 11
        font.bold: true
        font.family: "Segoe UI Mono, monospace"
        color: usedFraction > 0.8 ? "#EF4444" : (usedFraction > 0.5 ? "#F59E0B" : "#888888")
        Layout.alignment: Qt.AlignVCenter
    }
}
