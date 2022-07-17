// the code below is copied directly from https://github.com/frewsxcv/bevy-web-resizer
// with modifications in comments
#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use bevy::log;
    use bevy::prelude::*;
    use bevy::window::WindowId;
    use bevy::winit::WinitWindows;
    use crossbeam_channel::Receiver;
    use wasm_bindgen::JsCast;
    use winit::dpi::LogicalSize;

    pub struct Plugin;

    impl bevy::app::Plugin for Plugin {
        fn build(&self, app: &mut App) {
            app.add_system(web_resize_system);
        }
    }

    fn web_resize_system(winit_windows: Res<WinitWindows>, web_resizing: Local<WinitWebResizing>) {
        let winit_window = winit_windows.get_window(WindowId::primary()).unwrap();
        for size in web_resizing.rx.clone().try_iter().last() {
            winit_window.set_inner_size(size);
        }
    }

    impl Default for WinitWebResizing {
        fn default() -> Self {
            WinitWebResizing::new()
        }
    }

    pub struct WinitWebResizing {
        pub rx: Receiver<LogicalSize<f32>>,
    }

    impl WinitWebResizing {
        // on the creation of bevy
        pub fn new() -> Self {
            // create mpsc thingy, don't actually need crossbeam
            let (tx, rx) = crossbeam_channel::unbounded();

            // Get full size of window using websys. I guess mpsc
            let get_full_size = || {
                let win = web_sys::window().unwrap();
                // `inner_width` corresponds to the browser's `self.innerWidth` function, which are in
                // Logical, not Physical, pixels
                winit::dpi::LogicalSize::new(
                    win.inner_width().unwrap().as_f64().unwrap() as f32,
                    win.inner_height().unwrap().as_f64().unwrap() as f32,
                )
            };

            // kind of like an asynchronous request to js
            tx.send(get_full_size()).unwrap();

            // ??????
            // create a javascript function that takes in an event
            // upon resize, queue another producer to send new logical size down pipeline
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |e: web_sys::Event| {
                log::debug!("handling resize event: {:?}", e);
                tx.send(get_full_size()).unwrap();
            }) as Box<dyn FnMut(_)>);

            // add the resize event to the window
            let window = web_sys::window().unwrap();
            window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            // by default, the closure is invalidated after it is dropped in rust, but obviously for event listeners which we don't want to invalidate ever this is undesired
            // `closure.forget` relinquishes the control of the closure to javascript's gc
            closure.forget();

            return Self { rx };
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm32::Plugin;
