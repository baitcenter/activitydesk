import QtQuick.Layouts 1.12
import QtQuick.Controls 2.5
import QtQuick 2.0
import QtGraphicalEffects 1.12

GridLayout {
  property string displayed_name: "Jacky Alcine"
  property string avatar_image_url: "https://placedog.net/96"
  property string content: "Waiting to get this ready."
  id: parent
  Layout.fillWidth: true
  Layout.margins: 4
  Layout.minimumHeight: 108
  Layout.minimumWidth: 512
  Layout.preferredWidth: 512
  Layout.preferredHeight: 108
  rows: 2
  columns: 2

  Item {
    id: icon
    Layout.maximumWidth: 48
    Layout.minimumWidth: 48
    Layout.leftMargin: 8
    Layout.topMargin: 4
    Layout.rowSpan: 2
    Layout.row: 0
    Layout.column: 0
    width: 48
    height: 48
    Layout.alignment: Qt.AlignTop | Qt.AlignLeft

    Image {
      id: avatar
      source: parent.avatar_image_url
      sourceSize.height: 48
      height: 48
      fillMode: Image.PreserveAspectCrop
      Layout.rightMargin: 0
      z: -1
      layer.enabled: true
      layer.effect: OpacityMask {
        anchors.centerIn: avatar
        maskSource: Item {
          width: avatar.width
          height: avatar.height
          Rectangle {
            width: Math.min(avatar.width, avatar.height)
            height: width
            radius: Math.min(width, height)
          }
        }
      }
    }

    Image {
      id: platformLogo
      source: "https://placedog.net/16"
      Layout.maximumWidth: 16
      fillMode: Image.PreserveAspectCrop
      sourceSize.width: 16
      width: 16
      height: 16
      x: (Math.min(avatar.width, avatar.height) - width)
      y: (Math.min(avatar.width, avatar.height) - height)
      layer.enabled: true
      layer.effect: OpacityMask {
        anchors.centerIn: platformLogo
        maskSource: Item {
          width: platformLogo.width
          height: platformLogo.height
          Rectangle {
            width: Math.min(platformLogo.width, platformLogo.height)
            height: platformLogo.height
            radius: Math.min(width, height)
            border.width: 2
            border.color: "#0074D9"
          }
        }
      }
    }
  }

  Text {
    Layout.row: 0
    Layout.column: 1
    Layout.alignment: Qt.AlignTop | Qt.AlignLeft
    Layout.margins: 4
    Layout.bottomMargin: 2
    Layout.fillWidth: true
    text: parent.displayed_name
    font.weight: Font.Bold
  }

  Label {
    Layout.alignment: Qt.AlignTop | Qt.AlignLeft
    Layout.fillWidth: true
    Layout.fillHeight: true
    Layout.margins: 4
    Layout.topMargin: 2
    Layout.row: 1
    Layout.column: 1
    wrapMode: Text.WrapAnywhere
    width: parent.width - icon.width
    textFormat: Text.RichText
    text: parent.content
    padding: 4
    background: Rectangle {
      color: "#99BCBCBC"
      radius: 2
    }
  }
}
