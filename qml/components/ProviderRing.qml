import QtQuick
import QtQuick.Shapes

// Circular usage ring — grey track + colored arc, crisp glyph letter in center
Rectangle {
    id: ring

    property string label: "C"
    property color ringColor: "#DA7756"
    property real usedFraction: 0.0       // 0.0 to 1.0
    property bool isActive: false         // spinning arc when true

    implicitWidth: 30
    implicitHeight: 30
    color: "transparent"
    antialiasing: true

    // Spinning state for active indicator
    property real spinAngle: 0
    NumberAnimation on spinAngle {
        from: 0; to: 360
        duration: 1200
        loops: Animation.Infinite
        running: ring.isActive
    }

    // Grey track ring
    Shape {
        anchors.fill: parent
        antialiasing: true
        preferredRendererType: Shape.CurveRenderer

        ShapePath {
            strokeColor: "#333333"
            strokeWidth: 2.8
            fillColor: "transparent"
            capStyle: ShapePath.FlatCap
            PathAngleArc {
                centerX: ring.width / 2
                centerY: ring.height / 2
                radiusX: ring.width / 2 - 1.5
                radiusY: ring.height / 2 - 1.5
                startAngle: 0
                sweepAngle: 360
            }
        }
    }

    // Usage arc (colored, starts at 12 o'clock = -90°)
    Shape {
        anchors.fill: parent
        antialiasing: true
        preferredRendererType: Shape.CurveRenderer
        visible: usedFraction > 0

        ShapePath {
            strokeColor: ring.ringColor
            strokeWidth: 2.8
            fillColor: "transparent"
            capStyle: ShapePath.RoundCap
            PathAngleArc {
                centerX: ring.width / 2
                centerY: ring.height / 2
                radiusX: ring.width / 2 - 1.5
                radiusY: ring.height / 2 - 1.5
                startAngle: -90
                sweepAngle: ring.usedFraction * 360
            }
        }
    }

    // Active session spinner — thin neutral arc inside the track
    Shape {
        anchors.fill: parent
        anchors.margins: 5
        antialiasing: true
        preferredRendererType: Shape.CurveRenderer
        visible: ring.isActive

        ShapePath {
            strokeColor: "#DDDDDD"
            strokeWidth: 1.5
            fillColor: "transparent"
            capStyle: ShapePath.RoundCap
            PathAngleArc {
                centerX: (ring.width - 10) / 2
                centerY: (ring.height - 10) / 2
                radiusX: (ring.width - 10) / 2 - 1
                radiusY: (ring.height - 10) / 2 - 1
                startAngle: ring.spinAngle
                sweepAngle: 90
            }
        }
    }

    // Glyph letter in center
    Text {
        anchors.centerIn: parent
        text: ring.label
        font.pixelSize: 10
        font.bold: true
        font.family: "Segoe UI, Inter, sans-serif"
        color: "#E2E8F0"
        renderType: Text.NativeRendering
    }
}
