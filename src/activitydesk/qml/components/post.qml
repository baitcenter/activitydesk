import QtQuick.Layouts 1.12
import QtQuick.Controls 2.5
import QtQuick 2.7
import QtGraphicalEffects 1.12

RowLayout {
  anchors.fill: parent
  Layout.fillWidth: true
  Layout.fillHeight: true
  Layout.margins: 4
  Layout.minimumHeight: 108
  Layout.minimumWidth: 512
  width: 512
  height: 108

  Item {
    Layout.maximumWidth: 64
    Layout.minimumWidth: 48
    Layout.leftMargin: 8
    Layout.topMargin: 4
    width: 64
    height: 64
    Layout.alignment: Qt.AlignTop | Qt.AlignLeft

    Image {
      id: avatar
      source: "https://placedog.net/96"
      sourceSize.height: 64
      height: 64
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
      source: "https://placedog.net/32"
      Layout.maximumWidth: 24
      fillMode: Image.PreserveAspectCrop
      sourceSize.width: 24
      width: 24
      height: 24
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

  ColumnLayout {
    Layout.fillWidth: true
    Layout.fillHeight: true
    Layout.leftMargin: 8
    Layout.rightMargin: 8
    Layout.alignment: Qt.AlignTop | Qt.AlignLeft

    Text {
      Layout.alignment: Qt.AlignTop | Qt.AlignRight
      Layout.topMargin: 4
      Layout.fillWidth: true
      text: "Jacky Alcine"
      font.weight: Font.Bold
    }

    Label {
      Layout.alignment: Qt.AlignTop | Qt.AlignLeft
      Layout.fillWidth: true
      wrapMode: Text.WordWrap
      text: "I'm just testing out having posts here."
      padding: 8
      background: Rectangle {
        color: "#99BCBCBC"
        radius: 2
      }
    }
  }
}
