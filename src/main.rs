
use std::time::{Duration, Instant};
use rand::Rng;

fn timed<R, F>(f: F) -> (R, Duration) where F: Fn() -> R {
    let starting_point = Instant::now();
    let res = f();
    (res, starting_point.elapsed())
}
fn main() {
    // let mut rng = rand::thread_rng();
    // let vals: Vec<u64> = (0..1000).map(|_| rng.gen_range(0, 2000)).collect();
    // let mut list: LinkedList<u64> = LinkedList::new();
    // vals.iter().for_each(|i| list.push_back(*i));
    // let (sum, t) = timed(|| sorting::q_sort(vals.clone()));
    // println!("normal quick sort:Vec = {}s",t.as_secs_f64());
    // let (sum, t) = timed(|| sorting::par_q_sort(vals.clone()));
    // println!("parallel quick sort:Vec = {}s",t.as_secs_f64());
    // let (sum, t) = timed(|| sorting::merge_sort(vals.clone()));
    // println!("normal merge sort:Vec = {}s",t.as_secs_f64());
    // let (sum, t) = timed(|| sorting::par_merge_sort(vals.clone()));
    // println!("parallel merge sort:Vec = {}s",t.as_secs_f64());
    // let (sum, t) = timed(|| sorting_linked_list::q_sort(list.clone()));
    // println!("normal quick sort: Linked-List = {}s",t.as_secs_f64());
    // let (sum, t) = timed(|| sorting_linked_list::par_q_sort(list.clone()));
    // println!("parallel quick sort: Linked-List = {}s",t.as_secs_f64());
    // let (sum, t) = timed(|| sorting_linked_list::merge_sort(list.clone()));
    // println!("normal merge sort: Linked-List = {}s",t.as_secs_f64());
    // let (sum, t) = timed(|| sorting_linked_list::par_merge_sort(list.clone()));
    // println!("parallel merge sort: Linked-List = {}s",t.as_secs_f64());
}

