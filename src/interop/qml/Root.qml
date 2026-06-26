import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami
import org.dimkar.rhesis

Kirigami.ApplicationWindow {
    id: root

    width: 400
    height: 300

    title: "Hello Linux"

    property alias secondPage: secondPage

    pageStack.globalToolBar.showNavigationButtons: Kirigami.ApplicationHeaderStyle.NoNavigationButtons
    pageStack.columnView.scrollDuration: 0
    pageStack.initialPage: mainPage

    MainPage {
        id: mainPage
        highlighter: highlighter
        helper: helper
    }

    SecondPage {
        id: secondPage
        visible: false
    }


    CustomHighlighter {
        id: highlighter
        Component.onCompleted: highlighter.startMessageThread(helper)
    }

    AsyncHelper {
        id: helper
        Component.onCompleted: helper.start_async_worker()
    }
}