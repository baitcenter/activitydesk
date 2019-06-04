// TODO: Extend the Tumbler element to show a list of entries.
// TODO: Emit event when an account is selected.
// TODO: On construction, request loading of all known accounts.
import QtQuick 2.0
import QtQuick.Controls 2.0
import af.black.activitydesk.models 0.1

ComboBox {
  signal selected(string identity_url)
  textRole: "name"
  flat: true
  model: IdentityList { id: model }
  delegate: Component {
    ItemDelegate {
      text: "(" + name + ") " + url
      icon.name: "edit-copy"
    }
  }

  onActivated: function(index) {
    const identity_url = model.get_identity_url(index);
    selected(identity_url);
  }

  Component.onCompleted: function() {
    model.add_all_from_system();
    this.currentIndex = 0;
    const identity_url = model.get_identity_url(this.currentIndex);
    console.log("First ident", identity_url);
    selected(identity_url);
  }
}
