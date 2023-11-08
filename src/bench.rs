use std::time::Instant;

pub fn bench(f: impl FnOnce()) -> u128 {
    let now: Instant = Instant::now();

    f();

    println!(
        "[{ans}] ver1 for packet_size: [{packet_size: >2}] | time: [{: >12?}]",
        now.elapsed()
    );
    todo!()
}
