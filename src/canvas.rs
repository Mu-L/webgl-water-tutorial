use crate::app::App;
use crate::app::Msg;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

// FIXME: Single responsibility
// FIXME: Split event attachments into functions
pub fn create_webgl_context(app: Rc<App>) -> Result<WebGlRenderingContext, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into()?;

    canvas.set_width(500);
    canvas.set_height(500);

    // Mouse down
    {
        let app = Rc::clone(&app);

        let on_mouse_down = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.client_x();
            let y = event.client_y();
            app.store.borrow_mut().msg(&Msg::MouseDown(x, y));
        }) as Box<FnMut(_)>);

        canvas.add_event_listener_with_callback(
            "mousedown",
            on_mouse_down.as_ref().unchecked_ref(),
        )?;

        on_mouse_down.forget();
    }

    // Mouse up
    {
        let app = Rc::clone(&app);

        let on_mouse_up = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            app.store.borrow_mut().msg(&Msg::MouseUp);
        }) as Box<FnMut(_)>);

        canvas.add_event_listener_with_callback("mouseup", on_mouse_up.as_ref().unchecked_ref())?;

        on_mouse_up.forget();
    }

    // Mouse move
    {
        let app = Rc::clone(&app);

        let on_mouse_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.client_x();
            let y = event.client_y();
            app.store.borrow_mut().msg(&Msg::MouseMove(x, y));
        }) as Box<FnMut(_)>);

        canvas.add_event_listener_with_callback(
            "mousemove",
            on_mouse_move.as_ref().unchecked_ref(),
        )?;

        on_mouse_move.forget();
    }

    // Mose out
    {
        let app = Rc::clone(&app);

        let on_mouse_out = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            app.store.borrow_mut().msg(&Msg::MouseOut);
        }) as Box<FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mouseout", on_mouse_out.as_ref().unchecked_ref())?;

        on_mouse_out.forget();
    }

    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);

    document.body().unwrap().append_child(&canvas)?;

    Ok(gl)
}
