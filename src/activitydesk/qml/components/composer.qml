import QtQml 2.12
import QtQuick 2.7
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.5
import af.black.activitydesk.models 0.1
import af.black.activitydesk.handlers 0.1

RowLayout {
  function set_identity_url(identity_url) {
    console.log("Composer::set_identity_url", identity_url);
    model.load(identity_url);
    console.log("Composer::set_identity_url", model);
  }

  Layout.fillWidth: true
  Layout.maximumHeight: 128

  ComposerHandler {
    id: handle
    identity: model.ident
    message: textEditor.text
    summary: summaryEditor.text
  }

  IdentityModel {
    id: model
    onUpdated: function() {
      if (this.profile_image_url != null) {
        avatar.source = model.profile_image_url
      }

      avatar.visible = model.profile_image_url != ""
    }
  }

  Image {
    id: avatar
    source: model.profile_image_url
    sourceSize.height: 96
    height: 96
    fillMode: Image.PreserveAspectCrop
    Layout.margins: 8
    Layout.rightMargin: 0
    Layout.fillHeight: true
    Layout.maximumWidth: 96
    Layout.minimumWidth: 48
    Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
    z: -1
  }

  ColumnLayout {
    Layout.margins: 4
    TextField {
      id: summaryEditor
      Layout.fillWidth: true
      placeholderText: "Summary; content warning"
      visible: toggleCW.checked
      text: handle.summary
    }

    ScrollView {
      Layout.fillHeight: true
      Layout.fillWidth: true

      TextArea {
        id: textEditor
        text: handle.message
        hoverEnabled: true
        textFormat: TextEdit.AutoText
        placeholderText: qsTr("What's up?")
        wrapMode: TextEdit.WrapAnywhere
        verticalAlignment: TextEdit.AlignTop
        horizontalAlignment: TextEdit.AlignLeft
        Layout.bottomMargin: 4
      }
    }
  }

  ColumnLayout {
    spacing: 2
    Layout.margins: 4
    Layout.leftMargin: 0
    Layout.preferredWidth: 96
    Layout.maximumWidth: 112

    Item {
      Layout.fillHeight: true
    }

    ComboBox {
      id: visibility
      model: ["Public", "Followers", "Unlisted"]
      Layout.fillWidth: true
    }

    Switch {
      text: qsTr("CW")
      id: toggleCW
      checked: false
      Layout.fillWidth: true
    }

    Button {
      text: "Send"
      Layout.fillWidth: true
      Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
      onClicked: function() {
        if (handle.send()) {
          textEditor.clear();
          summaryEditor.clear();
        }
      }
    }
  }
}

/**
 * Provide an interface that allows the following:
 *
 * - [ ] Show the icon of the identity being used to post.
 * - [ ] Show an icon associated with the network being used.
 * - [ ] Show a composer box (plain text only).
 * - [ ] Emit event when message has been composed.
 */
