use raylib::color::Color;
//use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDraw; //, RaylibDrawHandle};
//use raylib::{RaylibHandle};

pub mod sort;
use crate::sort::sorting::{SortingAlgo, TableSort};

pub const SIZE: usize = 100;
const WIDTH: i32 = 1900;
const HEIGHT: i32 = 1000;
const FPS: u32 = 100;
const SEPARATOR: i32 = 2;
const TABLE_WIDTH: i32 = (WIDTH - 2 * SEPARATOR * SIZE as i32) / SIZE as i32;
const TABLE_MAX_HEIGHT: i32 = (0.75 * HEIGHT as f32).round() as i32;
const MARGING_LEFT: i32 = (WIDTH - (TABLE_WIDTH * SIZE as i32 + 2 * SEPARATOR * SIZE as i32)) / 2;

fn main() {
    println!("TABLE_WIDTH: {}", TABLE_WIDTH);
    let mut table_sort = TableSort {
        table: [0; SIZE],
        current_index: 0,
        no_swap: true,
        sorted: false,
        pivot: SIZE - 1,
        min_index: 0,
        sort_type: SortingAlgo::SimpleSort,
    };
    table_sort.init(SortingAlgo::QuickSort);

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
        if !table_sort.sorted {
            match table_sort.sort_type {
                SortingAlgo::SimpleSort => table_sort.sort_step(),
                SortingAlgo::SelectionSort => table_sort.selection_sort(),
                SortingAlgo::QuickSort => table_sort.quicksort(0, SIZE - 1),
            }
        }

        for i in 0..SIZE {
            let mut col = Color::WHITE;
            if table_sort.sorted {
                col = Color::LIGHTGREEN;
            } else if i == table_sort.current_index {
                col = Color::YELLOW;
            }

            let table_height: i32 = TABLE_MAX_HEIGHT * table_sort.table[i] / SIZE as i32;
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
