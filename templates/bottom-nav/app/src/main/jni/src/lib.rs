use jni::{JNIEnv, objects::JObject};
use jni::sys::jstring;

#[unsafe(no_mangle)]
pub extern "C" fn Java_{{ jni_package }}_MainActivity_callRustHome(
    env: JNIEnv,
    _: JObject,
) -> jstring {
    let message = "Rust says: Home";
    let java_string = env.new_string(message).expect("Couldn't create java string!");
    java_string.into_inner()
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_{{ jni_package }}_MainActivity_callRustSearch(
    env: JNIEnv,
    _: JObject,
) -> jstring {
    let message = "Rust says: Search";
    let java_string = env.new_string(message).expect("Couldn't create java string!");
    java_string.into_inner()
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_{{ jni_package }}_MainActivity_callRustProfile(
    env: JNIEnv,
    _: JObject,
) -> jstring {
    let message = "Rust says: Profile";
    let java_string = env.new_string(message).expect("Couldn't create java string!");
    java_string.into_inner()
}
