
use std::thread;
use std::sync::{
    atomic,
    Arc,
    mpsc::{self, Sender, Receiver, }};
use super::task::Task;

struct Worker {
    handle: thread::JoinHandle<()>,
    sender: Sender<WorkerMessage>,
    task_count: Arc<atomic::AtomicUsize>
}

enum WorkerMessage {
    Work(Box<dyn Task + Send>),
    Exit
}

pub struct WorkerManager {
    workers: Vec<Worker>
}

impl WorkerManager {
    pub fn new() -> WorkerManager {
        let num = num_cpus::get();
        let mut workers = Vec::new();
        for i in 0 .. num {
            let (sender, receiver): (Sender<WorkerMessage>, Receiver<WorkerMessage>) = mpsc::channel();
            let task_count = Arc::new(atomic::AtomicUsize::new(0));

            let task_count_thread = task_count.clone();
            let handle = thread::spawn(move || {
                worker_main(receiver, task_count_thread.clone());
            });

            workers.push(Worker{ 
                handle, 
                sender, 
                task_count: task_count.clone() });
        }

        WorkerManager {
            workers
        }
    }

    pub fn add_task(&mut self, task: Box<dyn Task + Send>) {
        let mut min_index = 0;
        for i in 1 .. self.workers.len() {
            let min_count = self.workers[min_index].task_count.load(atomic::Ordering::SeqCst);
            let cur_count = self.workers[i].task_count.load(atomic::Ordering::SeqCst);
            if cur_count < min_count {
                min_index = i;
            }
        }

        self.workers[min_index].task_count.fetch_add(1, atomic::Ordering::SeqCst);
        self.workers[min_index].sender.send(WorkerMessage::Work(task)).expect("error task send!");
    }
}

// finalizing worker thread
impl Drop for WorkerManager {
    fn drop(&mut self) {
        for i in 0 .. self.workers.len() {
            self.workers[i].sender.send(WorkerMessage::Exit).expect("error task send!");
        }

        while self.workers.is_empty() == false {
            let worker = self.workers.pop().unwrap();
            worker.handle.join().expect("error worker join!");
        }
    }
}

fn worker_main(receiver: mpsc::Receiver<WorkerMessage>, task_count: Arc<atomic::AtomicUsize>) {
    loop {
        // wait for task
        let recv = receiver.recv().unwrap();

        match recv {
            WorkerMessage::Work(mut task) => {
                task.do_work();
                task.finish_work();
                task_count.fetch_sub(1, atomic::Ordering::SeqCst);
            }
            WorkerMessage::Exit => { break; }
        }
    }
}

