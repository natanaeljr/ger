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
