@0x8cded0298d2ab273;

using Cxx = import "/capnp/c++.capnp";
using Json = import "/capnp/compat/json.capnp";
using ListMap = import "/util/listmap.capnp".ListMap;
$Cxx.namespace("gerrit::changes");

enum ChangeStatus {
  new @0 $Json.name("NEW");
  merged @1 $Json.name("MERGED");
  abandoned @2 $Json.name("ABANDONED");
  draft @3 $Json.name("DRAFT");
}

enum ReviewerState {
  reviewer @0 $Json.name("REVIEWER");
  cc @1 $Json.name("CC");
  removed @2 $Json.name("REMOVED");
}

struct ChangeInfo {
  id @0 :Text;
  project @1 :Text;
  branch @2 :Text;
  topic @3 :Text;
  status @4 :ChangeStatus;
  number @5 :UInt32 $Json.name("_number");
  reviewers @6 :ListMap(Text, List(Text));
}