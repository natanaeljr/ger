@0xfe7e01e3deddb5eb;

$import "/capnp/c++.capnp".namespace("util");

struct ListMap(Key, Value) {
  # ListMap is parsed as a map in JSON
  entries @0 :List(Node);
  struct Node {
   key @0 :Key;
   value @1 :Value;
  }
}
