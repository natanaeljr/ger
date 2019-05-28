@0xfe7e01e3deddb5eb;

$import "/capnp/c++.capnp".namespace("util");

struct ListMap(Key, Value) {
  # ListMap is parsed as a map in JSON
  # Key must be of type: (Text, enum)
  # Value can be of any type
  entries @0 :List(Node);
  struct Node {
   key @0 :Key;
   value @1 :Value;
  }
}
