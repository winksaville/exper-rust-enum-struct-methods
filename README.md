# Experiment rust enum and struct methods

While learning surrealdb I tried to use surrealdb::sql::Id::uuid() function
to initialize the RecordId field of a record and got a compiler error.

I got the explanation from ChatGPT4o in
[this conversation](https://chatgpt.com/share/230354e4-fea9-4183-92cd-7c834e5613fa)
and created this repo to preserve explation and code.

```rust
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
```

Here is the output:
```shell
$ cargo run
   Compiling exper-rust-enum-struct-methods v0.1.0 (/home/wink/prgs/SurrealDB/exper-rust-enum-struct-methods)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.47s
     Running `target/debug/exper-rust-enum-struct-methods`
[src/main.rs:26:5] &unique_id_2 = String(
    "019013e2-80f9-7560-89d4-9651d0b45f6f",
)
[src/main.rs:32:5] &unique_id_1 = String(
    "019013e2-80f9-7a34-ba79-063b133594d8",
)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
