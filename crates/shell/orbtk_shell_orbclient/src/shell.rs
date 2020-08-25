use std::sync::mpsc;

use orbtk_shell::prelude::{ShellRequest, WindowAdapter, WindowSettings};
use spin_sleep::LoopHelper;

use super::{Window, WindowBuilder};

/// Represents an application shell that could handle multiple windows.
pub struct Shell<A: 'static>
where
    A: WindowAdapter,
{
    pub(crate) window_shells: Vec<Window<A>>,
    requests: mpsc::Receiver<ShellRequest<A>>,
    loop_helper: LoopHelper,
}

impl<A> Shell<A>
where
    A: WindowAdapter,
{
    /// Creates a new application shell.
    pub fn new(requests: mpsc::Receiver<ShellRequest<A>>) -> Self {
        Shell {
            window_shells: vec![],
            requests,
            loop_helper: LoopHelper::builder()
                .report_interval_s(0.5) // report every half a second
                .build_with_target_rate(70.0),
        }
    }

    /// Creates a window builder, that could be used to create a window and add it to the application shell.
    pub fn create_window(&mut self, adapter: A) -> WindowBuilder<A> {
        WindowBuilder::new(self, adapter)
    }

    /// Creates a window builder from a settings object.
    pub fn create_window_from_settings(
        &mut self,
        settings: WindowSettings,
        adapter: A,
    ) -> WindowBuilder<A> {
        WindowBuilder::from_settings(settings, self, adapter)
    }

    /// Receives window request from the application and handles them.
    pub fn receive_requests(&mut self) {
        let mut requests = vec![];
        for request in self.requests.try_iter() {
            requests.push(request);
        }

        for request in requests {
            if let ShellRequest::CreateWindow(adapter, settings, window_requests) = request {
                self.create_window_from_settings(settings, adapter)
                    .request_receiver(window_requests)
                    .build();
            }
        }
    }

    /// Runs (starts) the application shell and its windows.
    pub fn run(&mut self) {
        loop {
            if self.window_shells.is_empty() {
                return;
            }

            let _delta = self.loop_helper.loop_start();

            for i in 0..self.window_shells.len() {
                let mut remove = false;
                if let Some(window_shell) = self.window_shells.get_mut(i) {
                    window_shell.update();
                    window_shell.render();

                    window_shell.update_clipboard();
                    window_shell.drain_events();
                    window_shell.receive_requests();

                    if !window_shell.is_open() {
                        remove = true;
                    }
                }

                if remove {
                    self.window_shells.remove(i);
                    break;
                }
            }

            self.receive_requests();

            if let Some(fps) = self.loop_helper.report_rate() {
                if cfg!(feature = "debug") {
                    println!("fps: {}", fps);
                }
            }

            self.loop_helper.loop_sleep();
        }
    }
}
