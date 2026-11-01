#![cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[no_mangle]
fn android_main(app: AndroidApp) {
    if let Some(dir) = app.internal_data_path() {
        std::env::set_var("HORIZON_HOME", dir.join("horizon"));
    }
    horizon_client::platform_loop::set_android_app(app);
    horizon_client::entry::main();
}
