#![allow(unused_imports)]
use std::{cmp::*, collections::*, error};
use svg::node::{
    element::{Circle, Group, Line, Rectangle, Title},
    Text,
};
use wasm_bindgen::prelude::*;

type Color = String;

#[derive(Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Datum {
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    #[wasm_bindgen(getter_with_clone)]
    pub value: String,
}

pub struct Visualizer {
    pub h: usize,
    pub w: usize,
    common: Vec<Command>,
    pub commands: Vec<Vec<Command>>,
    pub data: Vec<Datum>,
    pub scores: Vec<i32>,
}

#[derive(Clone)]
pub enum Command {
    Circle(Coord, usize, Color, String, String),
    Rect(Coord, Coord, Color, String, String),
    Line(Coord, Coord, Color, String, String),
}

pub fn gen_visualizer(f: &str) -> Visualizer {
    parse_command(f)
}

fn parse_command(f: &str) -> Visualizer {
    let mut h = !0;
    let mut w = !0;
    let mut common = vec![];
    let mut commands = vec![];
    let mut command = vec![];
    let mut data = vec![];
    let mut scores = vec![];
    for line in f.lines() {
        if line.starts_with('@') {
            let parts = line[1..].split_whitespace().collect::<Vec<_>>();
            if parts.is_empty() {
                continue;
            }
            match parts[0] {
                "setting" => {
                    h = parts[1].parse().unwrap();
                    w = parts[2].parse().unwrap();
                }
                "init" => {
                    common = command.clone();
                    command = vec![];
                }
                "circle" => {
                    let x = parts[1].parse().unwrap();
                    let y = parts[2].parse().unwrap();
                    let r = parts[3].parse().unwrap();
                    let color = parts[4].to_string();
                    let text = parts[5].trim_matches('\'').to_string();
                    let title = parts[6].trim_matches('\'').to_string();
                    command.push(Command::Circle(Coord::new(x, y), r, color, text, title));
                }
                "rect" => {
                    let x1 = parts[1].parse().unwrap();
                    let y1 = parts[2].parse().unwrap();
                    let x2 = parts[3].parse().unwrap();
                    let y2 = parts[4].parse().unwrap();
                    let color = parts[5].to_string();
                    let text = parts[6].trim_matches('\'').to_string();
                    let title = parts[7].trim_matches('\'').to_string();
                    command.push(Command::Rect(
                        Coord::new(x1, y1),
                        Coord::new(x2, y2),
                        color,
                        text,
                        title,
                    ));
                }
                "line" => {
                    let x1 = parts[1].parse().unwrap();
                    let y1 = parts[2].parse().unwrap();
                    let x2 = parts[3].parse().unwrap();
                    let y2 = parts[4].parse().unwrap();
                    let color = parts[5].to_string();
                    let text = parts[6].trim_matches('\'').to_string();
                    let title = parts[7].trim_matches('\'').to_string();
                    command.push(Command::Line(
                        Coord::new(x1, y1),
                        Coord::new(x2, y2),
                        color,
                        text,
                        title,
                    ));
                }
                "next" => {
                    commands.push(command);
                    command = vec![];
                }
                _ => panic!(),
            }
        } else if line.starts_with("[DATA]") {
            let parts = line[7..].split_whitespace().collect::<Vec<_>>();
            let name = parts[0].to_string();
            let value = parts[2].to_string();
            if name == "tscore" {
                scores.push(value.parse().unwrap());
            }
            if let Some(i) = data.iter().position(|d: &Datum| d.name == name) {
                data[i].value = value;
            } else {
                data.push(Datum { name, value });
            }
        }
    }
    Visualizer {
        h,
        w,
        common,
        commands,
        data,
        scores,
    }
}

fn command_to_svg(command: &Command, scale: f64) -> Group {
    match command {
        Command::Circle(p, r, color, text, title) => {
            let r = *r as f64 * scale;
            let color = color.clone();
            Group::new()
                .add(
                    Circle::new()
                        .set("cy", p.x as f64 * scale)
                        .set("cx", p.y as f64 * scale)
                        .set("r", r as f64 * scale)
                        .set("fill", color)
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .add(Title::new(title)),
                )
                .add(
                    svg::node::element::Text::new(text)
                        .set("y", p.x as f64 * scale)
                        .set("x", p.y as f64 * scale)
                        .set("font-size", r as f64 * scale / 2.0)
                        .set("dominant-baseline", "central")
                        .set("text-anchor", "middle"),
                )
        }
        Command::Rect(p1, p2, color, text, title) => {
            let color = color.clone();
            Group::new()
                .add(
                    Rectangle::new()
                        .set("y", p1.x as f64 * scale)
                        .set("x", p1.y as f64 * scale)
                        .set("height", p2.x as f64 * scale - p1.x as f64 * scale)
                        .set("width", p2.y as f64 * scale - p1.y as f64 * scale)
                        .set("fill", color)
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .add(Title::new(title)),
                )
                .add(
                    svg::node::element::Text::new(text)
                        .set("y", (p1.x + p2.x) as f64 * scale / 2.0)
                        .set("x", (p1.y + p2.y) as f64 * scale / 2.0)
                        .set("font-size", 20.0)
                        .set("dominant-baseline", "central")
                        .set("text-anchor", "middle"),
                )
        }
        Command::Line(p1, p2, color, text, title) => {
            let color = color.clone();
            Group::new()
                .add(
                    Line::new()
                        .set("y1", p1.x as f64 * scale)
                        .set("x1", p1.y as f64 * scale)
                        .set("y2", p2.x as f64 * scale)
                        .set("x2", p2.y as f64 * scale)
                        .set("stroke", color)
                        .set("stroke-width", 1)
                        .add(Title::new(title)),
                )
                .add(
                    svg::node::element::Text::new(text)
                        .set("y", (p1.x + p2.x) as f64 * scale / 2.0)
                        .set("x", (p1.y + p2.y) as f64 * scale / 2.0)
                        .set("font-size", 20.0)
                        .set("dominant-baseline", "central")
                        .set("text-anchor", "middle"),
                )
        }
    }
}

#[wasm_bindgen]
pub struct Res {
    pub score: i32,
    #[wasm_bindgen(getter_with_clone)]
    pub error: String,
    #[wasm_bindgen(getter_with_clone)]
    pub svg: String,
    #[wasm_bindgen(getter_with_clone)]
    pub data: Vec<Datum>,
}

#[allow(non_upper_case_globals)]
static mut visualizer: Visualizer = Visualizer {
    h: 0,
    w: 0,
    common: vec![],
    commands: vec![],
    data: vec![],
    scores: vec![],
};
static mut PREV_SEED: usize = !0;

#[wasm_bindgen]
pub fn vis(f: &str, t: usize, seed: usize) -> Res {
    #[allow(non_upper_case_globals)]
    unsafe {
        if seed != PREV_SEED {
            PREV_SEED = seed;
            visualizer = parse_command(f);
        }
        let svg_size = 650;
        let scale = svg_size as f64 / max(visualizer.h, visualizer.w) as f64;

        let score = if visualizer.scores.len() >= t && t > 0 {
            visualizer.scores[t - 1]
        } else {
            -1
        };
        let error = "".to_string();
        let mut doc = svg::Document::new()
            .set("viewBox", (-5, -5, svg_size + 10, svg_size + 10))
            .set("width", svg_size + 10)
            .set("height", svg_size + 10);

        let canvas_command = Command::Rect(
            Coord::new(0, 0),
            Coord::new(visualizer.h, visualizer.w),
            "lightgray".to_string(),
            "".to_string(),
            "".to_string(),
        );
        doc = doc.add(command_to_svg(&canvas_command, scale));
        for command in visualizer.common.iter() {
            doc = doc.add(command_to_svg(command, scale));
        }
        if t > 0 {
            for command in visualizer.commands[t - 1].iter() {
                doc = doc.add(command_to_svg(command, scale));
            }
        }

        Res {
            score,
            error,
            svg: doc.to_string(),
            data: visualizer.data.clone(),
        }
    }
}

#[wasm_bindgen]
pub fn get_max_turn(f: &str, seed: usize) -> i32 {
    unsafe {
        if seed != PREV_SEED {
            PREV_SEED = seed;
            visualizer = parse_command(f);
        }
        visualizer.commands.len() as i32
    }
}
