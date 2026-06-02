// Includes relevant modules used by the QML
import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami

// Provides basic features needed for all kirigami applications
Kirigami.ApplicationWindow {
    // Unique identifier to reference this object
    id: root

    width: 400
    height: 300

    // Window title
    title: "Hello World"

    // Set the first page that will be loaded when the app opens
    // This can also be set to an id of a Kirigami.Page
    pageStack.initialPage: Kirigami.Page {
        padding: 0
        
        actions: [
            Kirigami.Action {
                icon.name: "document-save"
                text: "Save"
            }
        ]

        Controls.TextArea {
            anchors.fill: parent
            padding: 5
            background: null
            wrapMode: TextArea.Wrap
            placeholderText: "Enter text..."
        }
    }
}

