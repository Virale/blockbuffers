var N = null;var searchIndex = {};
searchIndex["blockbuffers"]={"doc":"","items":[[0,"le","blockbuffers","",N,N],[8,"LE","blockbuffers::le","The trait `LE` converts between native endian and little endian.",N,N],[10,"to_le","","Converts a value in native endian to little endian.",0,[[["self"]],["self"]]],[10,"from_le","","Converts a value in little endian to native endian.",0,[[["self"]],["self"]]],[11,"from_le_slice","","Reads from slice in little endian form.",0,N],[0,"position","blockbuffers","",N,N],[3,"VectorPosition","blockbuffers::position","VectorPosition wrappers a position which points to a vector in the buffer.",N,N],[12,"0","","",1,N],[3,"StringPosition","","StringPosition wrappers a position which points to a string in the buffer.",N,N],[12,"0","","",2,N],[3,"VTablePosition","","VTablePosition wrappers a position which points to a vtable in the buffer.",N,N],[12,"0","","",3,N],[3,"TablePosition","","TablePosition wrappers a position which points to a table in the buffer.",N,N],[12,"0","","",4,N],[11,"clone","","",1,[[["self"]],["vectorposition"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"partial_cmp","","",1,[[["self"],["vectorposition"]],["option",["ordering"]]]],[11,"lt","","",1,[[["self"],["vectorposition"]],["bool"]]],[11,"le","","",1,[[["self"],["vectorposition"]],["bool"]]],[11,"gt","","",1,[[["self"],["vectorposition"]],["bool"]]],[11,"ge","","",1,[[["self"],["vectorposition"]],["bool"]]],[11,"eq","","",1,[[["self"],["vectorposition"]],["bool"]]],[11,"ne","","",1,[[["self"],["vectorposition"]],["bool"]]],[11,"len","","Reads the length of the vector.",1,N],[11,"as_slice","","Gets the reference to the items slice.",1,N],[11,"clone","","",2,[[["self"]],["stringposition"]]],[11,"fmt","","",2,[[["self"],["formatter"]],["result"]]],[11,"partial_cmp","","",2,[[["self"],["stringposition"]],["option",["ordering"]]]],[11,"lt","","",2,[[["self"],["stringposition"]],["bool"]]],[11,"le","","",2,[[["self"],["stringposition"]],["bool"]]],[11,"gt","","",2,[[["self"],["stringposition"]],["bool"]]],[11,"ge","","",2,[[["self"],["stringposition"]],["bool"]]],[11,"eq","","",2,[[["self"],["stringposition"]],["bool"]]],[11,"ne","","",2,[[["self"],["stringposition"]],["bool"]]],[11,"len","","Reads the length of the string in bytes.",2,N],[11,"as_str","","Gets the reference to the string.",2,N],[11,"clone","","",3,[[["self"]],["vtableposition"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"partial_cmp","","",3,[[["self"],["vtableposition"]],["option",["ordering"]]]],[11,"lt","","",3,[[["self"],["vtableposition"]],["bool"]]],[11,"le","","",3,[[["self"],["vtableposition"]],["bool"]]],[11,"gt","","",3,[[["self"],["vtableposition"]],["bool"]]],[11,"ge","","",3,[[["self"],["vtableposition"]],["bool"]]],[11,"eq","","",3,[[["self"],["vtableposition"]],["bool"]]],[11,"ne","","",3,[[["self"],["vtableposition"]],["bool"]]],[11,"vtable_bytes_len","","Reads the size of the vtable in bytes.",3,N],[11,"table_bytes_len","","Reads the size of the table in bytes.",3,N],[11,"field_offset","","Reads the field offset.",3,N],[11,"clone","","",4,[[["self"]],["tableposition"]]],[11,"fmt","","",4,[[["self"],["formatter"]],["result"]]],[11,"partial_cmp","","",4,[[["self"],["tableposition"]],["option",["ordering"]]]],[11,"lt","","",4,[[["self"],["tableposition"]],["bool"]]],[11,"le","","",4,[[["self"],["tableposition"]],["bool"]]],[11,"gt","","",4,[[["self"],["tableposition"]],["bool"]]],[11,"ge","","",4,[[["self"],["tableposition"]],["bool"]]],[11,"eq","","",4,[[["self"],["tableposition"]],["bool"]]],[11,"ne","","",4,[[["self"],["tableposition"]],["bool"]]],[11,"vtable","","Seeks the vtable position.",4,N],[11,"field_position","","Seeks the position for a field.",4,N],[0,"seek","blockbuffers","",N,N],[5,"seek_soffset","blockbuffers::seek","Reads a `SOffset` from `buf` at `pos`. Returns a new position by subtracting the read `SOffset` from `pos`.",N,N],[5,"seek_uoffset","","Reads a `UOffset` from `buf` at `pos`. Returns a new position by adding the read `UOffset` to `pos`.",N,N],[0,"types","blockbuffers","",N,N],[6,"UOffset","blockbuffers::types","Unsigned offset used for refernce to table, vector and string.",N,N],[6,"SOffset","","Signed offset used for vtable.",N,N],[6,"VOffset","","Unsigned offset used for field offset stored in vtable.",N,N],[6,"Len","","Length of vector and string.",N,N],[17,"SIZE_OF_UOFFSET","","",N,N],[17,"SIZE_OF_SOFFSET","","",N,N],[17,"SIZE_OF_VOFFSET","","",N,N],[17,"SIZE_OF_LEN","","",N,N],[14,"impl_le_for_enum","blockbuffers","The macro `impl_le_for_enum` implements trait `LE` for enum.",N,N]],"paths":[[8,"LE"],[3,"VectorPosition"],[3,"StringPosition"],[3,"VTablePosition"],[3,"TablePosition"]]};
initSearch(searchIndex);
