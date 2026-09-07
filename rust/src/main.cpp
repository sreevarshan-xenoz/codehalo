#include <QApplication>
#include <QQuickWindow>
#include <QQuickView>
#include <QQuickItem>
#include <QFile>
#include <QDir>
#include <QDebug>
#include <QScreen>
#include <QGuiApplication>

#ifdef Q_OS_WIN
#include <windows.h>
#endif

int main(int argc, char *argv[]) {
    // Must be called before QApplication for translucent background to work
    QQuickWindow::setDefaultAlphaBuffer(true);

    QApplication app(argc, argv);
    app.setOrganizationName("CodeHalo");
    app.setApplicationName("CodeHalo");

    // Locate QML — bundled in qrc
    QUrl qmlUrl(QStringLiteral("qrc:/CodeHalo/qml/Main.qml"));
    if (!QFile::exists(QStringLiteral(":/CodeHalo/qml/Main.qml"))) {
        // Fallback: local file (dev mode)
        QString localPath = QDir(QApplication::applicationDirPath()).filePath("../qml/Main.qml");
        if (!QFile::exists(localPath))
            localPath = QDir(QApplication::applicationDirPath()).filePath("qml/Main.qml");
        qmlUrl = QUrl::fromLocalFile(localPath);
        qDebug() << "[CodeHalo] Dev mode: loading from" << localPath;
    }

    qDebug() << "[CodeHalo] QML URL:" << qmlUrl;

    QQuickView view;
    view.setTitle(QStringLiteral("CodeHalo"));

    // ── TRUE TRANSPARENT WINDOW ───────────────────────────────────────────────
    // Qt::transparent means the WINDOW itself is fully transparent.
    // The QML content draws only the notch shape — everything else is see-through.
    view.setColor(QColor(Qt::transparent));

    // Window flags: frameless, always-on-top, no taskbar entry (Qt::Tool)
    // Qt::ToolTip avoids the window appearing in alt-tab and taskbar while
    // still getting a proper HWND with WS_EX_LAYERED compositing on Windows.
    view.setFlags(
        Qt::Window
        | Qt::FramelessWindowHint
        | Qt::WindowStaysOnTopHint
        | Qt::NoDropShadowWindowHint
        | Qt::WindowDoesNotAcceptFocus
        | Qt::Tool
    );

    view.setResizeMode(QQuickView::SizeViewToRootObject);

    // Log QML errors
    QObject::connect(&view, &QQuickView::statusChanged, [&view](QQuickView::Status status) {
        if (status == QQuickView::Error) {
            for (const auto &err : view.errors())
                qCritical() << "[CodeHalo QML Error]" << err.toString();
        }
    });

    view.setSource(qmlUrl);

    if (view.status() == QQuickView::Error) {
        for (const auto &err : view.errors())
            qCritical() << "[CodeHalo]" << err.toString();
        return 1;
    }

    // ── POSITION: FLUSH AT TOP EDGE, CENTERED HORIZONTALLY ───────────────────
    // codenotch sits at y=0 (top of screen). We do the same.
    // The pill's square-top + rounded-bottom visually merges with the screen edge.
    if (QScreen *screen = QGuiApplication::primaryScreen()) {
        QRect geom = screen->geometry(); // full screen rect, NOT visibleGeometry
        int pillWidth = view.rootObject() ? (int)view.rootObject()->width() : 240;
        int x = geom.x() + (geom.width() - pillWidth) / 2;
        int y = geom.y(); // y=0: flush at the very top edge
        view.setPosition(x, y);
    }

    view.show();
    view.raise();

#ifdef Q_OS_WIN
    HWND hwnd = (HWND)view.winId();

    // WS_EX_LAYERED: required for true alpha transparency via DWM
    // WS_EX_TRANSPARENT: mouse clicks pass through transparent regions
    // WS_EX_TOOLWINDOW: no taskbar entry
    // WS_EX_NOACTIVATE: does not steal focus
    LONG_PTR exStyle = GetWindowLongPtr(hwnd, GWL_EXSTYLE);
    exStyle |= WS_EX_LAYERED | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE;
    SetWindowLongPtr(hwnd, GWL_EXSTYLE, exStyle);

    // LWA_ALPHA = 255: fully opaque (DWM uses per-pixel alpha from the surface)
    // We set layered but let Qt's OpenGL surface supply the per-pixel alpha.
    SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA);

    // Pin to topmost z-order
    SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0,
                 SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW);

    // Update window dimensions from root object
    if (view.rootObject()) {
        int w = (int)view.rootObject()->width();
        int h = (int)view.rootObject()->height();
        if (QScreen *screen = QGuiApplication::primaryScreen()) {
            QRect geom = screen->geometry();
            int x = geom.x() + (geom.width() - w) / 2;
            SetWindowPos(hwnd, HWND_TOPMOST, x, geom.y(), w, h,
                         SWP_NOACTIVATE | SWP_SHOWWINDOW);
        }
    }
#endif

    qDebug() << "[CodeHalo] Running at" << view.position() << "size" << view.size();

    return app.exec();
}
