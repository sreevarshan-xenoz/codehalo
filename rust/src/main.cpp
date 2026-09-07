#include <QApplication>
#include <QQuickWindow>
#include <QQuickView>
#include <QQuickItem>
#include <QFile>
#include <QDir>
#include <QDebug>
#include <QScreen>
#include <QGuiApplication>
#include <QSurfaceFormat>
#include <QTimer>

#ifdef Q_OS_WIN
#include <windows.h>
#include <shellscalingapi.h>
#pragma comment(lib, "Shcore.lib")
#endif

int main(int argc, char *argv[]) {
#ifdef Q_OS_WIN
    // Per-monitor DPI awareness BEFORE Qt init — prevents Windows from
    // applying DPI virtualization (which renders at lower res and scales up,
    // causing the "everything looks pixelated" problem).
    SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
#endif

    // High-DPI: Use fractional pass-through so 125%, 150%, 175% scale crisp without rounding
    QGuiApplication::setHighDpiScaleFactorRoundingPolicy(Qt::HighDpiScaleFactorRoundingPolicy::PassThrough);

    // Multisampling (8x MSAA) + alpha buffer for smooth curves, rings, and rounded corners
    QSurfaceFormat format;
    format.setAlphaBufferSize(8);
    format.setSamples(8);
    QSurfaceFormat::setDefaultFormat(format);

    // Must be called before QApplication for translucent background to work
    QQuickWindow::setDefaultAlphaBuffer(true);

    QApplication app(argc, argv);
    app.setOrganizationName("CodeHalo");
    app.setApplicationName("CodeHalo");
    app.setQuitOnLastWindowClosed(false);

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
    auto recenterWindow = [&view]() {
        if (QScreen *s = view.screen() ? view.screen() : QGuiApplication::primaryScreen()) {
            QRect geom = s->geometry();
            int w = view.width();
            int x = geom.x() + (geom.width() - w) / 2;
            int y = geom.y();
            view.setPosition(x, y);
        }
    };

    QObject::connect(&view, &QQuickWindow::widthChanged, recenterWindow);
    recenterWindow();

    view.show();
    view.raise();

#ifdef Q_OS_WIN
    HWND hwnd = (HWND)view.winId();

    // Preserve tool window and non-activating styles while letting Qt's
    // hardware composition handle transparent surface presentation natively.
    LONG_PTR exStyle = GetWindowLongPtr(hwnd, GWL_EXSTYLE);
    exStyle |= WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE;
    SetWindowLongPtr(hwnd, GWL_EXSTYLE, exStyle);

    // Ensure topmost z-order without interfering with geometry managed by Qt
    SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0,
                 SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW);
#endif

    QScreen *curScreen = view.screen() ? view.screen() : QGuiApplication::primaryScreen();
    qDebug() << "[CodeHalo] Running at" << view.position() << "size" << view.size()
             << "DPR" << view.devicePixelRatio()
             << "screen" << (curScreen ? curScreen->name() : "none")
             << "screenDPI" << (curScreen ? curScreen->logicalDotsPerInch() : 0);

    if (app.arguments().contains(QStringLiteral("--snapshot"))) {
        QTimer::singleShot(600, [&view, &app]() {
            QImage img = view.grabWindow();
            img.save(QStringLiteral("E:/codehalo/build/notch_snapshot.png"));
            qDebug() << "[CodeHalo] Saved snapshot to E:/codehalo/build/notch_snapshot.png size:" << img.size();
            app.quit();
        });
    }

    return app.exec();
}
