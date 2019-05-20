import QtQuick 2.7
import QtQuick.Controls 2.5
import QtQuick.Window 2.12
import QtQuick.Layouts 1.12
import af.black.activitydesk.handlers 0.1

Window {
  id: accountNewDialog
  title: qsTr("Add New Account - ActivityDesk")
  visible: true
  width: 320
  height: 240
  modality: Qt.WindowModal
  flags: Qt.Dialog

  ColumnLayout {
    anchors.fill: parent
    anchors.margins: 8

    Text {
      text: "Enter your profile URI"
      Layout.fillWidth: true
      Layout.fillHeight: true
      fontSizeMode: Text.Fit
      minimumPixelSize: 10
      font.pixelSize: 24
      horizontalAlignment: Text.AlignLeft
    }

    Text {
      text: "The URI can be your Pleroma/Mastodon/PixelFed profile or to your representative h-card on your personal site. ActivityDesk will attempt to figure out what platform/network you're using."
      Layout.fillWidth: true
      Layout.fillHeight: true
      wrapMode: Text.Wrap
      horizontalAlignment: Text.AlignLeft
      verticalAlignment: Text.AlignTop
    }

    TextField {
      id: textFieldProfileURN
      Layout.fillWidth: true
      Layout.fillHeight: true
      placeholderText: qsTr("https://your.profile/on/the/web")
    }

    RowLayout {
      Layout.fillWidth: true
      anchors.horizontalCenter: accountNewDialog.horizontalCenter
      anchors.top: textFieldProfileURN.bottom
      anchors.topMargin: 8

      Button {
        text: "Continue"
        onClicked: resolveUserForAddition()
      }
    }
  }

  NewAccountDialogHandler {
    id: handler
  }

  function resolveUserForAddition() {
    const profileUrl = textFieldProfileURN.text;
    const result_type = handler.resolve_profile_kind(profileUrl);

    if (result_type != "unknown") {
      const authUrl = handler.get_auth_url(result_type, profileUrl);
      Qt.openUrlExternally(authUrl);
    } else {
      console.log("Unknown platform.")
    }
  }
}
