mod real_main;

use crate::real_main::common_main;

// #[cfg(not(target_os = "android"))] - если оставить то андроид не работает
fn main() {
    common_main();
}
