use std::{thread::sleep, time::Duration};

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        sleep(Duration::from_millis(50));
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
