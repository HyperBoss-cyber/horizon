#![cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[no_mangle]
fn android_main(app: AndroidApp) {
    let base = app.external_data_path().or_else(|| app.internal_data_path());
    if let Some(dir) = base {
        let _ = std::fs::create_dir_all(&dir);
        std::env::set_var("HORIZON_HOME", dir.join("horizon"));
        let _ = std::env::set_current_dir(&dir);
    }
    horizon_client::platform_loop::set_android_app(app);
    horizon_client::entry::main();
}
