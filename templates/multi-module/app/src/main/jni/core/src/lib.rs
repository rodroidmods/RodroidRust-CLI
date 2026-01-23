use jni::{JNIEnv, objects::JObject};
use jni::sys::jstring;

#[unsafe(no_mangle)]
pub extern "C" fn Java_{{ jni_package }}_MainActivity_callCore(
    env: JNIEnv,
    _: JObject,
) -> jstring {
    let message = "Hello from Rust (core)";
    let java_string = env.new_string(message).expect("Couldn't create java string!");
    java_string.into_inner()
}

