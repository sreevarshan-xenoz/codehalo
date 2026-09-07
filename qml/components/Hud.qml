import QtQuick
import QtQuick.Layouts
import "../theme"

Rectangle {
    id: hudPill
    signal clicked()

    property alias title: pillTitle.text
    property alias statusText: pillStatus.text
    property alias edgeTag: edgeLabel.text

    width: pillRow.implicitWidth + 24
    height: 38
    radius: 19
    color: mouseArea.containsMouse ? Colors.backgroundHover : Colors.backgroundHud
    border.color: mouseArea.containsMouse ? Colors.accentCyan : Colors.borderHud
    border.width: 1

    Behavior on color { ColorAnimation { duration: 150 } }
    Behavior on border.color { ColorAnimation { duration: 150 } }

    RowLayout {
        id: pillRow
        anchors.centerIn: parent
        spacing: 8

        Image {
            source: "qrc:/assets/icons/codehalo.png"
            sourceSize.width: 18
            sourceSize.height: 18
            Layout.alignment: Qt.AlignVCenter
            smooth: true
        }

        Text {
            id: pillTitle
            text: "CodeHalo"
            font.family: Typography.fontFamily
            font.pixelSize: Typography.fontTitle
            font.bold: true
            color: Colors.textMain
        }

        Rectangle {
            width: 6
            height: 6
            radius: 3
            color: Colors.accentGreen
            Layout.alignment: Qt.AlignVCenter
        }

        Text {
            id: pillStatus
            text: "Ready"
            font.family: Typography.fontFamily
            font.pixelSize: Typography.fontBody
            color: Colors.textMuted
        }

        Rectangle {
            radius: 3
            color: "#2538BDF8"
            implicitWidth: edgeLabel.implicitWidth + 8
            implicitHeight: edgeLabel.implicitHeight + 2

            Text {
                id: edgeLabel
                anchors.centerIn: parent
                text: "TOP"
                font.family: Typography.fontFamily
                font.pixelSize: Typography.fontBadge
                font.bold: true
                color: Colors.accentCyan
            }
        }
    }

    MouseArea {
        id: mouseArea
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: hudPill.clicked()
    }
}
