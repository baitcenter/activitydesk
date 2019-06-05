import QtQuick 2.0
import QtQuick.Controls 1.4
import af.black.activitydesk.models 0.1
import af.black.activitydesk.handlers 0.1
import "qrc:/qml/Components" as Components

ScrollView {
  id: view
    property string identity_url
    property string stream_kind

  ListView {
    snapMode: ListView.SnapOneItem
    spacing: 8
    delegate: Component {
      id: delegate
      Components.Post {
        displayed_name: model.displayed_name
        avatar_image_url: model.avatar_image_url
        content: model.content
      }
    }

    model: StreamList {
      id: model
    }

    Component.onCompleted: function() {
      model.set_stream(view.identity_url, view.stream_kind)
    }
  }
}
