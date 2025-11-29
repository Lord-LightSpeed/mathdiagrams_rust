use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    // let r = app.window_rect();
    let window = app.main_window();
    let win = window.rect();
    draw.background().color(GREEN);
    let crosshair_color = gray(0.5);

//     for r in r.subdivisions_iter() {
//         for r in r.subdivisions_iter() {
//             for r in r.subdivisions_iter() {
//                 let side = r.w().min(r.h());
//                 let start = r.xy();
//                 let start_to_mouse = app.mouse.position() - start;
//                 let target_mag = start_to_mouse.length().min(side * 0.5);
//                 let end = start + start_to_mouse.normalize_or_zero().rotate(PI / 2.0) * target_mag;
//                 draw.arrow().weight(5.0).points(start, end);
//             }
//         }
//     }

    let arrowcount = 5;
    let farowcount = (arrowcount * 2 + 1).to_f32().unwrap();

    for xval in (-1 * arrowcount)..=arrowcount {
        for yval in (-1 * arrowcount)..=arrowcount {
            let w = win.w();
            let h = win.h();
            let dw = w / farowcount;
            let dh = h / farowcount;
            let xf = xval.to_f32().unwrap();
            let yf = yval.to_f32().unwrap();
            let side = dw.min(dh);
            let start: Point2 = [xf * dw,  yf * dh].into();
            let start_to_mouse = app.mouse.position() - start;
            let target_mag = start_to_mouse.length().min(side * 0.5);
            let end = start + start_to_mouse.normalize_or_zero().rotate(PI / 2.0) * target_mag;
            draw.arrow().weight(5.0).points(start, end);
        }
    }

    if let Some(monitor) = window.current_monitor() {
        let w_scale_factor = window.scale_factor();
        let m_scale_factor = monitor.scale_factor();
        let mon_phys = monitor.size();
        let mon = mon_phys.to_logical(w_scale_factor as f64);
        let mon_w: f32 = mon.width;
        let mon_h: f32 = mon.height;
        let text = format!(
            "
        Window size: [{:.0}, {:.0}]
        Window ratio: {:.2}
        Window scale factor: {:.2}
        Monitor size: [{:.0}, {:.0}]
        Monitor ratio: {:.2}
        Monitor scale factor: {:.2}
        ",
            win.w(),
            win.h(),
            win.w() / win.h(),
            w_scale_factor,
            mon_w,
            mon_h,
            mon_w / mon_h,
            m_scale_factor
        );
        let pad = 6.0;
        draw.text(&text)
            .h(win.pad(pad).h())
            .w(win.pad(pad).w())
            .line_spacing(pad)
            .font_size(14)
            .align_text_bottom()
            .color(crosshair_color)
            .left_justify();

        // Ellipse at mouse.
        draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

        // Mouse position text.
        let mouse = app.mouse.position();
        let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
        draw.text(&pos)
            .xy(mouse + vec2(0.0, 20.0))
            .font_size(14)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
