@0x8cded0298d2ab273;

using Cxx = import "/capnp/c++.capnp";
using Json = import "/capnp/compat/json.capnp";
$Cxx.namespace("gerrit::changes");

struct ListMap(Key, Value) {
 # ListMap is parsed a map in JSON
 # Key must be of type: (Text, enum)
 # Value can be of any type
 entries @0 :List(Node);
 struct Node {
  key @0 :Key;
  value @1 :Value;
 }
}

interface Node {
  isDirectory @0 () -> (result :Bool);
}

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