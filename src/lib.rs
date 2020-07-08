mod app;
mod support;

#[cfg(not(target_os = "android"))]
pub use app::main;

#[cfg(target_os = "android")]
#[ndk_glue::main(backtrace)]
fn main() {
    app::main();
}
