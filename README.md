# Cross platform glutin example

An attempt to create a demonstration project that:
1. Cross platform (Windows, Macos, Linux, Android, iOs)
2. Can manage windows on the target system and handle events 
3. Opengl

This project relies on:
1. [winit](https://github.com/rust-windowing/winit) - for managing windows and handling events
2. [glutin](https://github.com/rust-windowing/glutin) - for OpenGL context creation
3. [gl_generator](https://github.com/brendanzab/gl-rs) - for creating bindings to the OpenGL API 
4. [android-ndk-rs](https://github.com/rust-windowing/android-ndk-rs) - for building on Android

The example was grabbed from [glutin example](https://github.com/rust-windowing/glutin/blob/master/glutin_examples/examples/window.rs) 
Example was transformed according [android-ndk-rs](https://github.com/rust-windowing/android-ndk-rs/tree/master/ndk-examples)
to have ability to run on Android and desktops. 