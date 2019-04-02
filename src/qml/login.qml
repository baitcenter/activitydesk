import QtQuick 2.7
import QtQuick.Controls 1.4
import QtQuick.Controls.Styles 1.4
import QtQuick.Layouts 1.2
import QtQuick.Window 2.2

Window {
  visible: true
  width: 400
  height: 512
  title: qsTr("Login - ActivityDesk")

  SystemPalette {
    id: palette
    colorGroup: SystemPalette.Active
  }

  FontMetrics {
    id: systemFont
  }
}
