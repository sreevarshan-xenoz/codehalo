import QtQuick
import QtQuick.Layouts
import "components"

// Root is TRANSPARENT — only the notch shape is visible
Item {
    id: root

    // Collapsed: just the pill height. Expanded: pill + card below
    property bool isExpanded: false
    property bool isHovered: false

    width: isExpanded ? Math.max(notchPill.width, expandedCard.implicitWidth) : notchPill.width
    height: isExpanded ? notchPill.height + expandedCard.height + 6 : notchPill.height

    // Dynamic-Island-style spring: snappy ~300ms with a hint of overshoot.
    // Same physics both ways so collapse feels natural too (no OutBack slam).
    Behavior on height {
        SpringAnimation { spring: 5.0; damping: 0.5; mass: 0.7; epsilon: 0.01 }
    }
    Behavior on width {
        SpringAnimation { spring: 5.0; damping: 0.5; mass: 0.7; epsilon: 0.01 }
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
    // Drops below the pill with fade + rise + settle-scale.
    // visible tracks opacity (not isExpanded) so the exit fade actually plays.
    ExpandedCard {
        id: expandedCard
        anchors.top: notchPill.bottom
        anchors.topMargin: root.isExpanded ? 6 : -8
        anchors.horizontalCenter: parent.horizontalCenter
        transformOrigin: Item.Top
        scale: root.isExpanded ? 1.0 : 0.95
        opacity: root.isExpanded ? 1.0 : 0.0
        visible: opacity > 0.01

        Behavior on opacity {
            NumberAnimation { duration: 240; easing.type: Easing.OutCubic }
        }
        Behavior on scale {
            NumberAnimation { duration: 300; easing.type: Easing.OutBack; easing.overshoot: 1.4 }
        }
        Behavior on anchors.topMargin {
            NumberAnimation { duration: 280; easing.type: Easing.OutCubic }
        }

        onClose: root.isExpanded = false
    }
}
