import QtQuick 2.7
import QtQuick.Controls 2.5
import QtQuick.Window 2.12
import QtQuick.Layouts 1.12
import Qt.labs.settings 1.0
import af.black.activitydesk.handlers 0.1

Window {
  id: accountNewDialog
  title: qsTr("Add New Account - ActivityDesk")
  visible: true
  width: 320
  height: 240
  modality: Qt.WindowModal
  flags: Qt.Dialog

  NewAccountDialogHandler {
    id: handler
  }

  StackView {
    id: stack
    initialItem: viewCollectURL
    anchors.fill: parent
    anchors.margins: 8
  }

  Settings {
    id: settingsAccount
    property var url: ""
    property var kind: ""
    property var token: ""
  }

  Component {
    id: viewCollectURL

    ColumnLayout {
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
        id: textFieldProfileURL
        Layout.fillWidth: true
        Layout.fillHeight: true
        placeholderText: qsTr("https://your.profile/on/the/web")
      }

      RowLayout {
        Layout.fillWidth: true

        Button {
          text: "Continue"
          onClicked: {
            const profileUrl = textFieldProfileURL.text;
            const result_type = handler.resolve_profile_kind(profileUrl);

            if (result_type != "unknown") {
              const authUrl = handler.get_auth_url(result_type, profileUrl);
              Qt.openUrlExternally(authUrl);
              settingsAccount.category = "Account " + Qt.md5(profileUrl);
              settingsAccount.url = profileUrl;
              settingsAccount.kind = result_type;
              stack.replace(viewCaptureCode);
            } else {
              console.log("Unknown platform.")
            }
          }
        }
      }
    }
  }

  Component {
    id: viewCaptureCode

    ColumnLayout {
      Text {
        text: "Enter Authorization Code"
        Layout.fillWidth: true
        Layout.fillHeight: true
        fontSizeMode: Text.Fit
        minimumPixelSize: 10
        font.pixelSize: 24
        horizontalAlignment: Text.AlignLeft
      }

      Text {
        text: "Fill out the authorization code provided below."
        Layout.fillWidth: true
        Layout.fillHeight: true
        wrapMode: Text.Wrap
        horizontalAlignment: Text.AlignLeft
        verticalAlignment: Text.AlignTop
      }

      TextField {
        id: textFieldAuthorizationCode
        Layout.fillWidth: true
        Layout.fillHeight: true
      }

      RowLayout {
        Layout.fillWidth: true

        Button {
          text: "Confirm Code"
          onClicked: {
            const code = textFieldAuthorizationCode.text
            const authToken = handler.get_auth_token(code);
            if (authToken != "") {
              settingsAccount.token = authToken;
              stack.replace(viewShowProfile);
            } else {
              stack.replace(viewCodeFailed);
            }
          }
        }
      }
    }
  }

  Component {
    id: viewShowProfile

    ColumnLayout {
      Text {
        text: "Account added!"
      }
      RowLayout {
        Layout.fillWidth: true

        Button {
          text: "Save Account"
        }
      }
    }
  }

  Component {
    id: viewCodeFailed

    ColumnLayout {
      Text {
        text: "Code failed on login attempt!"
      }
      RowLayout {
        Layout.fillWidth: true

        Button {
          text: "Try Again"
          onClicked: stack.push(viewCollectURL)
        }
      }
    }
  }

}
