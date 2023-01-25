use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

#[no_mangle]
pub extern "C" fn Java_com_theo_hello_Hello_greet(env: JNIEnv, _class: JClass, input: JString) -> jstring {
    let input = String::from(env.get_string(input).unwrap());
    let result = hello(&input);
    env.new_string(result).unwrap().into_raw()
}

pub fn hello(name: &str) -> String {
    format!("Hello, {name}! Greetings from Ferris.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hello("Christopher");
        assert_eq!(result, "Hello, Christopher! Greetings from Ferris.");
    }
}
