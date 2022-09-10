use eframe::{egui::*, NativeOptions};
use native_dialog::FileDialog;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
// use std::thread::current;
// use std::time::Instant;
// use egui::plot::{Line, Plot, Values, Value};
struct Path {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    d1: f32,
    d2: f32,
    rnd_end1: bool,
    rnd_end2: bool,
    hori: bool,
    layer: i8,
}
impl Path {
    fn default(x: f32, y: f32) -> Path {
        Path {
            x,
            y,
            w: 300.,
            h: 42.0,
            d1: 0.3,
            d2: 1.0,
            rnd_end1: true,
            rnd_end2: true,
            hori: true,
            layer: 1,
        }
    }
}
struct Input {
    n: bool,
    p: bool,
    click: bool,
    d: bool,
    dx: f32,
    dy: f32,
}
fn main() {
    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2 { y: 720., x: 1280. });
    native_options.initial_window_pos = Some(Pos2 { y: 0., x: 0. });
    eframe::run_native(
        "RevomApp",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
enum GridStatus {
    Off,
    OnButInvisible,
    OnAndVisible,
}
// #[derive(Default)]
struct App {
    curr_path: Option<usize>,
    paths: Vec<Path>,
    input: Input,
    // now: Instant,
    grid_size: f32,
    grid_status: GridStatus,
    min: Pos2,
    max: Pos2,
    move_path: bool,
    adjust_w: i8,
    adjust_h: bool,
    // click: Pos2,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            curr_path: Some(0),
            paths: vec![Path::default(209., 196.)],
            input: Input {
                n: false,
                p: false,
                click: false,
                d: false,
                dx: 0.,
                dy: 0.,
            },
            // now: Instant::now(),
            grid_size: 21.,
            grid_status: GridStatus::OnAndVisible,
            // click: Pos2 { x: 0., y: 0. },
            min: Pos2 { x: 20., y: 70. },
            max: Pos2 { x: 1133., y: 658. },
            move_path: false,
            adjust_w: 0,
            adjust_h: false,
        }
    }
    fn delete_path(&mut self) {
        match self.curr_path {
            None => (),
            Some(curr_path) => {
                if self.paths.len() > 1 {
                    self.paths.remove(curr_path);
                    self.curr_path = Some(0);
                }
            }
        }
    }
    fn map_to_string(&self) -> String {
        let mut res = String::new();
        // let mut res = self.paths.len().to_string();
        // res.push(':');
        for p in &self.paths {
            res.push_str(&p.x.to_string());
            res.push(',');
            res.push_str(&p.y.to_string());
            res.push(',');
            res.push_str(&p.w.to_string());
            res.push(',');
            res.push_str(&p.h.to_string());
            res.push(',');
            res.push_str(&p.d1.to_string());
            res.push(',');
            res.push_str(&p.d2.to_string());
            res.push(',');
            res.push_str(&p.rnd_end1.to_string());
            res.push(',');
            res.push_str(&p.rnd_end2.to_string());
            res.push(',');
            res.push_str(&p.hori.to_string());
            res.push(',');
            res.push_str(&p.layer.to_string());
            res.push(';');
        }
        res.pop();
        res
    }
    fn export_map(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;
        let map = self.map_to_string();
        write!(output, "{}", map)?;
        Ok(())
    }
    fn import_map(&mut self, file_name: &str) -> Result<(), Error> {
        let input = File::open(file_name)?;
        self.curr_path = None;
        let buffered = BufReader::new(input);
        for line in buffered.lines() {
            self.string_to_map(&line?);
        }
        Ok(())
    }
    fn string_to_map(&mut self, string: &str) {
        let mut paths: Vec<Path> = Vec::new();
        let path_strings: Vec<&str> = string.split(';').collect();
        for i in path_strings {
            let attributes: Vec<&str> = i.split(',').collect();
            // println!("{:?}", attributes);
            let path = Path {
                x: attributes[0].parse::<f32>().unwrap(),
                y: attributes[1].parse::<f32>().unwrap(),
                w: attributes[2].parse::<f32>().unwrap(),
                h: attributes[3].parse::<f32>().unwrap(),
                d1: attributes[4].parse::<f32>().unwrap(),
                d2: attributes[5].parse::<f32>().unwrap(),
                rnd_end1: attributes[6].parse::<bool>().unwrap(),
                rnd_end2: attributes[7].parse::<bool>().unwrap(),
                hori: attributes[8].parse::<bool>().unwrap(),
                layer: attributes[9].parse::<i8>().unwrap(),
            };
            paths.push(path);
        }
        self.paths = paths;
    }
    fn load_file_location(&mut self) {
        let path = FileDialog::new()
            .add_filter("RevomApp Map", &["revomap"])
            .show_open_single_file()
            .unwrap();

        let path = match path {
            Some(path) => path,
            None => return,
        };
        let path = path.to_str().unwrap();
        self.import_map(path).expect("");
    }
    fn write_to_file_location(&mut self) {
        let path = FileDialog::new()
            .add_filter("RevomApp Map", &["revomap"])
            .show_save_single_file()
            .unwrap();
        let path = match path {
            Some(path) => path,
            None => return,
        };
        let path = path.to_str().unwrap();
        self.export_map(path).expect("");
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // let time = format!("{}", 1000000 / self.now.elapsed().as_micros());
        // self.now = Instant::now();
        CentralPanel::default().show(&ctx, |ui| {
            ui.horizontal(|ui| {
                match self.grid_status {
                    GridStatus::OnAndVisible => {
                        if ui.button("Grid: Visible").clicked() {
                            self.grid_status = GridStatus::OnButInvisible;
                        }
                        ui.add(Slider::new(&mut self.grid_size, 7.5..=30.).text("grid size"));
                    }
                    GridStatus::OnButInvisible => {
                        if ui.button("Grid: Invisible").clicked() {
                            self.grid_status = GridStatus::Off;
                        }
                        ui.add(Slider::new(&mut self.grid_size,  7.5..=30.).text("grid size"));
                    }
                    GridStatus::Off => {
                        self.grid_size = 1.;
                        if ui.button("Grid: Off").clicked() {
                            self.grid_size = 15.;
                            self.grid_status = GridStatus::OnAndVisible;
                        }
                    }
                }
                if ui.button("export map").clicked() {
                    self.write_to_file_location();
                }
                if ui.button("import map").clicked() {
                    self.load_file_location();
                }
            });
            ui.separator();
            match self.curr_path {
                None => (),
                Some(curr_path) => {
                    ui.horizontal(|ui| {
                        if ui.button("rotate").clicked() {
                            self.paths[curr_path].hori = !self.paths[curr_path].hori;
                            let w = self.paths[curr_path].w;
                            self.paths[curr_path].w = self.paths[curr_path].h;
                            self.paths[curr_path].h = w;
                        }
                        ui.separator();
                        ui.add(Slider::new(&mut self.paths[curr_path].d1, 0.0..=1.0).text("d1"));
                        ui.separator();
                        ui.add(Slider::new(&mut self.paths[curr_path].d2, 0.0..=1.0).text("d2"));
                        ui.separator();
                        ui.label("x:");
                        ui.add(DragValue::new(&mut self.paths[curr_path].x));
                        ui.separator();
                        ui.label("y:");
                        ui.add(DragValue::new(&mut self.paths[curr_path].y));
                        ui.separator();
                        ui.label("width:");
                        ui.add(DragValue::new(&mut self.paths[curr_path].w));
                        ui.separator();
                        ui.label("height:");
                        ui.add(DragValue::new(&mut self.paths[curr_path].h));
                        ui.separator();
                        if ui.button("rounded end 1").clicked() {
                            self.paths[curr_path].rnd_end1 = !self.paths[curr_path].rnd_end1;
                        }
                        if ui.button("rounded end 2").clicked() {
                            self.paths[curr_path].rnd_end2 = !self.paths[curr_path].rnd_end2;
                        }
                        ui.separator();
                        ui.label("layer:");
                        ui.add(Slider::new(&mut self.paths[curr_path].layer, 1..=10));
                        // ui.label(time);
                    });
                }
            }

            if ui.input().key_down(Key::P) {
                if self.input.p {
                    let pos = ui.input().pointer.hover_pos();
                    let pos = match pos {
                        None => Pos2 { x: 209., y: 196. },
                        Some(pos) => pos,
                    };
                    self.paths.push(Path::default(pos.x, pos.y));
                    self.curr_path = Some(self.paths.len() - 1);
                    // println!("{}", self.paths.len());
                }
                self.input.p = false;
            } else {
                self.input.p = true;
            }
            if ui.input().key_down(Key::D) {
                // todo just a bookmark for key inputs
                if self.input.d {
                    self.delete_path();
                }
                self.input.d = false;
            } else {
                self.input.d = true;
            }
            if ui.input().key_down(Key::N) {
                if self.input.n {
                    if self.curr_path == Some(self.paths.len() - 1) {
                        self.curr_path = Some(0);
                    } else {
                        match self.curr_path {
                            None => {
                                self.curr_path = Some(0);
                            },
                            Some(curr_path) => {
                                self.curr_path = Some(curr_path + 1);
                            }
                        }
                    }
                }
                self.input.n = false;
            } else {
                self.input.n = true;
            }
            if ui.input().scroll_delta.y > 0. {
                let move_distance;
                if self.grid_size < 2. {
                    move_distance = 21f32;
                } else {
                    move_distance = self.grid_size;
                }
                for p in &mut self.paths {
                    p.x += move_distance;
                }
            }

            if ui.input().scroll_delta.y < 0. {
                let move_distance;
                if self.grid_size < 2. {
                    move_distance = 21f32;
                } else {
                    move_distance = self.grid_size;
                }
                for p in &mut self.paths {
                    p.x -= move_distance;
                }
            }
            let mut cursor_icon = CursorIcon::default();
            if let Some(pos) = { ctx.input().pointer.hover_pos() } {
                let mut i = -1f32;
                while i < 2. {
                    if clicked_move_circle(self, pos, i * (self.max.x - self.min.x)) || self.move_path {
                        cursor_icon = CursorIcon::AllScroll;
                    } else if clicked_w_circle(self, pos, i * (self.max.x - self.min.x)) || self.adjust_w != 0 {
                        cursor_icon = CursorIcon::ResizeHorizontal;
                    } else if clicked_h_circle(self, pos, i * (self.max.x - self.min.x)) || self.adjust_h {
                        cursor_icon = CursorIcon::ResizeVertical;

                    } else {
                    }
                    i += 1.;
                }
            }
            ctx.output().cursor_icon = cursor_icon;
            if ui.input().pointer.button_down(PointerButton::Primary) {
                let mouse_pos = ui.input().pointer.hover_pos();
                match mouse_pos {
                    None => (),
                    Some(pos) => {
                        if pos.y > self.min.y {
                            if self.input.click {
                                // println!("{}", clicked_path(self, pos));
                                let mut i = -1f32;
                                while i < 2. {
                                    if clicked_move_circle(self, pos, i * (self.max.x - self.min.x)) {
                                        self.move_path = true;
                                        break;
                                    } else if clicked_w_circle(self, pos, i * (self.max.x - self.min.x)) {
                                        self.adjust_w = 1 - i as i8;
                                    } else if clicked_h_circle(self, pos, i * (self.max.x - self.min.x)) {
                                        self.adjust_h = true;
                                    } else {
                                        // self.click = click_pos;
                                        self.input.click = false;
                                    }
                                    i += 1.;
                                }
                                if !self.move_path && self.adjust_w == 0 && !self.adjust_h {
                                    let mut i = -1f32;
                                    let mut all_found_paths = [None, None, None];
                                    while i < 2. {
                                        let new_path = clicked_path(self, pos, i * (self.max.x - self.min.x));
                                        if new_path != self.curr_path && !new_path.is_none() {
                                            all_found_paths[(i + 1.) as usize] = new_path;
                                        } else {
                                            all_found_paths[(i + 1.) as usize] = None;
                                        }
                                        // match self.curr_path {
                                        //     None => (),
                                        //     Some(curr_path) => {
                                        //         if new_path != Some(curr_path) {
                                        //             // self.curr_path = new_path; // todo switch between paths idk if it always works
                                        //         }
                                        //     }
                                        // }
                                        i += 1.;
                                    }
                                    if all_found_paths[2].is_none() {
                                        if all_found_paths[1].is_none() {
                                            self.curr_path = all_found_paths[0];
                                        } else {
                                            self.curr_path = all_found_paths[1];
                                        }
                                    } else {
                                        self.curr_path = all_found_paths[2];
                                    }
                                }
                                // if clicked_path(self, pos)
                                // {
                                //     self.paths[self.curr_path].x = pos.x
                                //         - self.paths[self.curr_path].w / 2.
                                //         - (pos.x - 20.) % self.grid_size
                                //         + self.grid_size / 2.;
                                //     self.paths[self.curr_path].y = pos.y
                                //         - self.paths[self.curr_path].h / 2.
                                //         - (pos.y - 70.) % self.grid_size
                                //         + self.grid_size / 2.;
                                // }
                                self.input.click = false;
                            } 
                        } else {
                                self.input.click = false;
                            }
                    }
                }
            } else {
                self.input.click = true;
                self.move_path = false;
                self.adjust_w = 0;
                self.adjust_h = false;
            }
            // if ui.input().pointer.button_down(PointerButton::Primary) {
            //     let click_pos = ui.input().pointer.hover_pos();
            //     match click_pos {
            //         None => (),
            //         Some(click_pos) => {
            //             if self.input.click
            //                 && click_pos.x == self.input.dx
            //                 && click_pos.y == self.input.dy
            //             {
            //                 mutate_clicked_path(self, click_pos);
            //                 // self.click = click_pos;
            //                 self.input.click = false;
            //             }
            //         }
            //     }
            // } else {
            //     self.input.click = true;
            // }
            let mouse_pos = ui.input().pointer.hover_pos();
            match mouse_pos {
                None => (),
                Some(pos) => {
                    match self.curr_path {
                        None => (),
                        Some(curr_path) => {
                            let p = &mut self.paths[curr_path];
                            if self.move_path {
                                p.x = pos.x - (pos.x - 20. + self.grid_size / 2.) % self.grid_size
                                    + self.grid_size / 2.;
                                p.y = pos.y - (pos.y - 70. + self.grid_size / 2.) % self.grid_size
                                    + self.grid_size / 2.;
                            } else if self.adjust_w != 0 {
                                p.w = -p.x + pos.x
                                    - (pos.x - 20. + self.grid_size / 2.) % self.grid_size
                                    + self.grid_size / 2.
                                    - p.h / 2. * p.hori as u8 as f32 * p.rnd_end2 as u8 as f32 + (self.adjust_w - 1) as f32 * (self.max.x - self.min.x);
                                if p.w < 10. {
                                    p.w = 10.
                                }
                            } else if self.adjust_h {
                                p.h = -p.y + pos.y
                                    - (pos.y - 70. + self.grid_size / 2.) % self.grid_size
                                    + self.grid_size / 2.
                                    - p.w / 2. * !p.hori as u8 as f32 * p.rnd_end2 as u8 as f32;
                                if p.h < 10. {
                                    p.h = 10.
                                }
                            }
                        }
                    }

                    self.input.dx = pos.x;
                    self.input.dy = pos.y;
                }
            }
            draw(&self, &ui);
        });
        for p in &mut self.paths {
            if p.x < self.min.x {
                p.x += self.max.x - self.min.x;
            } else if p.x > self.max.x {
                p.x -= self.max.x - self.min.x;
            }
        }
    }
}
fn draw(app: &App, ui: &Ui) {
    // let mut debug_max = app.max;
    // debug_max.x += 500.;
    // let mut debug_min = app.min;
    // debug_min.x -= 20.;
    // let painter = ui.painter_at(Rect {
    //     min: debug_min,
    //     max: debug_max,
    // });
    let painter = ui.painter_at(Rect {
        min: app.min,
        max: app.max,
    });
    painter.rect_filled(
        Rect {
            min: app.min,
            max: app.max,
        },
        Rounding::none(),
        Color32::BLACK,
    );

    match &app.grid_status {
        GridStatus::OnAndVisible => draw_grid(app.min, app.max, app, &painter),
        _ => (),
    }
    let mut i = -1f32;
    while i <= 1. {
        for j in 1..=10 {
            for p in &app.paths {
                if j == p.layer {
                    // if i == 0. {
                        // draw_path(&painter, p, app);
                    // } else {
                        draw_offset_path(app, &painter, p, i * (app.max.x - app.min.x));
                    // }
                }
            }
        }
        i += 1.;
    }
    let mut i = -1f32;
    match app.curr_path {
        None => (),
        Some(curr_path) => {
            while i <= 1. {
                draw_outline(&painter, &app.paths[curr_path], i * (app.max.x - app.min.x));
                draw_offset_path(app, &painter, &app.paths[curr_path], i * (app.max.x - app.min.x));
                // draw move circle
                painter.circle_filled(
                    Pos2 {
                        x: app.paths[curr_path].x + i * (app.max.x - app.min.x),
                        y: app.paths[curr_path].y,
                    },
                    15.,
                    Color32::from_rgb(50, 100, 255),
                );
                let p = &app.paths[curr_path];
                painter.circle_filled(
                    Pos2 {
                        x: p.x
                            + i * (app.max.x - app.min.x)
                            + p.w
                            + p.h / 2. * p.hori as u8 as f32 * p.rnd_end2 as u8 as f32,
                        y: p.y + p.h / 2.,
                    },
                    15.,
                    Color32::GOLD,
                );
                painter.circle_filled(
                    Pos2 {
                        x: p.x + i * (app.max.x - app.min.x) + p.w / 2.,
                        y: p.y + p.h + p.w / 2. * !p.hori as u8 as f32 * p.rnd_end2 as u8 as f32,
                    },
                    15.,
                    Color32::GOLD,
                );
                i += 1.;
            }
        }
    }
}
fn clicked_w_circle(app: &App, pos: Pos2, offset: f32) -> bool {
    match app.curr_path {
        None => false,
        Some(curr_path) => {
            let p: &Path = &app.paths[curr_path];
            let x = p.x - pos.x
                + offset
                + p.w
                + p.h / 2. * p.hori as u8 as f32 * p.rnd_end2 as u8 as f32;
            let y = p.y - pos.y + p.h / 2.;
            if ((x * x + y * y) as f64).sqrt() < 15. {
                return true;
            }
            false
        }
    }
}
fn clicked_h_circle(app: &App, pos: Pos2, offset: f32) -> bool {
    match app.curr_path {
        None => false,
        Some(curr_path) => {
            let p: &Path = &app.paths[curr_path];
            let x = p.x - pos.x + offset + p.w / 2.;
            let y = p.y - pos.y + p.h + p.w / 2. * !p.hori as u8 as f32 * p.rnd_end2 as u8 as f32;
            if ((x * x + y * y) as f64).sqrt() < 15. {
                return true;
            }
            false
        }
    }
}
fn clicked_move_circle(app: &App, pos: Pos2, offset: f32) -> bool {
    match app.curr_path {
        None => false,
        Some(curr_path) => {
            let p: &Path = &app.paths[curr_path];
            let x = p.x - pos.x + offset;
            let y = p.y - pos.y;
            if ((x * x + y * y) as f64).sqrt() < 15. {
                return true;
            }
            false
        }
    }
}
fn clicked_path(app: &App, click_pos: Pos2, offset: f32) -> Option<usize> {
    let mut found = false;
    let index;
    let mut return_index;
    match app.curr_path {
        None => {
            index = 0;
            return_index = None;
        }
        Some(curr_path) => {
            index = curr_path;
            return_index = Some(curr_path);
        }
    }
    for i in index..app.paths.len() {
        let l =
            (app.paths[i].hori as u8 * app.paths[i].rnd_end1 as u8) as f32 * app.paths[i].h * 0.5
                + offset;
        let r =
            (app.paths[i].hori as u8 * app.paths[i].rnd_end2 as u8) as f32 * app.paths[i].h * 0.5
                - offset;
        let t =
            (!app.paths[i].hori as u8 * app.paths[i].rnd_end1 as u8) as f32 * app.paths[i].w * 0.5;
        let b =
            (!app.paths[i].hori as u8 * app.paths[i].rnd_end2 as u8) as f32 * app.paths[i].w * 0.5;
        if click_pos.x >= app.paths[i].x - l
            && click_pos.x <= app.paths[i].x + app.paths[i].w + r
            && click_pos.y >= app.paths[i].y - t
            && click_pos.y <= app.paths[i].y + app.paths[i].h + b
            && Some(i) != app.curr_path
        {
            return_index = Some(i);
            found = true;
            break;
        }
    }

    if !found {
        for i in 0..app.paths.len() {
            let l = (app.paths[i].hori as u8 * app.paths[i].rnd_end1 as u8) as f32
                * app.paths[i].h
                * 0.5
                + offset;
            let r = (app.paths[i].hori as u8 * app.paths[i].rnd_end2 as u8) as f32
                * app.paths[i].h
                * 0.5
                - offset;
            let t = (!app.paths[i].hori as u8 * app.paths[i].rnd_end1 as u8) as f32
                * app.paths[i].w
                * 0.5;
            let b = (!app.paths[i].hori as u8 * app.paths[i].rnd_end2 as u8) as f32
                * app.paths[i].w
                * 0.5;
            match app.curr_path {
                None => {}
                Some(curr_path) => {
                    if click_pos.x >= app.paths[i].x - l
                        && click_pos.x <= app.paths[i].x + app.paths[i].w + r
                        && click_pos.y >= app.paths[i].y - t
                        && click_pos.y <= app.paths[i].y + app.paths[i].h + b
                        && i != curr_path
                    {
                        return_index = Some(i);
                        break;
                    }
                }
            }
        }
    }
    return_index
}
/*fn mutate_clicked_path(app: &mut App, click_pos: Pos2, offset: f32) -> bool {
    let mut found = false;
    for i in app.curr_path..app.paths.len() {
        let l =
            (app.paths[i].hori as u8 * app.paths[i].rnd_end1 as u8) as f32 * app.paths[i].h * 0.5
                + offset;
        let r =
            (app.paths[i].hori as u8 * app.paths[i].rnd_end2 as u8) as f32 * app.paths[i].h * 0.5
                - offset;
        let t =
            (!app.paths[i].hori as u8 * app.paths[i].rnd_end1 as u8) as f32 * app.paths[i].w * 0.5;
        let b =
            (!app.paths[i].hori as u8 * app.paths[i].rnd_end2 as u8) as f32 * app.paths[i].w * 0.5;
        if click_pos.x >= app.paths[i].x - l
            && click_pos.x <= app.paths[i].x + app.paths[i].w + r
            && click_pos.y >= app.paths[i].y - t
            && click_pos.y <= app.paths[i].y + app.paths[i].h + b
            && i != app.curr_path
        {
            app.curr_path = i;
            found = true;
            break;
        }
    }
    if !found {
        for i in 0..app.paths.len() {
            let l = (app.paths[i].hori as u8 * app.paths[i].rnd_end1 as u8) as f32
                * app.paths[i].h
                * 0.5
                + offset;
            let r = (app.paths[i].hori as u8 * app.paths[i].rnd_end2 as u8) as f32
                * app.paths[i].h
                * 0.5
                - offset;
            let t = (!app.paths[i].hori as u8 * app.paths[i].rnd_end1 as u8) as f32
                * app.paths[i].w
                * 0.5;
            let b = (!app.paths[i].hori as u8 * app.paths[i].rnd_end2 as u8) as f32
                * app.paths[i].w
                * 0.5;
            if click_pos.x >= app.paths[i].x - l
                && click_pos.x <= app.paths[i].x + app.paths[i].w + r
                && click_pos.y >= app.paths[i].y - t
                && click_pos.y <= app.paths[i].y + app.paths[i].h + b
                && i != app.curr_path
            {
                app.curr_path = i;
                found = true;
                break;
            }
        }
    }
    found
}*/
fn draw_offset_path(app: &App, painter: &Painter, path: &Path, offset: f32) {
    let r: f32;
    let color1 = (200. * path.d1) as u8;
    let color2 = (200. * path.d2) as u8;
    if path.hori {
        r = path.h / 2. + 0.5;
        if path.w < 2. * r
            && (path.rnd_end1 || path.rnd_end2)
            && path.x + offset > app.min.x - r
            && path.x + offset < app.max.x + r
        {
            draw_left_circle(painter, path, offset, color1, r);
        } else {
            if path.rnd_end1 && path.x + offset > app.min.x - r && path.x + offset < app.max.x + r {
                draw_left_circle(painter, path, offset, color1, r);
            }
            if path.rnd_end2
                && path.x + offset + path.w > app.min.x - r
                && path.x + offset + path.w < app.max.x + r
            {
                draw_right_circle(painter, path, offset, color2, r);
            }
            draw_hori_rect(painter, path, offset, app);
        }
    } else {
        r = path.w / 2. + 0.5;
        if path.h < 2. * r
            && (path.rnd_end1 || path.rnd_end2)
            && path.x + offset > app.min.x - 2. * r
            && path.x + offset < app.max.x
        {
            draw_top_circle(painter, path, offset, color1, r);
        } else {
            if path.rnd_end1 && path.x + offset > app.min.x - 2. * r && path.x + offset < app.max.x
            {
                draw_top_circle(painter, path, offset, color1, r);
            }
            if path.rnd_end2 && path.x + offset > app.min.x - 2. * r && path.x + offset < app.max.x
            {
                draw_bottom_circle(painter, path, offset, color2, r);
            }
            draw_vert_rect(painter, path, offset, app);
        }
    }
}
fn draw_left_circle(painter: &Painter, path: &Path, offset: f32, color: u8, r: f32) {
    painter.circle_filled(
        Pos2 {
            x: path.x + offset,
            y: path.y + path.h / 2.,
        },
        r,
        Color32::from_rgb(255 - color, 255 - color, 255 - color),
    );
}
fn draw_right_circle(painter: &Painter, path: &Path, offset: f32, color: u8, r: f32) {
    painter.circle_filled(
        Pos2 {
            x: path.x + path.w + offset,
            y: path.y + path.h / 2.,
        },
        r,
        Color32::from_rgb(255 - color, 255 - color, 255 - color),
    );
}
fn draw_top_circle(painter: &Painter, path: &Path, offset: f32, color: u8, r: f32) {
    painter.circle_filled(
        Pos2 {
            x: path.x + path.w / 2. + offset,
            y: path.y,
        },
        r,
        Color32::from_rgb(255 - color, 255 - color, 255 - color),
    );
}
fn draw_bottom_circle(painter: &Painter, path: &Path, offset: f32, color: u8, r: f32) {
    painter.circle_filled(
        Pos2 {
            x: path.x + path.w / 2. + offset,
            y: path.y + path.h,
        },
        r,
        Color32::from_rgb(255 - color, 255 - color, 255 - color),
    );
}
fn draw_hori_rect(painter: &Painter, path: &Path, offset: f32, app: &App) {
    // if path.d1 == path.d2 {
    //     let color = (200. * path.d1) as u8;
    //      painter.rect_filled(
    //         Rect {
    //             min: Pos2 {
    //                 x: path.x,
    //                 y: path.y,
    //             },
    //             max: {
    //                 Pos2 {
    //                     x: path.x + path.w,
    //                     y: path.y + path.h,
    //                 }
    //             },
    //         },
    //         Rounding::none(),
    //         Color32::from_rgb(255 - color, 255 - color, 255 - color),
    //     );
    //      return;
    // }
    for i in (path.x as i32)..((path.x + path.w) as i32) {
        if i as f32 + offset >= app.min.x && i as f32 + offset <= app.max.x {
            let color = ((path.d1 * (-i as f32 + path.x + path.w)
                + (path.d2 * (i as f32 - path.x)))
                / (path.w)
                * 200.) as u8;
            painter.line_segment(
                [
                    Pos2 {
                        x: i as f32 + offset,
                        y: path.y,
                    },
                    Pos2 {
                        x: i as f32 + offset,
                        y: path.y + path.h,
                    },
                ],
                Stroke {
                    width: 2.,
                    color: Color32::from_rgb(255 - color, 255 - color, 255 - color),
                },
            )
        }
    }
}
fn draw_vert_rect(painter: &Painter, path: &Path, offset: f32, app: &App) {
    if path.x + offset + path.w > app.min.x && path.x + offset < app.max.x {
        // if path.d1 == path.d2 {
        //     let color = (200. * path.d1) as u8;
        //      painter.rect_filled(
        //         Rect {
        //             min: Pos2 {
        //                 x: path.x,
        //                 y: path.y,
        //             },
        //             max: {
        //                 Pos2 {
        //                     x: path.x + path.w,
        //                     y: path.y + path.h,
        //                 }
        //             },
        //         },
        //         Rounding::none(),
        //         Color32::from_rgb(255 - color, 255 - color, 255 - color),
        //     );
        //      return;
        // }
        for i in (path.y as i32)..((path.y + path.h) as i32) {
            let color = ((path.d1 * (-i as f32 + path.y + path.h)
                + (path.d2 * (i as f32 - path.y)))
                / (path.h)
                * 200.) as u8;
            painter.line_segment(
                [
                    Pos2 {
                        x: path.x + offset,
                        y: i as f32,
                    },
                    Pos2 {
                        x: path.x + path.w + offset,
                        y: i as f32,
                    },
                ],
                Stroke {
                    width: 2.,
                    color: Color32::from_rgb(255 - color, 255 - color, 255 - color),
                },
            )
        }
    }
}
fn draw_outline(painter: &Painter, path: &Path, offset: f32) {
    let r: f32;
    if path.hori {
        r = path.h / 2. + 2.0;
        if path.w < 2. * r && (path.rnd_end1 || path.rnd_end2) {
            painter.circle_filled(
                Pos2 {
                    x: path.x + offset,
                    y: path.y + path.h / 2.,
                },
                r,
                Color32::YELLOW,
            );
        } else {
            painter.rect_filled(
                Rect {
                    min: Pos2 {
                        x: path.x - 3. + offset,
                        y: path.y - 2.,
                    },
                    max: {
                        Pos2 {
                            x: path.x + path.w + 2. + offset,
                            y: path.y + path.h + 2.,
                        }
                    },
                },
                Rounding::none(),
                Color32::YELLOW,
            );
            if path.rnd_end1 {
                painter.circle_filled(
                    Pos2 {
                        x: path.x + offset,
                        y: path.y + path.h / 2.,
                    },
                    r,
                    Color32::YELLOW,
                );
            }
            if path.rnd_end2 {
                painter.circle_filled(
                    Pos2 {
                        x: path.x + path.w + offset,
                        y: path.y + path.h / 2.,
                    },
                    r,
                    Color32::YELLOW,
                );
            }
        }
    } else {
        r = path.w / 2. + 2.0;
        if path.h < 2. * r && (path.rnd_end1 || path.rnd_end2) {
            painter.circle_filled(
                Pos2 {
                    x: path.x + path.w / 2. + offset,
                    y: path.y,
                },
                r,
                Color32::YELLOW,
            );
        } else {
            painter.rect_filled(
                Rect {
                    min: Pos2 {
                        x: path.x - 2. + offset,
                        y: path.y - 2.,
                    },
                    max: {
                        Pos2 {
                            x: path.x + path.w + 2. + offset,
                            y: path.y + path.h + 2.,
                        }
                    },
                },
                Rounding::none(),
                Color32::YELLOW,
            );
            if path.rnd_end1 {
                painter.circle_filled(
                    Pos2 {
                        x: path.x + path.w / 2. + offset,
                        y: path.y,
                    },
                    r,
                    Color32::YELLOW,
                );
            }
            if path.rnd_end2 {
                painter.circle_filled(
                    Pos2 {
                        x: path.x + path.w / 2. + offset,
                        y: path.y + path.h,
                    },
                    r,
                    Color32::YELLOW,
                );
            }
        }
    }
}
fn draw_grid(min: Pos2, max: Pos2, app: &App, painter: &Painter) {
    let mut i = min.x + app.grid_size;
    while i < max.x {
        painter.line_segment(
            [Pos2 { x: i, y: min.y }, Pos2 { x: i, y: max.y }],
            Stroke {
                width: 0.25,
                color: Color32::from_rgb(255, 255, 255),
            },
        );
        i += app.grid_size;
    }
    i = min.y + app.grid_size;
    while i < max.y {
        painter.line_segment(
            [Pos2 { x: min.x, y: i }, Pos2 { x: max.x, y: i }],
            Stroke {
                width: 0.25,
                color: Color32::from_rgb(255, 255, 255),
            },
        );
        i += app.grid_size;
    }
}
/*// painter.rect_filled(
//     Rect {
//         min: Pos2 {
//             x: path1.x1,
//             y: path1.y1,
//         },
//         max: {
//             Pos2 {
//                 x: path1.x2,
//                 y: path1.y2,
//             }
//         },
//     },
//     Rounding::none(),
//     Color32::BLUE,
// );
// ui.separator();
// ui.collapsing("Click to see what is hidden!", |ui| {
//     ui.label("Not much, as it turns out");
// });*/
