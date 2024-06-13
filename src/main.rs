fn main() {
    // The code below fails to compile:
    //use surrealdb::sql::Id::uuid;
    //let unique_id = uuid();
    //dbg!(&unique_id);

    // The reason is the uuid() function is a method on an enum, not
    // a standalone function at the global scope. It can be called
    // using the full path to the method or by importing the enum.
    //
    // Here is the error message:
    //   $ cargo run
    //      Compiling exper-rust-enum-struct-methods v0.1.0 (/home/wink/prgs/SurrealDB/exper-rust-enum-struct-methods)
    //   error[E0432]: unresolved import `surrealdb::sql::Id::uuid`
    //    --> src/main.rs:6:9
    //     |
    //   6 |     use surrealdb::sql::Id::uuid;
    //     |         ^^^^^^^^^^^^^^^^^^^^^^^^ no `uuid` in `sql::id::Id`
    //   
    //   For more information about this error, try `rustc --explain E0432`.
    //   error: could not compile `exper-rust-enum-struct-methods` (bin "exper-rust-enum-struct-methods") due to 1 previous error
    //

    // Here is an example using the full path
    let unique_id_2 = surrealdb::sql::Id::uuid();
    dbg!(&unique_id_2);

    // Here is an example using the import of the enum Id
    // which is defined at the global scope.
    use surrealdb::sql::Id;
    let unique_id_1 = Id::uuid();
    dbg!(&unique_id_1);
}
