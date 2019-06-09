import QtQuick 2.5
import QtQuick.Controls 2.5
import QtQuick.Controls.Styles 1.0
import QtQuick.Window 2.2
import QtQuick.Layouts 1.12
import af.black.activitydesk.handlers 0.1

Window {
  id: accountNewDialog
  title: qsTr("Add New Account - ActivityDesk")
  visible: true
  width: 320
  height: 240
  modality: Qt.ApplicationModal
  flags: Qt.Sheet
  property string profile_image_url: ""
  property string profile_url: ""
  property string authorization_code: ""
  property url authorization_url: ""
  property var app_handler: Null

  NewAccountDialogHandler {
    id: handler
  }

  StackView {
    id: stack
    initialItem: viewCollectURL
    anchors.fill: parent
    anchors.margins: 8
  }

  Component {
    id: viewCollectURL

    ColumnLayout {
      Text {
        text: "Enter your profile URI."
        Layout.fillWidth: true
        fontSizeMode: Text.Fit
        minimumPixelSize: 10
        font.pixelSize: 24
        horizontalAlignment: Text.AlignLeft
      }

      Text {
        text: "The URI can be the instance of your Mastodon or your IndieWeb site. ActivityDesk will figure out what platform/network you're using."
        Layout.fillWidth: true
        Layout.fillHeight: true
        wrapMode: Text.Wrap
        horizontalAlignment: Text.AlignLeft
        verticalAlignment: Text.AlignTop
      }

      TextField {
        id: textFieldProfileURL
        Layout.fillWidth: true
        placeholderText: qsTr("https://your.profile/on/the/web")
      }

      RowLayout {
        Layout.fillWidth: true

        Button {
          text: "Continue"
          id: buttonContinue
          onClicked: {
            const profileUrl = textFieldProfileURL.text;
            handler.prepare_account_for(profileUrl);

            if (handler.can_login()) {
              const authUrl = handler.get_url();
              console.log(authUrl);
              stack.replace(viewCaptureCode);
              authorization_url = authUrl;
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
        fontSizeMode: Text.Fit
        minimumPixelSize: 10
        font.pixelSize: 24
        horizontalAlignment: Text.AlignLeft
      }

      Text {
        Layout.fillWidth: true
        Layout.fillHeight: true
        text: "A browser window will be launhed in a few moments. Once that happens, be sure to either collect the code and enter it into the field below."
        wrapMode: Text.Wrap
        horizontalAlignment: Text.AlignLeft
        verticalAlignment: Text.AlignTop
      }

      RowLayout {
        id: codeConfirmationLayout
        visible: false

        TextField {
          id: textFieldCode
          Layout.fillWidth: true
          font.family: "monospace"
        }

        Button {
          text: "Confirm Code"
          onClicked: function() {
            handler.obtain_token(textFieldCode.text);

            if (handler.has_token() && handler.resolve_user()) {
              console.log(handler.user_image_url);
              profile_url = handler.user_url;
              profile_image_url = handler.user_image_url;
              stack.replace(viewShowProfile);
            } else {
              stack.replace(viewCodeFailed);
              codeConfirmationLayout.visible = false
            }
          }
        }
      }

      RowLayout {
        id: launchBrowserLayout
        visible: !codeConfirmationLayout.visible

        TextField {
          Layout.fillWidth: true
          text: accountNewDialog.authorization_url
          readOnly: true
          enabled: false
        }

        Button {
          text: "Launch"
          onClicked: function() {
            codeConfirmationLayout.visible = Qt.openUrlExternally(accountNewDialog.authorization_url);
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
        fontSizeMode: Text.Fit
        minimumPixelSize: 10
        font.pixelSize: 24
      }

      Image {
        id: imageProfile
        source: profile_image_url
        sourceSize.width: 256
        sourceSize.height: 256
        Layout.alignment: Qt.AlignVCenter | Qt.AlignHCenter
        Layout.fillWidth: true
        Layout.fillHeight: true
        fillMode: Image.PreserveAspectFit
      }

      RowLayout {
        Layout.fillWidth: true

        Button {
          text: "Save Account"
          onClicked: {
            let result = handler.result();
            console.log(result);
            app_handler.register_new_account(result);
            accountNewDialog.close();
          }
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
