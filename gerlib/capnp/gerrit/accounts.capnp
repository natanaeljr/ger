@0xd0c25293d845e201;

using Cxx = import "/capnp/c++.capnp";
using Json = import "/capnp/compat/json.capnp";

$Cxx.namespace("gerrit::accounts");

##############################################################################

struct AccountInfo {
  accountId @0 :UInt32 $Json.name("_account_id");
  name @1 :Text;
  email @2 :Text;
  username @3 :Text;
}

enum GpgKeyStatus {
  bad @0 $Json.name("BAD");
  ok @1 $Json.name("OK");
  trusted @2 $Json.name("TRUSTED");
}

struct GpgKeyInfo {
  id @0 :Text;
  fingerprint @1 :Text;
  userIds @2 :Text;
  key @3 :Text;
  status @4 :GpgKeyStatus;
  problems @5 :List(Text);
}