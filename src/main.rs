#![feature(int_roundings)]

mod model;
mod view;

mod upstream;

fn main() -> eframe::Result {
    view::main()
}
