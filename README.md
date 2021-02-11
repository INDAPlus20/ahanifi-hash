# ahanifi-hash
JaySonDB

Database built upon a hashmap that implements open addressing using robin hood hashing. The probing sequence is linear.

To start the CLI for the database navigate to the database folder in the terminal and write ```cargo run``` to run with the default save file. To start the database with another file write ```cargo run filepath```.

The database supports any type that implement the following traits Serialize, Deserialize, Hash and Eq for the key and Serialize, Deserialize for the value. The hashmap supports any type that implement Hash and Eq for the key. The value has no restrictions on its type.
