//! Creates the window's event loop: plain on desktop, tied to the app on Android.
use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

#[cfg(target_os = "android")]
static APP: std::sync::OnceLock<winit::platform::android::activity::AndroidApp> =
    std::sync::OnceLock::new();

#[cfg(target_os = "android")]
pub fn set_android_app(app: winit::platform::android::activity::AndroidApp) {
    let _ = APP.set(app);
}

#[cfg(target_os = "android")]
pub fn new() -> Result<EventLoop<()>, EventLoopError> {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let app = APP.get().expect("android app not set").clone();
    EventLoop::builder().with_android_app(app).build()
}

#[cfg(not(target_os = "android"))]
pub fn new() -> Result<EventLoop<()>, EventLoopError> {
    EventLoop::new()
}
