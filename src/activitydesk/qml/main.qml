import QtQuick 2.7
import QtQuick.Controls 1.5
import QtQuick.Controls.Styles 1.3
import QtQuick.Layouts 1.5
import QtQuick.Window 2.7
import af.black.activitydesk.handlers 0.1

ApplicationWindow {
  id: mainWindow
  visible: true
  width: 480
  height: 640
  title: qsTr("ActivityDesk")

  MainWindowHandler {
    id: handler
  }

  menuBar: MenuBar {
    id: menuBar
    Menu {
      title: "App"

      MenuItem {
        text: "Settings"
      }
      MenuSeparator {}
      MenuItem {
        text: "Quit"
        onTriggered: Qt.quit()
      }
    }

    Menu {
      title: "Account"

      MenuItem {
        text: "Add"
        onTriggered: { addAccountMenuItem_clicked() }
      }
      MenuItem { text: "Remove" }

      MenuSeparator {}

      MenuItem { text: "Manage..." }
    }

    Menu {
      title: "Feeds"
      MenuItem { text: "Jump To" }
      MenuSeparator {}
      MenuItem { text: "Manage..." }
    }

    Menu {
      title: "About"
      MenuItem { text: "Help" }
      MenuItem { text: "Contribute" }
      MenuItem { text: "Donate" }
      MenuSeparator {}
      MenuItem { text: "About ActivityDesk" }
    }
  }

  ColumnLayout {
    anchors.fill: parent

    ComboBox {
      id: accountSelector
      model: []
      Layout.fillWidth: true
      Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
    }


    TabView {
      id: tabView
      Layout.fillWidth: true
      Layout.fillHeight: true

      Tab {
        id: tabTimeline
        title: "Everything"

        ListView {
          id: timeline
          spacing: 8
          leftMargin: 8
          rightMargin: 8
          topMargin: 8
          bottomMargin: 8
          anchors.fill: parent

          model: ListModel {
            id: wow
            ListElement {
              name: "Jacky"
              src: "http://placedog.net/200/200"
              content: "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
            }
            ListElement {
              name: "Jacky"
              src: "http://placedog.net/150/150/b"
              content: "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
            }
            ListElement {
              name: "Jacky"
              src: "http://placedog.net/300/300/invert"
              content: "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
            }
          }

          delegate: Row {
            spacing: 8

            Image {
              source: src
              sourceSize.width: 96
              sourceSize.height: 96
              height: 48
              width: 48
              autoTransform: true
              smooth: false
              fillMode: Image.PreserveAspectFit
              opacity: 0.5
              cache: false
            }

            ColumnLayout {
              anchors.leftMargin: 8

              Text {
                text: name
                minimumPixelSize: 10
                font.pixelSize: 16
                font.bold: true
                Layout.fillWidth: true
                Layout.rightMargin: 8
                horizontalAlignment: Text.AlignLeft
                height: 24
              }

              Text {
                topPadding: 4
                bottomPadding: 4
                text: content
                Layout.fillWidth: true
                Layout.rightMargin: 8
                Layout.fillHeight: true
                wrapMode: Text.WrapAnywhere
              }
            }
          }
        }
      }
    }
  }

  Component.onCompleted : {
    Qt.application.name = "ActivityDesk"
    Qt.application.organization = "black.af"
    Qt.application.domain = "black.af"
    handler.load_streams()
  }

  // TODO: Refactor this into a helper method.
  function addAccountMenuItem_clicked() {
    const dialogKlass = Qt.createComponent("qrc:/qml/Accounts/New.qml");
    dialogKlass.createObject(mainWindow, {app_handler: handler, visible: true});
  }
}
