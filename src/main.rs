use std::{thread::{self, Thread}, time::Duration, f32::consts::E, sync::{Mutex, Arc}};

use macroquad::{window::{next_frame, screen_width, screen_height}, shapes::draw_rectangle, color::Color};
use rand::Rng;

fn check(elements: &Vec<i32>) -> bool {
    for i in 0..elements.len() - 1 {
        if elements[i] < elements[i + 1] {
            continue;
        }
        else {
            return false
        }
    }
    true
}

async fn uptade(elements: &mut [i32], done: bool) {
    let element_count = elements.len();
    let element_max_value = 100000;
    let element_width = screen_width() / element_count as f32;

    for x in 0..element_count {
        let element = elements[x as usize];
        let element_height = element as f32 / element_max_value as f32 * screen_height();
        let color = if done {Color::from_hex(0x00ff00)} else {Color::from_hex(0xffffff)};
        draw_rectangle(
            x as f32 * element_width,
            screen_height() - element_height,
            element_width,
            element_height,
            color
        )
    }
    next_frame().await;
}

pub fn order(elements: &mut [i32]) {

    let (a,b) = elements.split_at_mut(elements.len() / 2);
    let ra = Mutex::new(a);
    let rb = Mutex::new(b);
    let ca = Arc::new(ra);
    let cb: Arc<Mutex<&mut [i32]>> = Arc::new(rb);
    
    thread::scope(move |scope| {
        let at = scope.spawn(move || {
            let mut un = ca.lock().unwrap();
            for i in 0..un.len() - 1 {
                if un[i] > un[i + 1] {
                    un.swap(i, i + 1);
                }
            }
        });

        let bt = scope.spawn(move || {
            let mut un = cb.lock().unwrap();
            for i in 0..un.len() - 1 {
                if un[i] > un[i + 1] {
                    un.swap(i, i + 1);
                }
            }
        });

        at.join().unwrap();
        bt.join().unwrap();
    });

    for i in (0..elements.len() - 1).rev() {
        if elements[i] > elements[i + 1] {
            elements.swap(i, i + 1);
        }
    }

}

#[macroquad::main("sorter")]
async fn main() {
    let element_count = 1000;
    let element_max_value = 100000;
    let mut elements = vec![0i32;element_count];
    for element in elements.iter_mut() {
        *element = rand::prelude::thread_rng().gen_range(0..element_max_value);
    }
    let mut done = false;
    loop {
        if done {
            uptade(elements.as_mut_slice(), done).await;
            continue;
        }
        if check(&elements) {
            println!("done!");
            done = true;
        }
        order(elements.as_mut_slice());
        uptade(elements.as_mut_slice(), done).await;
    }
}
