use std::os::windows::thread;
use std::time::{Instant, Duration};

use simple_moving_average::{SingleSumSMA, SMA};


fn do_render() {

}

fn main() {
    let mut X;
    let mut Y = Instant::now();
    // let mut start = Epoch::now().unwrap();
    let mut curr;

    let start = Instant::now();
    let mut target = Instant::now();

    let mut microwait = 0;

    let mut avg = SingleSumSMA::<_, u128, 60>::new();

    loop {
        X = Duration::from_micros(match microwait {
            0 => 16666,
            1 => 16667,
            2 => 16667,
            _ => unreachable!()
        });
        // target += Duration::from_micros(16666);
        target += X;

        // Y = match state {
        //     FrameLimiterState::NotLimited(curr) => {
        //         curr
        //     },
        //     FrameLimiterState::Limited(prev) => {
        //         prev + X
        //     }
        // };


        do_render();

        curr = Instant::now();

        if curr > (Y + X) {
            Y = Y + X;
            // state = FrameLimiterState::NotLimited(curr);
        } else {
            let sleep_time = (Y + X) - curr;
            println!("sleep_time: {sleep_time:?}");
            std::thread::sleep(sleep_time.into());
            Y = Y + X;
        }
        let elapsed = Instant::now();

        avg.add_sample((elapsed - target).as_micros());

        println!("target_time: {:?}, actual_time: {:?}, diff: {:?}, avg_diff: {:?}", target.duration_since(start),
             elapsed.duration_since(start),
             elapsed - target,
             Duration::from_micros(avg.get_average() as u64));
        microwait = (microwait + 1) % 3;
    }
    println!("Hello, world!");
}
