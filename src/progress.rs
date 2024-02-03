use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[cfg(not(feature = "progress"))]
pub fn bar_create(_len: usize) -> (Sender<String>, Option<std::thread::JoinHandle<()>>) {
    let (sender, _): (Sender<String>, Receiver<String>) = mpsc::channel();

    (sender, None)
}

#[cfg(feature = "progress")]
pub fn bar_create(len: usize) -> (Sender<String>, Option<std::thread::JoinHandle<()>>) {
    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    (
        sender,
        Some(std::thread::spawn(move || {
            bar_create_internal(len, receiver);
        })),
    )
}

#[cfg(feature = "progress")]
fn bar_create_internal(len: usize, receiver: Receiver<String>) {
    let pb = indicatif::ProgressBar::new(len as u64);

    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:20}] {percent:>2}% ({pos}/{len}) {msg}")
            .unwrap(),
    );
    pb.tick();

    let (instruction_increment, instruction_finish) = ("INC".to_string(), "FINISH".to_string());

    loop {
        pb.tick();

        if let Ok(msg) = receiver.try_recv() {
            if msg.eq(&instruction_increment) {
                pb.inc(1);
                continue;
            }
            if msg.eq(&instruction_finish) {
                pb.finish();
                continue;
            }

            let m = format!("Redeeming code {}..", msg.replace("CODE ", ""));
            pb.set_message(m);
        };

        if pb.is_finished() {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    pb.finish();
}
