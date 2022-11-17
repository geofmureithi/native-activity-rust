use jni::{
    objects::{JObject, JValue},
    signature::{Primitive, ReturnType},
    AttachGuard,
};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    let android_context = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(android_context.vm().cast()) }.unwrap();
    let env = vm.attach_current_thread().unwrap();

    let activity = ndk_glue::native_activity();
    let button = new_android_view("android/widget/Button", &env);
    let clazz = unsafe { JObject::from_raw(activity.activity() as jni::sys::jobject) };

    env.call_method(
        clazz,
        "setContentView",
        "(Landroid/view/View;)V",
        &[JValue::Object(button)],
    )
    .unwrap();
}

fn new_android_view<'a>(view: &'a str, env: &'a AttachGuard) -> JObject<'a> {
    let ctx = ndk_context::android_context();
    let class = env.find_class(view).unwrap();
    let init = env
        .get_method_id(*&class, "<init>", "(Landroid/content/Context;)V")
        .unwrap();

    let ctx = unsafe { JValue::Object(JObject::from_raw(ctx.context() as jni::sys::jobject)) };
    let obj = env.new_object_unchecked(*&class, init, &[ctx]).unwrap();
    obj
}
