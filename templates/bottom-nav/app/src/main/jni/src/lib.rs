use jni::{JNIEnv, objects::{JObject, JString}};
use jni::sys::jstring;

#[unsafe(no_mangle)]
pub extern "C" fn Java_{{ jni_package }}_MainActivity_greetFromRust(
    mut env: JNIEnv,
    _: JObject,
    screen: JString,
) -> jstring {
    let screen_name: String = env.get_string(&screen)
        .expect("Couldn't get screen string")
        .into();

    let message = format!("Hello from Rust! Screen: {}", screen_name);
    let java_string = env.new_string(message).expect("Couldn't create java string");
    java_string.into_raw()
}
