use raylib::color::Color; //use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDraw;
use std::sync::Arc; //, RaylibDrawHandle};
use std::sync::RwLock;
use std::thread;
//use raylib::{RaylibHandle};

pub mod sort;
use crate::sort::sorting::{SortingAlgo, TableSort};

pub const SIZE: usize = 100;
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 800;
const FPS: u32 = 100;
const SEPARATOR: i32 = 2;
const TABLE_WIDTH: i32 = (WIDTH - 2 * SEPARATOR * SIZE as i32) / SIZE as i32;
const TABLE_MAX_HEIGHT: i32 = (0.75 * HEIGHT as f32).round() as i32;
const MARGING_LEFT: i32 = (WIDTH - (TABLE_WIDTH * SIZE as i32 + 2 * SEPARATOR * SIZE as i32)) / 2;

fn main() {
    let table_sort = Arc::new(RwLock::new(TableSort::default()));

    let tri_clone = Arc::clone(&table_sort);
    thread::spawn(move || {
        let mut data = tri_clone.write().unwrap();
        data.init(SortingAlgo::QuickSort);
        data.quicksort(0, SIZE - 1);
    });

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Sort Visualizer")
        .build();

    rl.set_target_fps(FPS);

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::BLACK);
        draw.draw_text("Sort Visualizer", 12, 12, 20, Color::WHITE);

        // On avance d'une étape
        //if !table_sort.sorted {
        //    match table_sort.sort_type {
        //        SortingAlgo::SimpleSort => table_sort.sort_step(),
        //        SortingAlgo::SelectionSort => table_sort.selection_sort(),
        //        SortingAlgo::QuickSort => table_sort.quicksort(0, SIZE - 1),
        //    }
        //}

        let data = table_sort.read().unwrap();
        for i in 0..SIZE {
            let mut col = Color::WHITE;
            if data.sorted {
                col = Color::LIGHTGREEN;
            } else if i == data.current_index {
                col = Color::YELLOW;
            }

            let table_height: i32 = TABLE_MAX_HEIGHT * data.table[i] / SIZE as i32;
            draw.draw_rectangle(
                i as i32 * TABLE_WIDTH + 2 * SEPARATOR * i as i32 + MARGING_LEFT,
                HEIGHT - 10 - table_height,
                TABLE_WIDTH,
                table_height,
                col,
            );
        }
    }
}
