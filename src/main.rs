use midas::parse_evtx;
use midas::relational_midas;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = args[1].clone();
    let mut ev = parse_evtx::read(&filepath[..]);
    ev.sort_by(|a, b| a.time.cmp(&b.time));
    let first = &ev[0];
    let stime: u64 = first.time as u64;

    let mut midas: relational_midas::RelationalCore<u64> =
        relational_midas::RelationalCore::new(0.1, 0.001, 100_000);
    println!("IP\tUser\tTime\trtime\tScore");
    for e in ev.into_iter() {
        let (item, mut time) = e.to_scoreable();

        //convert time to 10s of minutes from start
        time = (time - stime) / 60;
        let s = midas.update(item, time);

        println!("{}\t{}\t{}\t{}\t{}", e.workstation, e.user, e.time, time, s);
    }
}
