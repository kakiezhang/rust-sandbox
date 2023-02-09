use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let m = Arc::new(Mutex::new(HashMap::<&str, usize>::new()));

    for _i in 1..=30 {
        let cm = m.clone();
        thread::spawn(move || {
            let mut g = cm.lock().unwrap();
            let mg = &mut *g;
            let v = mg.entry("hello").or_insert(1);
            *v += 1;
        });
    }

    thread::sleep(Duration::from_millis(100));

    println!("m: {:?}", m.lock().unwrap());
}
