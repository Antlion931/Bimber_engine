
use bimber_renderer::{app::{launch, App, AppContext}, context::RenderingContext};
use pollster::block_on;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

struct MyApp {
    clear_col: wgpu::Color,
    t: f64,
}

impl MyApp {
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn render(&mut self, rtx: &mut RenderingContext) -> Result<(), wgpu::SurfaceError> {
        let output = rtx.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = rtx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_col),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
    
        rtx.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }

    fn update(&mut self) {
        use std::f64::consts::PI;
        
        self.clear_col.r = (f64::sin(self.t)            + 1.) / 2.; 
        self.clear_col.g = (f64::sin(self.t + PI / 2.)  + 1.) / 2.; 
        self.clear_col.b = (f64::sin(self.t + PI)       + 1. ) / 2.; 
        self.t += 0.001
    }
    
}

impl App<()> for MyApp {
    fn handle_event(
        &mut self,
        ctx: &mut AppContext<()>,
        event: Event<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == ctx.window.id() => {
                if !self.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            ctx.r_ctx.resize_surface(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            ctx.r_ctx.resize_surface(**new_inner_size);
                        }
                        
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == ctx.window.id() => {
                self.update();
                match self.render(&mut ctx.r_ctx) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => ctx.r_ctx.resize_surface(ctx.r_ctx.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                ctx.window.request_redraw();
            }
            _ => {}
        }
    }

    fn setup(&mut self, ctx: &mut AppContext<()>) {}
}

fn main() {
    let app = MyApp { clear_col: wgpu::Color::WHITE, t: 0.0};
    block_on(launch(app));
}
