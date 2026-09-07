#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickWindow>
#include <QColor>

int main(int argc, char *argv[]) {
    // Enable software/GPU acceleration & high-DPI scaling
    QGuiApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QGuiApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    QGuiApplication app(argc, argv);
    app.setOrganizationName("CodeHalo");
    app.setApplicationName("CodeHalo");

    QQmlApplicationEngine engine;

    // Load main QML interface
    const QUrl url(QStringLiteral("qrc:/CodeHalo/qml/Main.qml"));
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreated,
        &app,
        [url](QObject *obj, const QUrl &objUrl) {
            if (!obj && url == objUrl)
                QCoreApplication::exit(-1);
        },
        Qt::QueuedConnection
    );

    engine.load(url);

    return app.exec();
}
