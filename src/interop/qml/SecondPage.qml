import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami
import org.dimkar.rhesis

Kirigami.Page {
    id: secondPage

    actions: [
        Kirigami.Action {
            icon.name: "go-previous"
            text: "Back"
            onTriggered: applicationWindow().pageStack.pop()
        }
    ]

}

