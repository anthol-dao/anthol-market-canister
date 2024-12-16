use super::*;
use canbench_rs::bench;

const VEC: [usize; 24] = [0; 24];

#[bench]
fn iter_with_vec_deque() {
    let vec: VecDeque<usize> = VEC.into();
    let slices = vec.as_slices();
    let _vec_copy = [slices.0, slices.1].concat();
}

#[bench]
fn iter_with_vec() {
    let vec: Vec<usize> = VEC.into();

    let _vec_copy = vec.clone();
}

#[bench]
fn push_front_pop_back_with_vec_deque() {
    let mut vec: VecDeque<usize> = VEC.into();

    for i in 0..24 {
        vec.push_front(i);
        vec.pop_back();
    }
}

#[bench]
fn push_front_pop_back_with_vec() {
    let mut vec: Vec<usize> = VEC.into();

    for i in 0..24 {
        vec.insert(0, i);
        vec.pop();
    }
}
