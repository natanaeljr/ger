@0xd0c25293d845e201

using Cxx = import "/capnp/c++.capnp";

$Cxx.namespace("gerrit::accounts");

##############################################################################

struct AccountInfo {
  _account_id @0 :UInt32;
  name @1 :Text;
  email @2 :Text;
  username @3 :Text;
}
