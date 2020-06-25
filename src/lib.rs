mod real_main;

#[cfg(target_os = "android")]
use crate::real_main::common_main;

#[cfg(target_os = "android")]
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace))]
fn main() {
    common_main();
}


