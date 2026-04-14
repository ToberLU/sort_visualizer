use crate::SIZE;
use rand::prelude::*;

pub enum SortingAlgo {
    SelectionSort,
    SimpleSort,
    QuickSort,
}

pub struct TableSort {
    pub table: [i32; SIZE],
    pub current_index: usize,
    pub no_swap: bool,
    pub sorted: bool,
    pub pivot: usize,
    pub min_index: usize,
    pub sort_type: SortingAlgo,
}

impl TableSort {
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
            SortingAlgo::QuickSort => {
                self.pivot = 0;
            }
        }
    }

    pub fn sort_step(&mut self) {
        println!("pivot:{}", self.pivot);
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
            self.table.swap(self.current_index, self.current_index + 1);
            self.no_swap = false;
        }

        if self.current_index < self.pivot {
            self.current_index += 1;
        } else {
            self.current_index = 0;
        }
    }

    pub fn selection_sort(&mut self) {
        let current_value = self.table[self.current_index];
        println!("pivot2:{}", self.pivot);
        println!("current_value:{}", current_value);
        println!("table[min_index]:{}", self.table[self.min_index]);
        if self.current_index > self.pivot && current_value < self.table[self.min_index] {
            self.min_index = self.current_index;
            println!("min_index:{}", self.min_index);
        }
        if self.current_index < SIZE - 1 {
            self.current_index += 1;
            println!("current_index:{}", self.current_index);
        } else {
            self.table.swap(self.pivot, self.min_index);

            self.pivot += 1;
            self.current_index = self.pivot;
            self.min_index = self.current_index;
        }
        if self.pivot == SIZE {
            self.sorted = true;
        }
    }

    pub fn heapsort(&mut self) {
        // https://fr.wikipedia.org/wiki/Tri_par_tas
        // Complexe à afficher étape par étape à cause des boucles multiples.
    }

    pub fn quicksort(&mut self, lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }
        // Partition array and get the pivot index
        let p: usize = self.partition(lo, hi);

        // Sort the two partitions
        self.quicksort(lo, p - 1); // Left side of pivot
        self.quicksort(p + 1, hi); // Right side of pivot
    }

    pub fn partition(&mut self, lo: usize, hi: usize) -> usize {
        let pivot = self.table[hi]; // Choose the last element as the pivot

        // Temporary pivot index
        let mut i = lo;

        for j in lo..hi {
            // If the current element is less than or equal to the pivot
            if self.table[j] <= pivot {
                // Swap the current element with the element at the temporary pivot index
                self.table.swap(i, j);
                // Move the temporary pivot index forward
                i += 1;
            }
        }
        // Swap the pivot with the last element
        self.table.swap(i, hi);

        i // the pivot index
    }
}
