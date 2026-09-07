import QtQuick
import QtQuick.Layouts

// The collapsed notch pill — black, rounded bottom corners, flush at top
Rectangle {
    id: pill

    signal hoverEnter()
    signal hoverExit()
    signal toggleExpand()

    property bool isExpanded: false
    property bool isHovered: false
    property bool isPressed: false

    antialiasing: true

    // Tactile press squash; state scale is orchestrated by Main.qml
    transformOrigin: Item.Top
    scale: isPressed ? 0.97 : 1.0

    // Pill size: comfortable height + clear spacing
    implicitWidth: pillRow.implicitWidth + 32
    implicitHeight: 40

    // Black, rounded only at the BOTTOM (top edge is flush with screen top)
    color: "#000000"
    radius: 20

    // Top edge clip: overlay a black rectangle over the top rounded corners
    Rectangle {
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: parent.radius
        color: "#000000"
        z: 1
    }

    // Subtle bottom & side glow when hovered (top is flush outside)
    Rectangle {
        anchors.top: parent.top
        anchors.topMargin: -2
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        radius: parent.radius
        color: "transparent"
        border.color: isHovered ? "#22FFFFFF" : "transparent"
        border.width: 1
        antialiasing: true
        z: 2

        Behavior on border.color {
            ColorAnimation { duration: 160; easing.type: Easing.OutCubic }
        }
    }

    property var providers: []

    // Content row
    RowLayout {
        id: pillRow
        anchors.centerIn: parent
        anchors.verticalCenterOffset: 2
        spacing: 12
        z: 3

        // CodeHalo icon/logo — crisp and clear
        Rectangle {
            width: 22
            height: 22
            radius: 5
            color: "#1A1A2E"
            border.color: "#38BDF8"
            border.width: 1
            antialiasing: true
            Layout.alignment: Qt.AlignVCenter

            Image {
                anchors.fill: parent
                anchors.margins: 3
                source: "qrc:/CodeHalo/assets/icons/codehalo.png"
                sourceSize.width: 48
                sourceSize.height: 48
                fillMode: Image.PreserveAspectFit
                smooth: true
                mipmap: true
            }
        }

        // Provider usage rings rendered dynamically from model
        Repeater {
            model: pill.providers
            ProviderRing {
                label: modelData.label
                ringColor: modelData.color
                usedFraction: modelData.usedFraction
                isActive: modelData.isActive
                Layout.alignment: Qt.AlignVCenter
            }
        }
    }

    // Hover + click handling
    MouseArea {
        id: hoverArea
        anchors.fill: parent
        hoverEnabled: true
        z: 4

        onEntered: {
            pill.isHovered = true
            pill.hoverEnter()
        }
        onExited: {
            pill.isHovered = false
            pill.isPressed = false
            pill.hoverExit()
        }
        onPressed: pill.isPressed = true
        onReleased: pill.isPressed = false
        onClicked: pill.toggleExpand()
    }
}
