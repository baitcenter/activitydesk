import QtQuick 2.0
import QtQuick.Controls 2.0
import QtQuick.Controls.Styles 1.0
import QtQuick.Layouts 1.12
import QtQuick.Window 2.0
import Qt.labs.platform 1.1 as Platform
import af.black.activitydesk.handlers 0.1
import af.black.activitydesk.models 0.1
import "qrc:/qml/Components" as Components

ApplicationWindow {
  id: mainWindow
  visible: true
  width: 480
  height: 640
  minimumWidth: 480
  minimumHeight: 640
  title: qsTr("ActivityDesk")
  property var composer: null
  property var chooser: null
  property var streams: []

  // TODO: Refactor this into a helper method.
  function addAccountMenuItem_clicked() {
    const dialogKlass = Qt.createComponent("qrc:/qml/Accounts/New.qml");
    dialogKlass.createObject(mainWindow, {app_handler: handler, visible: true});
  }

  function wipeAccountMenuItem_clicked() {
    handler.wipe_it()
  }

  Component {
    id: newStreamView
    Components.Stream {}
  }

  MainWindowHandler {
    id: handler

    onCurrent_identity_changed: function() {
      composer.set_identity_url(handler.current_identity_url);
    }

    onPresent_new_stream: function(identity_url, stream_id) {
      const stream_data = {"identity_url": identity_url, "stream_kind": stream_id};
      const new_view = newStreamView.createObject(null, stream_data);
      streamView.addItem(new_view);
      streams.push(stream_data);
      streamBarRepeater.model = streams;
    }
  }

  Platform.MenuBar {
    id: menuBar
    Platform.Menu {
      title: "Account"

      Platform.MenuItem {
        text: "Add"
        onTriggered: { addAccountMenuItem_clicked() }
      }

      Platform.MenuItem {
        text: "Wipe"
        onTriggered: { wipeAccountMenuItem_clicked() }
      }
    }
  }

  GridLayout {
    anchors.fill: parent
    columns: 1
    rows: 4
    flow: GridLayout.TopToBottom
    Layout.fillWidth: true
    Layout.fillHeight: true
    Layout.preferredWidth: parent.width

    Components.AccountChooser {
      id: accountChooser
      Layout.fillWidth: true
      Layout.preferredHeight: 32

      Component.onCompleted: {
        this.selected.connect((uri) => {
          handler.set_current_identity(uri);
        });
      }
    }

    Components.Composer {
      id: composer
      Layout.fillWidth: true
      Layout.preferredHeight: 96
      Layout.maximumHeight: 128
      Layout.bottomMargin: 8
      Component.onCompleted: function() {
        handler.current_identity_changed.connect(() => {
          composer.set_identity_url(handler.current_identity_url);
        })
      }
    }

    TabBar {
      id: streamBar
      Layout.fillWidth: true
      Layout.preferredWidth: parent.width

      Repeater {
        id: streamBarRepeater
        model: mainWindow.streams

        TabButton {
          text: modelData.stream_kind + ":" + modelData.identity_url
        }
      }
    }

    SwipeView {
      id: streamView
      currentIndex: streamBar.currentIndex
      Layout.fillWidth: true
      Layout.fillHeight: true
    }
  }

  Component.onCompleted: function() {
    handler.setup();
  }
}
