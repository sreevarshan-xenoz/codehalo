import QtQuick
import QtQuick.Layouts
import "components"

// Root is TRANSPARENT — only the notch shape and expanded card are visible
Item {
    id: root

    // Centralized provider state (ready to be bound to CXX-Qt / Rust ProviderState)
    property var providers: [
        { name: "Claude Code",  label: "C", color: "#DA7756", usedFraction: 0.35, status: "3 sessions active", isActive: true,  isBlocked: false },
        { name: "Gemini / AGY", label: "G", color: "#38BDF8", usedFraction: 0.72, status: "72% used · resets 2h", isActive: false, isBlocked: false },
        { name: "Codex",        label: "X", color: "#A78BFA", usedFraction: 0.15, status: "Plenty remaining",   isActive: false, isBlocked: false },
        { name: "Cursor",       label: "K", color: "#34D399", usedFraction: 0.58, status: "Waiting on you",     isActive: false, isBlocked: true  }
    ]

    property bool isExpanded: false
    property bool isHovered: false

    width: notchPill.implicitWidth
    height: notchPill.implicitHeight

    // ── STATE MACHINE ────────────────────────────────────────────────────────
    state: isExpanded ? "EXPANDED" : (isHovered ? "HOVER" : "IDLE")

    states: [
        State {
            name: "IDLE"
            PropertyChanges { target: notchPill; scale: 1.0 }
            PropertyChanges { target: root; width: notchPill.implicitWidth; height: notchPill.implicitHeight }
            PropertyChanges { target: expandedCard; opacity: 0.0; scale: 0.96; anchors.topMargin: -8; visible: false }
        },
        State {
            name: "HOVER"
            PropertyChanges { target: notchPill; scale: 1.02 }
            PropertyChanges { target: root; width: notchPill.implicitWidth; height: notchPill.implicitHeight }
            PropertyChanges { target: expandedCard; opacity: 0.0; scale: 0.96; anchors.topMargin: -8; visible: false }
        },
        State {
            name: "EXPANDED"
            PropertyChanges { target: notchPill; scale: 1.0 }
            PropertyChanges { target: root; width: Math.max(notchPill.implicitWidth, expandedCard.implicitWidth); height: notchPill.implicitHeight + expandedCard.implicitHeight + 10 }
            PropertyChanges { target: expandedCard; opacity: 1.0; scale: 1.0; anchors.topMargin: 6; visible: true }
        }
    ]

    // ── TRANSITION CHOREOGRAPHY ──────────────────────────────────────────────
    transitions: [
        // 1. Idle <-> Hover: subtle, snappy (~140ms)
        Transition {
            from: "IDLE"; to: "HOVER"
            NumberAnimation { target: notchPill; property: "scale"; duration: 140; easing.type: Easing.OutQuad }
        },
        Transition {
            from: "HOVER"; to: "IDLE"
            NumberAnimation { target: notchPill; property: "scale"; duration: 140; easing.type: Easing.OutQuad }
        },

        // 2. Expansion: Single choreographed morph (~280-320ms)
        // Container geometry leads with smooth quart ease; card smoothly settles with gentle 0.7 overshoot
        Transition {
            to: "EXPANDED"
            ParallelAnimation {
                NumberAnimation {
                    target: root
                    properties: "width,height"
                    duration: 280
                    easing.type: Easing.OutQuart
                }
                SequentialAnimation {
                    PauseAnimation { duration: 35 }
                    ParallelAnimation {
                        NumberAnimation {
                            target: expandedCard
                            property: "opacity"
                            duration: 220
                            easing.type: Easing.OutCubic
                        }
                        NumberAnimation {
                            target: expandedCard
                            property: "anchors.topMargin"
                            duration: 250
                            easing.type: Easing.OutCubic
                        }
                        NumberAnimation {
                            target: expandedCard
                            property: "scale"
                            duration: 280
                            easing.type: Easing.OutBack
                            easing.overshoot: 0.7
                        }
                    }
                }
            }
        },

        // 3. Collapse: Smooth, natural contraction (~220-250ms)
        // Card content fades out quickly while container smoothly contracts
        Transition {
            from: "EXPANDED"
            ParallelAnimation {
                NumberAnimation {
                    target: expandedCard
                    property: "opacity"
                    duration: 150
                    easing.type: Easing.InQuad
                }
                NumberAnimation {
                    target: expandedCard
                    property: "scale"
                    duration: 180
                    easing.type: Easing.InCubic
                }
                NumberAnimation {
                    target: expandedCard
                    property: "anchors.topMargin"
                    duration: 180
                    easing.type: Easing.InCubic
                }
                NumberAnimation {
                    target: root
                    properties: "width,height"
                    duration: 240
                    easing.type: Easing.InOutQuad
                }
            }
        }
    ]

    // ── Collapsed Notch Pill ──────────────────────────────────────────────────
    NotchPill {
        id: notchPill
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        providers: root.providers
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
        anchors.horizontalCenter: parent.horizontalCenter
        transformOrigin: Item.Top
        providers: root.providers
        onClose: root.isExpanded = false
    }
}
