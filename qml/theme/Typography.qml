import QtQuick

QtObject {
    id: typography
    readonly property string fontFamily: "Segoe UI, Inter, sans-serif"
    readonly property string monospaceFamily: "Consolas, monospace"

    readonly property int fontTitle: 12
    readonly property int fontBody: 11
    readonly property int fontBadge: 9
    readonly property int fontLarge: 14
}
