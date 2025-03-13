use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        println!("{} is trying to eat...", &self.name);
        let _left = self.left_fork.lock().unwrap();
        let _right = self.right_fork.lock().unwrap();

        // Pick up forks...
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // フォークを作成する
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    let (tx, rx) = mpsc::channel();
    for (i, &name) in PHILOSOPHERS.iter().enumerate() {
        let tx = tx.clone();
        // 哲学者を作成する
        let mut philosopher = Philosopher {
            name: name.to_string(),
            thoughts: tx,
            left_fork: Arc::clone(&forks[i]),
            right_fork: Arc::clone(&forks[(i + 1) % forks.len()]),
        };

        if i == PHILOSOPHERS.len() - 1 {
            std::mem::swap(&mut philosopher.left_fork, &mut philosopher.right_fork);
        }

        // それぞれの哲学者が思索と食事を 100 回行うようにする
        thread::spawn(move || {
            for _ in (0..100) {
                philosopher.eat();
                philosopher.think();
            }
        });
    }

    // 哲学者の思索を出力する
    for thought in rx {
        println!("{thought}");
    }
}
