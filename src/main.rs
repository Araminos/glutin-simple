fn main() {
    #[cfg(not(target_os = "android"))]
    glutin_simple::main();
}
