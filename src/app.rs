use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest, PossiblyCurrent, WindowedContext};

#[cfg(target_os = "android")]
use glutin::platform::android::AndroidContextExt;

use super::support;

struct ContextAndHandle {
    context: WindowedContext<PossiblyCurrent>,
    gl: support::Gl,
}

fn init_context(el: &EventLoopWindowTarget<()>) -> ContextAndHandle {
    let wb = WindowBuilder::new().with_title("A fantastic window!");
    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGlEs, (2, 0)))
        .with_gl_debug_flag(false)
        .build_windowed(wb, &el)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        context.get_pixel_format()
    );

    let gl = support::load(&context.context());

    ContextAndHandle { context, gl }
}

pub fn main() {
    let el = EventLoop::new();

    let mut context_and_handle = None;

    #[cfg(not(target_os = "android"))]
    {
        // on desktop platforms we should just create context when create window
        context_and_handle = init_context(&el).into();
    }

    el.run(move |event, el, control_flow| {
        // println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    if let Some(ContextAndHandle { context, .. }) = &context_and_handle {
                        context.resize(physical_size);
                    }
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                if let Some(ContextAndHandle { context, gl }) = &context_and_handle {
                    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    context.swap_buffers().unwrap();
                }
            }
            Event::Suspended => {
                #[cfg(target_os = "android")]
                context_and_handle.as_ref().unwrap().context.suspend();
            }
            Event::Resumed => {
                #[cfg(target_os = "android")]
                if context_and_handle.is_none() {
                    // on android we can create context only after start event
                    //
                    // (winit needs some reworks to do that in same manner on different platforms)
                    context_and_handle = init_context(el).into();
                }
                #[cfg(target_os = "android")]
                context_and_handle.as_ref().unwrap().context.resume();
            }
            _ => (),
        }
    });
}
