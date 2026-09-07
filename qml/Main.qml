import QtQuick
import QtQuick.Layouts
import "components"

// Root is TRANSPARENT — only the notch shape is visible
Item {
    id: root

    // Collapsed: just the pill height. Expanded: pill + card below
    property bool isExpanded: false
    property bool isHovered: false

    width: notchPill.width
    height: isExpanded ? notchPill.height + expandedCard.height + 6 : notchPill.height

    Behavior on height {
        NumberAnimation { duration: 200; easing.type: Easing.OutCubic }
    }
    Behavior on width {
        NumberAnimation { duration: 200; easing.type: Easing.OutCubic }
    }

    // ── Collapsed Notch Pill ──────────────────────────────────────────────────
    NotchPill {
        id: notchPill
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        isExpanded: root.isExpanded
        isHovered: root.isHovered
        onHoverEnter: root.isHovered = true
        onHoverExit: root.isHovered = false
        onToggleExpand: root.isExpanded = !root.isExpanded
    }

    // ── Expanded Card ─────────────────────────────────────────────────────────
    ExpandedCard {
        id: expandedCard
        anchors.top: notchPill.bottom
        anchors.topMargin: 6
        anchors.horizontalCenter: parent.horizontalCenter
        visible: root.isExpanded
        opacity: root.isExpanded ? 1.0 : 0.0

        Behavior on opacity {
            NumberAnimation { duration: 180; easing.type: Easing.OutCubic }
        }

        onClose: root.isExpanded = false
    }
}
