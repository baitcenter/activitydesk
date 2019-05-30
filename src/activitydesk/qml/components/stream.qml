import QtQuick 2.12
import af.black.activitydesk.models 0.1
import af.black.activitydesk.handlers 0.1
import "qrc:/qml/Components" as Components

ListView {
  id: view
  delegate: Components.Post {}
  model: StreamList { id: model }
  highlight: Rectangle { color: "lightsteelblue"; radius: 5 }
  property string identity_url
  property string stream_kind

  function setStream(identity_url, stream_kind) {
    model.set_stream(identity_url, stream_kind)
  }

  signal streamUpdated(int newPosts);

  Component.onCompleted: function() {
    setStream(this.identity_url, this.stream_kind)
  }
}
