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

      MenuItem { text: "Add" }
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
    Tab { title: "Home"
    ListView {
      anchors.fill: parent
      delegate: Text {
        text: name + ": " + number
      }
      model: ListModel {
        ListElement {
          name: "Bill Smith"
          number: "555 3264"
        }
        ListElement {
          name: "John Brown"
          number: "555 8426"
        }
        ListElement {
          name: "Sam Wise"
          number: "555 0473"
        }
      }
    }

  }
  Tab { title: "Mentions" }
  Tab { title: "Direct Messages" }
}
}
