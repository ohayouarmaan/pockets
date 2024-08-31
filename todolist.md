TODOs
[  ] 1. Create a language schema for communication
[  ] 2. Create a simple poc for the datastore to work.
[  ] 3. Create a query engine
[  ] 4. Create a language parser


1:
    * should be easy
    * should be a keyword based query language
    * eg: SET x = 5; GET x; DELETE x; GET t > t.regex("*_address_*").greater(5);
    * communication should also be signed using some sort of signing algorithm (i have to read more about it)

2.a:
    * Should be able to run in a distributed environment
    * Should have some datastructure for managing race conditions (probably a queue)
    * Should be a mixture of both in memory and file based data storage
    * Should be optimized for writes
2.b:
    * Should maintain some sort of architecure for recovery of lost data somehow
    * Shold also be able to handle sharding
