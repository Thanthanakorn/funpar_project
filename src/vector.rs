use std::cmp::Ordering;
use std::iter::Scan;
use std::time::{Duration, Instant};
use std::vec;
use rayon::iter::*;

pub fn q_sort<T: Copy + Ord>(mut xs: Vec<T>) -> Vec<T> {
    if xs.len() <= 1 {
        return xs.to_vec();
    }
    let mut pivot = xs.remove(xs.len() - 1);
    let mut left: Vec<T> = vec![];
    let mut right: Vec<T> = vec![];
    use std::cmp::Ordering;
    for i in 0..xs.len() {
        if xs[i].cmp(&pivot) == Ordering::Greater {
            right.push(xs[i]);
        } else {
            left.push(xs[i]);
        }
    }

    let mut ans_left = q_sort(left);
    let mut ans_right = q_sort(right);
    ans_left.append(&mut vec![pivot]);
    ans_left.append(&mut ans_right);

    return ans_left;
}

pub fn par_q_sort<T: Copy + Ord + Send>(mut xs: Vec<T>) -> Vec<T> {
    if xs.len() <= 1 {
        return xs.to_vec();
    }
    let mut pivot = xs.remove(xs.len() - 1);
    let mut left: Vec<T> = vec![];
    let mut right: Vec<T> = vec![];
    use std::cmp::Ordering;
    for i in 0..xs.len() {
        if xs[i].cmp(&pivot) == Ordering::Greater {
            right.push(xs[i]);
        } else {
            left.push(xs[i]);
        }
    }

    let (mut ans_left, mut ans_right) = rayon::join(|| par_q_sort(left), || par_q_sort(right));
    ans_left.append(&mut vec![pivot]);
    ans_left.append(&mut ans_right);

    return ans_left;
}


pub fn merge<T: Copy + Ord>(mut xs: Vec<T>, mut ys: Vec<T>) -> Vec<T> {
    let mut combined = vec![];
    use std::cmp::Ordering;
    while !xs.is_empty() && !ys.is_empty() {
        if xs[0].cmp(&ys[0]) == Ordering::Less {
            combined.push(xs[0]);
            xs.remove(0);
        } else {
            combined.push(ys[0]);
            ys.remove(0);
        }
    }
    while !xs.is_empty() {
        combined.push(xs[0]);
        xs.remove(0);
    }
    while !ys.is_empty() {
        combined.push(ys[0]);
        ys.remove(0);
    }
    combined
}

pub fn merge_sort<T: Clone + Copy + Ord>(mut xs: Vec<T>) -> Vec<T> {
    if xs.len() <= 1 {
        return xs.to_vec();
    }
    let mut left = xs[0..xs.len() / 2].to_vec();
    let mut right = xs[xs.len() / 2..xs.len()].to_vec();
    let left = merge_sort(left);
    let right = merge_sort(right);
    merge(left, right)
}

pub fn par_merge_sort<T: Copy + Ord + Send>(mut xs: Vec<T>) -> Vec<T> {
    if xs.len() <= 1 {
        return xs.to_vec();
    }
    let mut left = xs[0..xs.len() / 2].to_vec();
    let mut right = xs[xs.len() / 2..xs.len()].to_vec();
    let (left, right) = rayon::join(|| par_merge_sort(left), || par_merge_sort(right));
    merge(left, right)
}

pub fn evenoddsort(mut xs: Vec<i64>) -> Vec<i64>{
    let ys = xs.clone();
    let odd: Vec<i64> = xs.into_iter().filter(|i| i%2!=0).collect();
    let even: Vec<i64> = ys.into_iter().filter(|i| i%2==0).collect();
    let mut left = q_sort(odd);
    let mut right = q_sort(even);
    left.append(&mut right);
    left
}
pub fn par_evenoddsort(mut xs: Vec<i64>) -> Vec<i64>{
    let ys = xs.clone();
    let odd: Vec<i64> = xs.into_par_iter().filter(|i| i%2!=0).collect();
    let even: Vec<i64> = ys.into_par_iter().filter(|i| i%2==0).collect();
    let(mut left, mut right) = rayon::join(|| q_sort(odd), ||q_sort(even));
    left.append(&mut right);
    left
}

fn matrixmul(n: usize, e1: &Vec<Vec<f64>>, o: &Vec<Vec<f64>>) -> Vec<Vec<f64>>
{
    let mut row = (0..n).into_par_iter().map(move |i|{
        let erow = &e1[i];

        (i, sumrow(erow, o, n))
    }).collect::<Vec<(usize, Vec<f64>)>>();

    row.par_sort_by(|left, right| left.0.cmp(&right.0));
    row.into_par_iter().map(|(_, row)| row).collect()
}

fn sumrow(a: &Vec<f64>, b: &Vec<Vec<f64>>, n: usize) -> Vec<f64>
{
    (0..n).map(|x|
        (0..n).map(|y|
            a[y] * b[y][x]).sum())
        .collect::<Vec<f64>>()
}