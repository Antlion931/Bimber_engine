use winit::{window::{Window, WindowBuilder}, event_loop::{EventLoopBuilder, ControlFlow, EventLoop, EventLoopProxy}, event::Event};

use crate::context::RenderingContext;

pub trait App<E> {
    fn handle_event(&mut self, ctx: &mut AppContext<E>, event: Event<E>, control_flow: &mut ControlFlow);
    fn setup(&mut self, ctx: &mut AppContext<E>);
}   

pub async fn launch<A : App<E> + 'static, E : 'static>(mut app : A) {
    env_logger::init();
    let event_loop: EventLoop<E> = EventLoopBuilder::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let r_ctx = RenderingContext::configure(&window).await;    

    let mut ctx = AppContext {window, proxy, r_ctx};

    app.setup(&mut ctx);

    event_loop.run(move |event, _, control_flow| {
        app.handle_event(&mut ctx, event, control_flow);
    });
}

pub struct AppContext<E : 'static> {
    pub window: Window,
    pub proxy: EventLoopProxy<E>,
    pub r_ctx: RenderingContext,
}


