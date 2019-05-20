import QtQuick 2.7
import QtQuick.Controls 1.5
import QtQuick.Controls.Styles 1.3
import QtQuick.Layouts 1.5
import QtQuick.Window 2.7

ApplicationWindow {
  id: mainWindow
  visible: true
  width: 480
  height: 640
  title: qsTr("ActivityDesk")

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
    Tab { title: "Home" }
    Tab { title: "Mentions" }
    Tab { title: "Direct Messages" }
  }

  Component.onCompleted : {
    Qt.application.name = "ActivityDesk"
    Qt.application.organization = "black.af"
    Qt.application.domain = "black.af"
  }

  // TODO: Refactor this into a helper method.
  function addAccountMenuItem_clicked() {
    const dialogKlass = Qt.createComponent("qrc:/qml/Accounts/New.qml");
    const dialog = dialogKlass.createObject(mainWindow);
    dialog.visible = true;
  }
}
