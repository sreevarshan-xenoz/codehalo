import QtQuick
import QtQuick.Layouts

Rectangle {
    id: hudPill
    signal clicked()

    property alias title: pillTitle.text
    property alias statusText: pillStatus.text
    property alias edgeTag: edgeLabel.text

    width: pillRow.implicitWidth + 24
    height: 38
    radius: 19
    color: mouseArea.containsMouse ? "#F5161B22" : "#DE0E1117"
    border.color: mouseArea.containsMouse ? "#38BDF8" : "#1FFFFFFF"
    border.width: 1

    Behavior on color { ColorAnimation { duration: 150 } }
    Behavior on border.color { ColorAnimation { duration: 150 } }

    RowLayout {
        id: pillRow
        anchors.centerIn: parent
        spacing: 8

        Image {
            source: "qrc:/CodeHalo/assets/icons/codehalo.png"
            sourceSize.width: 18
            sourceSize.height: 18
            Layout.alignment: Qt.AlignVCenter
            smooth: true
        }

        Text {
            id: pillTitle
            text: "CodeHalo"
            font.family: "Segoe UI, Inter, sans-serif"
            font.pixelSize: 12
            font.bold: true
            color: "#F1F5F9"
        }

        Rectangle {
            width: 6
            height: 6
            radius: 3
            color: "#10B981"
            Layout.alignment: Qt.AlignVCenter
        }

        Text {
            id: pillStatus
            text: "Ready"
            font.family: "Segoe UI, Inter, sans-serif"
            font.pixelSize: 11
            color: "#94A3B8"
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
                font.family: "Segoe UI, Inter, sans-serif"
                font.pixelSize: 9
                font.bold: true
                color: "#38BDF8"
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
