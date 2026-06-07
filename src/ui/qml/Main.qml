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
            closePolicy: Controls.Popup.CloseOnEscape
            
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
                        highlighter.replaceWord(page.wordStart, page.wordEnd, text)
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
                var pos = sourceArea.positionAt(mouse.x, mouse.y)
                if (pos < 0) return

                sourceArea.cursorPosition = pos
                sourceArea.selectWord()

                if (sourceArea.selectionStart < 0 || sourceArea.selectionEnd < 0) return

                page.wordStart = sourceArea.selectionStart
                page.wordEnd = sourceArea.selectionEnd

                var selected = sourceArea.selectedText
                if (selected.length > 0) {
                    var raw = highlighter.getSuggestions(page.wordStart, page.wordEnd)
                    console.log(raw)
                    if(raw.length > 0) {
                        contextMenu.rebuild(raw.split(';'))
                        contextMenu.popup(mouse.x, mouse.y + 10)
                    }
                } else {
                    contextMenu.rebuild([])
                }
            }
        }
    }
}

