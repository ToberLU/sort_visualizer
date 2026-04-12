use raylib::color::Color;
//use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::{RaylibHandle};
//use std::array;
use rand::prelude::*;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;
const SIZE: usize = 50;
const FPS: u32 = 100;
const SEPARATOR: i32 = 2;
const TABLE_WIDTH: i32 = (WIDTH - 2*SEPARATOR*SIZE as i32) / SIZE as i32;
const TABLE_MAX_HEIGHT: i32 = (0.75 * HEIGHT as f32).round() as i32;
const MARGING_LEFT: i32 = (WIDTH - (TABLE_WIDTH * SIZE as i32 + 2*SEPARATOR*SIZE as i32)) / 2;

pub enum SortingAlgo {
    SelectionSort,
    SimpleSort,
}

pub struct TableSort {
    table: [i32; SIZE],
    current_index: usize,
    no_swap: bool,
    sorted: bool,
    pivot: usize,
    min_index: usize,
    sort_type: SortingAlgo,
}

impl TableSort{

    pub fn init(&mut self, sort: SortingAlgo) {
        for i in 0..SIZE {
            self.table[i] = 1 + i as i32;
        }

        self.table.shuffle(&mut rand::rng());
        println!("Table : {0:?}", self.table);

        self.sort_type = sort;
        match self.sort_type {
            SortingAlgo::SelectionSort => {
                self.pivot = 0;
            }
            SortingAlgo::SimpleSort => {
                self.pivot = SIZE - 1;
            }
        }
    }

    pub fn sort_step(&mut self) {
        println!("pivot:{}",self.pivot);
        let current_value = self.table[self.current_index];
        if self.current_index >= self.pivot {
            if self.no_swap {
                self.sorted = true;
            }
            self.no_swap = true;
            self.current_index = 0;
            self.pivot -= 1;
            return;
        }
        let next_value = self.table[self.current_index + 1];

        if current_value > next_value {
            let tmp = self.table[self.current_index];
            self.table[self.current_index] = self.table[self.current_index + 1];
            self.table[self.current_index + 1] = tmp;
            self.no_swap = false;
        }

        if self.current_index < self.pivot {
            self.current_index += 1;
        }
        else {
            self.current_index = 0;
        }
    }

    pub fn selection_sort(&mut self) {
        let current_value = self.table[self.current_index];
        println!("pivot2:{}",self.pivot);
        println!("current_value:{}",current_value);
        println!("table[min_index]:{}",self.table[self.min_index]);
        if self.current_index > self.pivot && current_value < self.table[self.min_index] {
            self.min_index = self.current_index;
            println!("min_index:{}", self.min_index);
        }
        if self.current_index < SIZE - 1 {
            self.current_index += 1;
            println!("current_index:{}", self.current_index);
        }
        else {
            let tmp = self.table[self.pivot];
            self.table[self.pivot] = self.table[self.min_index];
            self.table[self.min_index] = tmp;

            self.pivot += 1;
            self.current_index = self.pivot;
            self.min_index = self.current_index;
        }
        if self.pivot == SIZE {
            self.sorted = true;
        }
     
    }

}

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
    table_sort.init(SortingAlgo::SelectionSort);

    let mut table: [i32; SIZE] = [0; SIZE];

    for i in 0..SIZE {
        table[i] = 1 + i as i32;
    }
    table.shuffle(&mut rand::rng());
    println!("Table : {table:?}");

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Sort Visualizer")
        .build();
    
    rl.set_target_fps(FPS);

//    let table_width: i32 = WIDTH / SIZE as i32;
//    let table_max_height: i32 = (0.75 * HEIGHT as f32).round() as i32;

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);
         
        draw.clear_background(Color::BLACK);
        draw.draw_text("Sort Visualizer", 12, 12, 20, Color::WHITE);
        
        // On avance d'une étape
        if !table_sort.sorted {
            match table_sort.sort_type {
                SortingAlgo::SimpleSort => table_sort.sort_step(),
                SortingAlgo::SelectionSort => table_sort.selection_sort(),
            }
        }

        for i in 0..SIZE {
            let mut col = Color::WHITE;
            if table_sort.sorted {
                col = Color::LIGHTGREEN;
            }
            else if i == table_sort.current_index {
                col = Color::YELLOW;
            }

            let table_height: i32 = TABLE_MAX_HEIGHT * table_sort.table[i] / SIZE as i32;
            draw.draw_rectangle(i as i32 * TABLE_WIDTH + 2*SEPARATOR*i as i32 + MARGING_LEFT, HEIGHT - 10 - table_height, TABLE_WIDTH, table_height, col);
        }
    }
}
