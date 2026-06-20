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
            Component.onCompleted: highlighter.startMessageThread(helper)
        }

        AsyncHelper {
            id: helper
            Component.onCompleted: helper.start_async_worker()
        }

        actions: [
            Kirigami.Action {
                icon.name: "document-save"
                text: "Save"
            }
        ]

        Controls.Menu {
            id: contextMenu
            // closePolicy: Controls.Popup.CloseOnEscape
            
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
            Component.onCompleted: highlighter.setTextDocument(sourceArea.textDocument)
            onTextChanged: t => {
                helper.text_area_changed(sourceArea.text)
            }
        }

        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.RightButton
            onClicked: mouse => {
                var pos = sourceArea.positionAt(mouse.x, mouse.y)
                if (pos < 0) return

                var bounds = highlighter.findRecommendation(pos)
                if (bounds.length === 0) {
                    contextMenu.rebuild([])
                    return
                }

                var parts = bounds.split(';')
                page.wordStart = parseInt(parts[0])
                page.wordEnd = parseInt(parts[1])

                sourceArea.cursorPosition = page.wordStart
                sourceArea.moveCursorSelection(page.wordEnd, TextEdit.SelectCharacters)

                var items = highlighter.getSuggestions(page.wordStart, page.wordEnd)
                if (items.length > 0) {
                    contextMenu.rebuild(items)
                    contextMenu.popup(mouse.x, mouse.y + 10)
                }
            }
        }
    }
}

