#include <QApplication>
#include <QQmlApplicationEngine>
#include <QQuickWindow>
#include <QQuickWidget>
#include <QLabel>
#include <QFile>
#include <QDir>
#include <QDebug>

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    app.setOrganizationName("CodeHalo");
    app.setApplicationName("CodeHalo");

    QQmlApplicationEngine engine;

    // Locate QML file
    QUrl qmlUrl(QStringLiteral("qrc:/CodeHalo/qml/Main.qml"));
    if (!QFile::exists(QStringLiteral(":/CodeHalo/qml/Main.qml"))) {
        QString localPath = QDir::current().filePath(QStringLiteral("../qml/Main.qml"));
        if (!QFile::exists(localPath)) {
            localPath = QDir::current().filePath(QStringLiteral("qml/Main.qml"));
        }
        qmlUrl = QUrl::fromLocalFile(localPath);
    }

    qDebug() << "[CodeHalo] Loading QML engine from:" << qmlUrl;

    QObject::connect(&engine, &QQmlApplicationEngine::objectCreationFailed,
                     &app, []() { QCoreApplication::exit(-1); },
                     Qt::QueuedConnection);

    // Create top-level overlay window using QWidget with native Win32 window manager integration
    QWidget overlay;
    overlay.setWindowTitle(QStringLiteral("CodeHalo"));
    overlay.setWindowFlags(Qt::Window | Qt::FramelessWindowHint | Qt::WindowStaysOnTopHint);
    overlay.setAttribute(Qt::WA_TranslucentBackground, true);

    QQuickWidget *quickWidget = new QQuickWidget(&overlay);
    quickWidget->setClearColor(Qt::transparent);
    quickWidget->setResizeMode(QQuickWidget::SizeRootObjectToView);
    quickWidget->setSource(qmlUrl);

    int initialWidth = 380;
    int initialHeight = 54;
    overlay.resize(initialWidth, initialHeight);
    quickWidget->resize(initialWidth, initialHeight);

    if (QScreen *screen = QApplication::primaryScreen()) {
        QRect geom = screen->availableGeometry();
        int x = geom.x() + (geom.width() - initialWidth) / 2;
        int y = geom.y() + 16;
        overlay.move(x, y);
    }

    overlay.show();
    overlay.raise();

    qDebug() << "[CodeHalo] Native Overlay active at:" << overlay.pos() 
             << "Size:" << overlay.size()
             << "WinId:" << (void*)overlay.winId();

    return app.exec();
}
