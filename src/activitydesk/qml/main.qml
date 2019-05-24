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

  TabView {
    id: tabView
    anchors.fill: parent
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
