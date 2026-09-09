mod data;
mod render;
mod sim;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, KeyboardEvent, PointerEvent};

use data::{BODIES, CORE_PROFILE, CORE_PROJECTS};
use render::{canvas2d::Canvas2d, Renderer, Scene};
use sim::{CoreHit, Sim};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = orbitalSelect)]
    fn orbital_select(id: &str);
}

struct App {
    sim: Sim,
    renderer: Canvas2d,
    canvas: HtmlCanvasElement,
    hovered: Option<usize>,
    pointer: (f32, f32),
    running: bool,
    last: f64,
    dragging: bool,
    drag_last: (f32, f32),
    drag_start: (f32, f32),
    zoom: f32,
    zoom_target: f32,
    zoom_level: u8,
    overlay_open: bool,
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

fn toggle_class(class: &str, on: bool) {
    let Some(doc) = window().document() else { return };
    let Some(stage) = doc.get_element_by_id("stage") else { return };
    let cl = stage.class_list();
    if on { let _ = cl.add_1(class); } else { let _ = cl.remove_1(class); }
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
        self.sim.step(if self.running { dt } else { 0.0 });

        // Smooth zoom interpolation
        let diff = self.zoom_target - self.zoom;
        if diff.abs() > 0.01 {
            self.zoom += diff * 0.12;
        } else {
            self.zoom = self.zoom_target;
        }

        let scene = Scene {
            sim: &self.sim,
            selected: None,
            hovered: self.hovered,
            pointer: self.pointer,
            show_orbits: true,
            zoom: self.zoom,
            zoom_level: self.zoom_level,
        };
        self.renderer.draw(&scene);
    }

    fn open_overlay_body(&mut self, idx: usize) {
        render_body_overlay(idx);
        self.overlay_open = true;
        toggle_class("overlay-open", true);
        orbital_select(BODIES[idx].id);
    }

    fn open_overlay_project(&mut self, idx: usize) {
        render_project_overlay(idx);
        self.overlay_open = true;
        toggle_class("overlay-open", true);
        orbital_select(CORE_PROJECTS[idx].id);
    }

    fn open_overlay_profile(&mut self) {
        render_profile_overlay();
        self.overlay_open = true;
        toggle_class("overlay-open", true);
    }

    fn close_overlay(&mut self) {
        self.overlay_open = false;
        toggle_class("overlay-open", false);
    }

    fn zoom_in(&mut self) {
        self.zoom_level = 1;
        self.zoom_target = 1.0;
        toggle_class("zoomed", true);
    }

    fn zoom_out(&mut self) {
        self.zoom_level = 0;
        self.zoom_target = 0.0;
        toggle_class("zoomed", false);
    }
}

fn render_body_overlay(idx: usize) {
    let b = &BODIES[idx];
    let Some(doc) = window().document() else { return };
    let Some(el) = doc.get_element_by_id("ov-content") else { return };

    if let Some(ov) = doc.get_element_by_id("overlay") {
        let _ = ov.dyn_ref::<web_sys::HtmlElement>()
            .map(|h| h.style().set_property("--accent", b.kind.color()));
    }

    let bullets: String = b.bullets.iter().map(|x| format!("<li>{}</li>", x)).collect();
    let chips: String = b.chips.iter()
        .map(|c| format!("<span class=\"ov-chip\">{}</span>", c)).collect();

    el.set_inner_html(&format!(
        "<div class=\"ov-top\"><p class=\"ov-name\">{name}</p><span class=\"ov-when\">{when}</span></div>\
         <p class=\"ov-role\">{role}</p>\
         <ul class=\"ov-list\">{bullets}</ul>\
         <div class=\"ov-chips\">{chips}</div>",
        name = b.name, when = b.when, role = b.role,
        bullets = bullets, chips = chips,
    ));
}

fn render_project_overlay(idx: usize) {
    let cp = &CORE_PROJECTS[idx];
    let Some(doc) = window().document() else { return };
    let Some(el) = doc.get_element_by_id("ov-content") else { return };

    if let Some(ov) = doc.get_element_by_id("overlay") {
        let _ = ov.dyn_ref::<web_sys::HtmlElement>()
            .map(|h| h.style().set_property("--accent", cp.kind.color()));
    }

    let bullets: String = cp.bullets.iter().map(|x| format!("<li>{}</li>", x)).collect();
    let chips: String = cp.chips.iter()
        .map(|c| format!("<span class=\"ov-chip\">{}</span>", c)).collect();

    el.set_inner_html(&format!(
        "<div class=\"ov-top\"><p class=\"ov-name\">{name}</p><span class=\"ov-when\">{when}</span></div>\
         <p class=\"ov-role\">{role}</p>\
         <ul class=\"ov-list\">{bullets}</ul>\
         <div class=\"ov-chips\">{chips}</div>",
        name = cp.name, when = cp.when, role = cp.role,
        bullets = bullets, chips = chips,
    ));
}

fn render_profile_overlay() {
    let Some(doc) = window().document() else { return };
    let Some(el) = doc.get_element_by_id("ov-content") else { return };

    if let Some(ov) = doc.get_element_by_id("overlay") {
        let _ = ov.dyn_ref::<web_sys::HtmlElement>()
            .map(|h| h.style().set_property("--accent", "#EDEAE2"));
    }

    let edu: String = CORE_PROFILE.education.iter()
        .map(|e| format!("<li>{}</li>", e)).collect();
    let beliefs: String = CORE_PROFILE.beliefs.iter()
        .map(|b| format!("<li>{}</li>", b)).collect();

    el.set_inner_html(&format!(
        "<h2 class=\"ov-heading\">EDUCATION</h2>\
         <ul class=\"ov-list\">{edu}</ul>\
         <h2 class=\"ov-heading\">BELIEFS</h2>\
         <ul class=\"ov-list\">{beliefs}</ul>",
        edu = edu, beliefs = beliefs,
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
        hovered: None,
        pointer: (-9999.0, -9999.0),
        running: true,
        last: now(),
        dragging: false,
        drag_last: (0.0, 0.0),
        drag_start: (0.0, 0.0),
        zoom: 0.0,
        zoom_target: 0.0,
        zoom_level: 0,
        overlay_open: false,
    }));

    app.borrow_mut().resize();
    APP.with(|a| *a.borrow_mut() = Some(app.clone()));

    // resize
    {
        let cb = Closure::<dyn FnMut()>::new(move || with_app(|app| app.resize()));
        window().add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // overlay close button
    {
        let cb = Closure::<dyn FnMut()>::new(move || with_app(|app| app.close_overlay()));
        if let Some(btn) = document.get_element_by_id("ov-close") {
            btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())?;
        }
        cb.forget();
    }

    // Escape key — layered: overlay → zoom → nothing
    {
        let cb = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            if e.key() == "Escape" {
                with_app(|app| {
                    if app.overlay_open {
                        app.close_overlay();
                    } else if app.zoom_level > 0 {
                        app.zoom_out();
                    }
                });
            }
        });
        window().add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // pointer move → drag or hover
    {
        let target = canvas.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |e: PointerEvent| {
            let rect = target.get_bounding_client_rect();
            let x = e.client_x() as f32 - rect.left() as f32;
            let y = e.client_y() as f32 - rect.top() as f32;
            let cx = e.client_x() as f32;
            let cy = e.client_y() as f32;
            with_app(|app| {
                if app.dragging {
                    let dx = (cx - app.drag_last.0) / 200.0;
                    let dy = (cy - app.drag_last.1) / 300.0;
                    app.sim.rotate_camera(dx, -dy);
                    app.drag_last = (cx, cy);
                    return;
                }
                app.pointer = (x, y);
                app.hovered = app.sim.hit(x, y);
                let cursor = if app.hovered.is_some() { "pointer" } else { "grab" };
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
                app.dragging = false;
            });
        });
        canvas.add_event_listener_with_callback("pointerleave", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // pointer down
    {
        let canvas2 = canvas.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |e: PointerEvent| {
            let rect = canvas2.get_bounding_client_rect();
            let x = e.client_x() as f32 - rect.left() as f32;
            let y = e.client_y() as f32 - rect.top() as f32;
            let cx = e.client_x() as f32;
            let cy = e.client_y() as f32;
            with_app(|app| {
                app.drag_start = (cx, cy);

                if app.zoom_level == 0 {
                    if let Some(hit) = app.sim.core_hit(x, y) {
                        match hit {
                            CoreHit::Core => app.zoom_in(),
                            CoreHit::Project(_) => app.zoom_in(),
                        }
                    } else if let Some(i) = app.sim.hit(x, y) {
                        app.open_overlay_body(i);
                    } else {
                        app.dragging = true;
                        app.drag_last = (cx, cy);
                        let _ = app.canvas.style().set_property("cursor", "grabbing");
                    }
                } else {
                    // Zoom level 1
                    if let Some(hit) = app.sim.core_hit(x, y) {
                        match hit {
                            CoreHit::Core => app.open_overlay_profile(),
                            CoreHit::Project(i) => app.open_overlay_project(i),
                        }
                    } else {
                        app.dragging = true;
                        app.drag_last = (cx, cy);
                    }
                }
            });
        });
        canvas.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // pointer up — short click on empty space zooms out
    {
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |e: PointerEvent| {
            with_app(|app| {
                let cx = e.client_x() as f32;
                let cy = e.client_y() as f32;
                let dx = cx - app.drag_start.0;
                let dy = cy - app.drag_start.1;
                let dist = (dx * dx + dy * dy).sqrt();

                if app.dragging && dist < 5.0 && app.zoom_level > 0 {
                    app.zoom_out();
                }

                app.dragging = false;
                let _ = app.canvas.style().set_property("cursor", "grab");
            });
        });
        canvas.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref())?;
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

#[wasm_bindgen]
pub fn select_by_id(id: &str) {
    if let Some(i) = BODIES.iter().position(|b| b.id == id) {
        with_app(|app| app.open_overlay_body(i));
        return;
    }
    if let Some(i) = CORE_PROJECTS.iter().position(|cp| cp.id == id) {
        with_app(|app| {
            app.zoom_in();
            app.open_overlay_project(i);
        });
    }
}
