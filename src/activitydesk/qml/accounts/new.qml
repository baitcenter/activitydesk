import QtQuick 2.7
import QtQuick.Controls 2.5
import QtQuick.Window 2.12
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.12
import QtWebKit 3.0
import af.black.activitydesk.handlers 0.1

Window {
  id: accountNewDialog
  title: qsTr("Add New Account - ActivityDesk")
  visible: true
  width: 640
  height: 480
  modality: Qt.ApplicationModal
  flags: Qt.Sheet
  property var profile_image_url: ""
  property var profile_url: ""
  property var app_handler: Null
  property url authorization_url: ""

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
        text: "Enter your profile URI"
        Layout.fillWidth: true
        fontSizeMode: Text.Fit
        minimumPixelSize: 10
        font.pixelSize: 24
        horizontalAlignment: Text.AlignLeft
      }

      Text {
        text: "The URI can be the instance of Pleroma/Mastodon/PixelFed or your IndieWeb site. ActivityDesk will attempt to figure out what platform/network you're using."
        Layout.fillWidth: true
        Layout.fillHeight: true
        padding: 16
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

      TextField {
        Layout.fillWidth: true
        id: fieldUrl
        readOnly: true
        text: webView.url
      }

      WebView {
        id: webView
        Layout.fillWidth: true
        Layout.fillHeight: true
        url: authorization_url
        onLoadingChanged: function(request) {
          if (request.status == WebView.LoadSucceededStatus && request.url.toString().includes("code=")) {
            const code_part = request.url.toString().split("code=")[1].split("&")[0]
            handler.obtain_token(code_part);
            authorization_url = "about:blank"

            if (handler.has_token() && handler.resolve_user()) {
              console.log(handler.user_image_url);
              profile_url =  handler.user_url;
              profile_image_url = handler.user_image_url;
              stack.replace(viewShowProfile);
            } else {
              stack.replace(viewCodeFailed);
            }

          } else {
            console.log(request.url)
          }
        }
      }

      ProgressBar {
        id: progressWebView
        Layout.fillWidth: true
        visible: webView.loading
        value: webView.loadProgress
        from: 0
        to: 100
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
