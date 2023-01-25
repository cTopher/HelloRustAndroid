# Hello Rust Android

## TODO: JNI in rust

```toml
[dependencies]
jni = "0.20"

[lib]
crate_type = ["cdylib"]
```

```rust
#[no_mangle]
pub extern "C" fn Java_com_theo_hello_Hello_greet(env: JNIEnv, _class: JClass, input: JString) -> jstring {
    let input: String = env.get_string(input).unwrap().into();
    let output = env.new_string(hello(&input)).unwrap();
    output.into_raw()
}
```