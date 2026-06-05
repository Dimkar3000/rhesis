// Includes relevant modules used by the QML
import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as Controls
import org.kde.kirigami as Kirigami
import org.kde.rhesis

// Provides basic features needed for all kirigami applications
Kirigami.ApplicationWindow {
    // Unique identifier to reference this object
    id: root

    width: 400
    height: 300

    // Window title
    title: "Hello Linux"

    // Set the first page that will be loaded when the app opens
    // This can also be set to an id of a Kirigami.Page
    pageStack.initialPage: Kirigami.Page {
        id: page

        padding: 0

        property int wordStart: 0
        property int wordEnd: 0

        CustomHighlighter {
            id: highlighter
            Component.onCompleted: highlighter.setTextDocument(sourceArea.textDocument)
        }

        actions: [
            Kirigami.Action {
                icon.name: "document-save"
                text: "Save"
            }
        ]

        Controls.Menu {
            id: contextMenu

            function rebuild(suggestions) {
                while (contextMenu.count > 0) {
                    var item = contextMenu.itemAt(0)
                    contextMenu.removeItem(item)
                    item.destroy()
                }

                for (var i = 0; i < suggestions.length; i++) {
                    var item = menuItemComponent.createObject(null)
                    item.text = suggestions[i]
                    contextMenu.addItem(item)
                }
            }

            Component {
                id: menuItemComponent
                Controls.MenuItem {
                    onTriggered: {
                        sourceArea.text = sourceArea.text.substring(0, page.wordStart) + text + sourceArea.text.substring(page.wordEnd)
                    }
                }
            }
        }

        Controls.TextArea {
            id: sourceArea
            anchors.fill: parent
            padding: 5
            background: null
            wrapMode: Controls.TextArea.Wrap
            placeholderText: "Enter text..."
        }

        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.RightButton
            onClicked: mouse => {
                sourceArea.cursorPosition = sourceArea.positionAt(mouse.x, mouse.y)
                sourceArea.selectWord()
                page.wordStart = sourceArea.selectionStart
                page.wordEnd = sourceArea.selectionEnd

                var selected = sourceArea.selectedText
                if (selected.length > 0) {
                    var raw = highlighter.getSuggestions(selected)
                    contextMenu.rebuild(raw.split(';'))
                } else {
                    contextMenu.rebuild([])
                }

                contextMenu.popup(mouse.x, mouse.y)
            }
        }
    }
}

