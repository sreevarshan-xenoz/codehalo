import QtQuick
import "../theme"

Rectangle {
    id: haloRing
    property int diameter: 24
    property color glowColor: Colors.accentCyan
    property bool active: true

    width: diameter
    height: diameter
    radius: diameter / 2
    color: "transparent"
    border.color: glowColor
    border.width: 2

    // Fluid pulse rotation & scale animation
    SequentialAnimation on scale {
        running: haloRing.active
        loops: Animation.Infinite
        NumberAnimation { to: 1.12; duration: 1600; easing.type: Easing.InOutQuad }
        NumberAnimation { to: 1.0; duration: 1600; easing.type: Easing.InOutQuad }
    }

    SequentialAnimation on opacity {
        running: haloRing.active
        loops: Animation.Infinite
        NumberAnimation { to: 0.7; duration: 1600; easing.type: Easing.InOutQuad }
        NumberAnimation { to: 1.0; duration: 1600; easing.type: Easing.InOutQuad }
    }
}
