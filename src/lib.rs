//! Entry point and the JS boundary.
//!
//! Rust owns the canvas: simulation, rendering, hit-testing. The DOM owns the
//! detail panel, because canvas text is invisible to screen readers, crawlers
//! and text selection. The only thing crossing the boundary is a body id.

mod data;
mod render;
mod sim;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, PointerEvent};

use data::BODIES;
use render::{canvas2d::Canvas2d, Renderer, Scene};
use sim::Sim;

#[wasm_bindgen]
extern "C" {
    /// Defined in index.html. Receives the id of the newly locked body.
    #[wasm_bindgen(js_namespace = window, js_name = orbitalSelect)]
    fn orbital_select(id: &str);
}

struct App {
    sim: Sim,
    renderer: Canvas2d,
    canvas: HtmlCanvasElement,
    selected: Option<usize>,
    hovered: Option<usize>,
    pointer: (f32, f32),
    show_orbits: bool,
    running: bool,
    last: f64,
}

thread_local! {
    static APP: RefCell<Option<Rc<RefCell<App>>>> = RefCell::new(None);
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no window")
}

fn now() -> f64 {
    window().performance().map(|p| p.now()).unwrap_or(0.0)
}

impl App {
    fn resize(&mut self) {
        let rect = self.canvas.get_bounding_client_rect();
        let (w, h) = (rect.width() as f32, rect.height() as f32);
        let dpr = window().device_pixel_ratio().min(2.0) as f32;
        self.canvas.set_width((w * dpr) as u32);
        self.canvas.set_height((h * dpr) as u32);
        self.sim.resize(w, h);
        self.renderer.resize(w, h, dpr);
    }

    fn frame(&mut self, t: f64) {
        let dt = ((t - self.last) / 1000.0).min(0.05) as f32;
        self.last = t;
        if self.running {
            self.sim.step(dt);
        }
        let scene = Scene {
            sim: &self.sim,
            selected: self.selected,
            hovered: self.hovered,
            pointer: self.pointer,
            show_orbits: self.show_orbits,
        };
        self.renderer.draw(&scene);
    }

    fn select(&mut self, idx: usize) {
        self.selected = Some(idx);
        render_panel(idx);
        orbital_select(BODIES[idx].id);
    }
}

/// Writes the detail panel into the DOM. Content is static and authored in
/// `data.rs`, so `set_inner_html` is safe here — never feed it fetched data.
fn render_panel(idx: usize) {
    let b = &BODIES[idx];
    let Some(doc) = window().document() else { return };
    let Some(el) = doc.get_element_by_id("readout") else { return };

    let bullets: String = b.bullets.iter().map(|x| format!("<li>{}</li>", x)).collect();
    let chips: String = b
        .chips
        .iter()
        .map(|c| format!("<span class=\"chip\">{}</span>", c))
        .collect();

    let _ = el.set_attribute("style", &format!("--accent:{}", b.kind.color()));
    el.set_inner_html(&format!(
        "<div class=\"r-top\"><p class=\"r-name\">{name}</p><span class=\"r-when\">{when}</span></div>\
         <p class=\"r-role\">{role}</p>\
         <ul class=\"r-list\">{bullets}</ul>\
         <div class=\"chips\">{chips}</div>",
        name = b.name,
        when = b.when,
        role = b.role,
        bullets = bullets,
        chips = chips,
    ));
}

fn with_app<F: FnOnce(&mut App)>(f: F) {
    APP.with(|a| {
        if let Some(app) = a.borrow().as_ref() {
            f(&mut app.borrow_mut());
        }
    });
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let document = window().document().ok_or("no document")?;
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("sky")
        .ok_or("missing #sky canvas")?
        .dyn_into()?;

    let renderer = Canvas2d::new(&canvas)?;
    let app = Rc::new(RefCell::new(App {
        sim: Sim::new(),
        renderer,
        canvas: canvas.clone(),
        selected: Some(0),
        hovered: None,
        pointer: (-9999.0, -9999.0),
        show_orbits: true,
        running: true,
        last: now(),
    }));

    app.borrow_mut().resize();
    orbital_select(BODIES[0].id);
    APP.with(|a| *a.borrow_mut() = Some(app.clone()));

    // resize
    {
        let cb = Closure::<dyn FnMut()>::new(move || with_app(|app| app.resize()));
        window().add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // pointer move → hover + belt proximity
    {
        let target = canvas.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |e: PointerEvent| {
            let rect = target.get_bounding_client_rect();
            let x = e.client_x() as f32 - rect.left() as f32;
            let y = e.client_y() as f32 - rect.top() as f32;
            with_app(|app| {
                app.pointer = (x, y);
                app.hovered = app.sim.hit(x, y);
                let cursor = if app.hovered.is_some() { "pointer" } else { "default" };
                let _ = app.canvas.style().set_property("cursor", cursor);
            });
        });
        canvas.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // pointer leave
    {
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |_| {
            with_app(|app| {
                app.hovered = None;
                app.pointer = (-9999.0, -9999.0);
            });
        });
        canvas.add_event_listener_with_callback("pointerleave", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // pointer down → lock
    {
        let canvas2 = canvas.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |e: PointerEvent| {
            let rect = canvas2.get_bounding_client_rect();
            let x = e.client_x() as f32 - rect.left() as f32;
            let y = e.client_y() as f32 - rect.top() as f32;
            with_app(|app| {
                if let Some(i) = app.sim.hit(x, y) {
                    app.select(i);
                }
            });
        });
        canvas.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // render loop
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move |t: f64| {
        with_app(|app| app.frame(t));
        let _ = window().request_animation_frame(
            f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        );
    }));
    window().request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}

// ---- controls called from the HTML buttons ----

#[wasm_bindgen]
pub fn set_speed(speed: f32) {
    with_app(|app| app.sim.speed = speed);
}

#[wasm_bindgen]
pub fn toggle_running() -> bool {
    let mut state = false;
    with_app(|app| {
        app.running = !app.running;
        app.last = now();
        state = app.running;
    });
    state
}

#[wasm_bindgen]
pub fn toggle_orbits() -> bool {
    let mut state = false;
    with_app(|app| {
        app.show_orbits = !app.show_orbits;
        state = app.show_orbits;
    });
    state
}

/// Lock a body by id — used for deep links (`/#lia`).
#[wasm_bindgen]
pub fn select_by_id(id: &str) {
    if let Some(i) = BODIES.iter().position(|b| b.id == id) {
        with_app(|app| app.select(i));
    }
}
