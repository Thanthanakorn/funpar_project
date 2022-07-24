use std::cmp::Ordering;
use std::iter::Scan;
use std::time::{Duration, Instant};
use std::vec;
use std::collections::LinkedList;
use rayon::iter::*;
pub fn q_sort<T: Copy + Ord>(mut xs: LinkedList<T>) -> LinkedList<T> {
    if xs.len() <= 1 {
        return xs;
    }
    let mut pivot = xs.pop_back().unwrap();
    let mut left: LinkedList<T> = LinkedList::new();
    let mut right: LinkedList<T> = LinkedList::new();
    use std::cmp::Ordering;
    for i in xs.into_iter(){
        if i.cmp(&pivot) == Ordering::Greater{
            right.push_back(i);
        } else {
            left.push_back(i);
        }
    }
    let mut ans_left = q_sort(left);
    let mut ans_right = q_sort(right);
    ans_left.append(&mut vec![pivot].into_iter().collect::<LinkedList<T>>());
    ans_left.append(&mut ans_right.into_iter().collect::<LinkedList<T>>());

    return ans_left;
}

pub fn par_q_sort<T: Copy + Ord + Send>(mut xs: LinkedList<T>) -> LinkedList<T> {
    if xs.len() <= 1 {
        return xs;
    }
    let mut pivot = xs.pop_back().unwrap();
    let mut left: LinkedList<T> = LinkedList::new();
    let mut right: LinkedList<T> = LinkedList::new();
    use std::cmp::Ordering;
    for i in xs.into_iter(){
        if i.cmp(&pivot) == Ordering::Greater{
            right.push_back(i);
        } else {
            left.push_back(i);
        }
    }
    let (mut ans_left,mut ans_right) = rayon::join(||par_q_sort(left), ||par_q_sort(right));
    ans_left.append(&mut vec![pivot].into_iter().collect::<LinkedList<T>>());
    ans_left.append(&mut ans_right.into_iter().collect::<LinkedList<T>>());

    return ans_left;
}

pub fn merge<T: Copy + Ord>(mut xs: LinkedList<T>, mut ys: LinkedList<T>) -> LinkedList<T>{
    let mut combined:LinkedList<T> = LinkedList::new();
    use std::cmp::Ordering;
    while !xs.is_empty() && !ys.is_empty() {
        if xs.front().unwrap().cmp(&ys.front().unwrap()) == Ordering::Less {
            combined.push_back(*xs.front().unwrap());
            xs.pop_front();
        } else {
            combined.push_back(*ys.front().unwrap());
            ys.pop_front();
        }
    }
    while !xs.is_empty() {
        combined.push_back(*xs.front().unwrap());
        xs.pop_front();
    }
    while !ys.is_empty() {
        combined.push_back(*ys.front().unwrap());
        ys.pop_front();
    }
    combined
}


pub fn merge_sort<T: Clone + Copy + Ord>(mut xs: LinkedList<T>) -> LinkedList<T> {
    if xs.len() <= 1 {
        return xs;
    }
    let mut right = xs.split_off(xs.len()/2);
    let mut left = xs;
    let left = merge_sort(left);
    let right = merge_sort(right);
    merge(left, right)
}

pub fn par_merge_sort<T: Copy + Ord + Send>(mut xs: LinkedList<T>) -> LinkedList<T> {
    if xs.len() <= 1 {
        return xs;
    }
    let mut right = xs.split_off(xs.len()/2);
    let mut left = xs;
    let (left, right) = rayon::join(|| par_merge_sort(left), || par_merge_sort(right));
    merge(left, right)
}

pub fn evenoddsort(mut xs: LinkedList<usize>) -> LinkedList<usize> {
    let ys = xs.clone();
    let odd: LinkedList<usize> = xs.into_iter().filter(|i| i % 2 != 0).collect();
    let even: LinkedList<usize> = ys.into_iter().filter(|i| i % 2 == 0).collect();
    let mut left = q_sort(odd);
    let mut right = q_sort(even);
    left.append(&mut right);
    left
}

pub fn par_evenoddsort(mut xs: LinkedList<usize>) -> LinkedList<usize> {
    let ys = xs.clone();
    let odd: LinkedList<usize> = xs.into_par_iter().filter(|i| i % 2 != 0).collect();
    let even: LinkedList<usize> = ys.into_par_iter().filter(|i| i % 2 == 0).collect();
    let (mut left, mut right) = rayon::join(|| q_sort(odd), || q_sort(even));
    left.append(&mut right);
    left
}

pub fn oddevensort(mut xs: LinkedList<usize>) -> LinkedList<usize> {
    let ys = xs.clone();
    let odd: LinkedList<usize> = xs.into_iter().filter(|i| i % 2 != 0).collect();
    let even: LinkedList<usize> = ys.into_iter().filter(|i| i % 2 == 0).collect();
    let mut left = q_sort(odd);
    let mut right = q_sort(even);
    right.append(&mut left);
    right
}

pub fn par_oddevensort(mut xs: LinkedList<usize>) -> LinkedList<usize> {
    let ys = xs.clone();
    let odd: LinkedList<usize> = xs.into_par_iter().filter(|i| i % 2 != 0).collect();
    let even: LinkedList<usize> = ys.into_par_iter().filter(|i| i % 2 == 0).collect();
    let (mut left, mut right) = rayon::join(|| q_sort(odd), || q_sort(even));
    right.append(&mut left);
    right
}