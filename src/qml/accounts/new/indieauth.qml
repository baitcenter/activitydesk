import QtQuick 2.7
import QtQuick.Controls 1.4
import QtQuick.Controls.Styles 1.4
import QtQuick.Layouts 1.2
import QtQuick.Window 2.2
import QtQuick.Controls.Styles.Desktop 1.0

ApplicationWindow {
  id: window
  visible: true
  width: 400
  height: 512
  title: qsTr("Sign Into Your IndieWeb Site - ActivityDesk")

  Column {
    id: column
    anchors.fill: parent

    TextField {
      id: textField
      height: 32
      anchors.left: parent.left
      anchors.leftMargin: 59
      anchors.right: parent.right
      anchors.rightMargin: 59
      anchors.bottom: parent.bottom
      anchors.bottomMargin: 73
      placeholderText: qsTr("URI to your h-card")
    }

    Button {
      id: button
      width: 80
      height: 32
      text: qsTr("Sign In")
      anchors.right: parent.right
      anchors.rightMargin: 158
      anchors.left: parent.left
      anchors.leftMargin: 158
      anchors.bottom: parent.bottom
      anchors.bottomMargin: 24
    }
  }
}
