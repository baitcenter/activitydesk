import QtQuick 2.7
import QtQuick.Controls 2.5
import QtQuick.Controls.Styles 1.3
import QtQuick.Layouts 1.5
import QtQuick.Window 2.7
import af.black.activitydesk.handlers 0.1
import af.black.activitydesk.models 0.1

ApplicationWindow {

  id: mainWindow
  visible: true
  width: 480
  height: 640
  title: qsTr("ActivityDesk")

  // TODO: Refactor this into a helper method.
  function addAccountMenuItem_clicked() {
    const dialogKlass = Qt.createComponent("qrc:/qml/Accounts/New.qml");
    dialogKlass.createObject(mainWindow, {app_handler: handler, visible: true});
  }

  MainWindowHandler {
    id: handler
  }

  IdentityList {
    id: identityList
  }

  MenuBar {
    id: menuBar
    Menu {
      title: "Account"

      Action {
        text: "Add"
        onTriggered: { addAccountMenuItem_clicked() }
      }
    }
  }

  ColumnLayout {
    anchors.fill: parent

    Component {
      id: accountSelectorDelegate
      ItemDelegate {
        Layout.fillWidth: true
        text: "(" + name + ") " + url
        icon.name: "edit-copy"
      }
    }

    ComboBox {
      id: accountSelector
      Layout.fillWidth: true
      textRole: "name"
      flat: true
      model: identityList
      delegate: accountSelectorDelegate
    }

    TabBar {
      id: mainBar
      Layout.fillWidth: true

      TabButton {
        text: qsTr("Home")
      }
    }

    StackLayout {
      Layout.fillWidth: true
      Layout.fillHeight: true
      currentIndex: mainBar.currentIndex
    }
  }


  Component.onCompleted: function() {
    identityList.add_all_from_system();
  }
}
