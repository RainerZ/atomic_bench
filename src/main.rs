use std::sync::atomic::{Ordering, fence};
use std::thread;
use std::{hint::black_box, sync::atomic::AtomicU64};

static A: AtomicU64 = AtomicU64::new(0);

fn main() {
    black_box(&A);

    let start = std::time::Instant::now();
    for _ in 0..1000000000 {
        black_box(A.load(Ordering::Relaxed));
    }
    println!("load {:?}", start.elapsed());

    let start = std::time::Instant::now();
    for i in 0..1000000000 {
        black_box(A.store(i, Ordering::Relaxed));
    }
    println!("store {:?}", start.elapsed());

    //------------------------------------------------------------------
    // let start = std::time::Instant::now();
    // for _ in 0..1000000000 {
    //     black_box(A.load(Ordering::Relaxed));
    //     std::hint::spin_loop();
    // }
    // println!("spin_loop{ :?}", start.elapsed());

    //------------------------------------------------------------------
    // thread::spawn(|| {
    //     loop {
    //         black_box(A.load(Ordering::Relaxed));
    //     }
    // });
    // let start = std::time::Instant::now();

    // for _ in 0..1000000000 {
    //     black_box(A.load(Ordering::Relaxed));
    // }

    // println!("relaxed background load {:?}", start.elapsed());

    //------------------------------------------------------------------
    // thread::spawn(|| {
    //     loop {
    //         black_box(A.store(1, Ordering::Relaxed));
    //     }
    // });
    // let start = std::time::Instant::now();
    // for _ in 0..1000000000 {
    //     black_box(A.load(Ordering::Relaxed));
    // }
    // println!("relaxed background store {:?}", start.elapsed());

    //------------------------------------------------------------------
    thread::spawn(|| {
        loop {
            black_box(A.store(1, Ordering::Release));
        }
    });
    let start = std::time::Instant::now();
    for _ in 0..1000000000 {
        black_box(A.load(Ordering::Acquire));
    }
    println!("Release/Acquire background store {:?}", start.elapsed());

    //------------------------------------------------------------------

    thread::spawn(|| {
        loop {
            black_box(A.store(1, Ordering::SeqCst));
        }
    });

    let start = std::time::Instant::now();
    for _ in 0..1000000000 {
        black_box(A.load(Ordering::SeqCst));
    }
    println!("SeqCst background store {:?}", start.elapsed()); // thread::spawn(|| {

    let start = std::time::Instant::now();
    for _ in 0..1000000000 {
        black_box(A.load(Ordering::Relaxed));
        fence(Ordering::SeqCst);
    }
    println!("SegCst Fence background store {:?}", start.elapsed());
}
