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

    // Pill size: wide enough for 4 rings + spacing + logo + label
    implicitWidth: pillRow.implicitWidth + 28
    implicitHeight: 36

    // Black, rounded only at the BOTTOM (top edge is flush with screen top)
    color: "#000000"
    radius: 18

    // Clip so the top corners stay square (flush with screen edge)
    layer.enabled: true
    layer.effect: null

    // Top edge clip: overlay a black rectangle over the top rounded corners
    Rectangle {
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: parent.radius
        color: "#000000"
        z: 1
    }

    // Subtle bottom glow when hovered
    Rectangle {
        anchors.fill: parent
        radius: parent.radius
        color: "transparent"
        border.color: isHovered ? "#22FFFFFF" : "transparent"
        border.width: 1
        z: 2
    }

    // Content row
    RowLayout {
        id: pillRow
        anchors.centerIn: parent
        anchors.verticalCenterOffset: 4  // Push down slightly since top is cut off visually
        spacing: 10
        z: 3

        // CodeHalo icon/logo — small
        Rectangle {
            width: 16
            height: 16
            radius: 4
            color: "#1A1A2E"
            border.color: "#38BDF8"
            border.width: 1
            Layout.alignment: Qt.AlignVCenter

            Image {
                anchors.fill: parent
                anchors.margins: 2
                source: "qrc:/CodeHalo/assets/icons/codehalo.png"
                fillMode: Image.PreserveAspectFit
                smooth: true
            }
        }

        // Provider usage rings
        ProviderRing {
            label: "C"
            ringColor: "#DA7756"  // Claude orange
            usedFraction: 0.35
            isActive: true
            Layout.alignment: Qt.AlignVCenter
        }

        ProviderRing {
            label: "G"
            ringColor: "#38BDF8"  // Gemini/AGY blue
            usedFraction: 0.72
            isActive: false
            Layout.alignment: Qt.AlignVCenter
        }

        ProviderRing {
            label: "X"
            ringColor: "#A78BFA"  // Codex purple
            usedFraction: 0.15
            isActive: false
            Layout.alignment: Qt.AlignVCenter
        }

        ProviderRing {
            label: "K"
            ringColor: "#34D399"  // Cursor green
            usedFraction: 0.58
            isActive: false
            Layout.alignment: Qt.AlignVCenter
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
            pill.hoverExit()
        }
        onClicked: pill.toggleExpand()
    }
}
