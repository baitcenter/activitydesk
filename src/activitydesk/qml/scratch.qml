import QtQuick 2.2
import QtQuick.Controls 1.5
import QtQuick.Layouts 1.5
import QtQuick.Window 2.0

ApplicationWindow {
    visible: true
    width: 320 * 1.5
    height: 480 * 1.5
    Component.onCompleted: {
        setX(Screen.width / 2 - width / 2);
        setY(Screen.height / 2 - height / 2);
    }

    ColumnLayout {
      ListView {
        model: [1, 2]
        delegate: Row {
          Layout.fillWidth: true

          Image { source: "https://placedog.net/500" }
          Text { text: "WOW"}
        }
      }
    }
}
