
pub mod ray_worker;

use std::thread::{JoinHandle, self};
use std::sync::mpsc::Sender;
use crate::camera::Camera;
use crate::world::World;
use self::ray_worker::{RayWorker, RayResult, RayWorkerSettings};

pub struct RayWorkerManager {
    thread_handles: Vec<JoinHandle<()>>,
    worker_nums: usize
}

impl RayWorkerManager {
    pub fn new() -> RayWorkerManager {
        RayWorkerManager {
            thread_handles: Vec::new(),
            worker_nums: 0
        }
    }

    pub fn start_worker(&mut self, world: World, camera: Camera, pixel_sender: Sender<RayResult>, worker_settings: RayWorkerSettings) {
        let thread_handle = thread::spawn(move || {
            let mut ray_worker = RayWorker::new(
                    world,  
                    camera,
                    pixel_sender, 
                    worker_settings
                );
            ray_worker.run();
        });

        self.thread_handles.push(thread_handle);
        self.worker_nums += 1;
    }

    pub fn join_workers(&mut self) {
        while !self.thread_handles.is_empty() {
            let popped_handle = self.thread_handles.pop();
            match popped_handle {
                Some(handle) => {
                    handle.join().unwrap();
                }
                _ => { }
            }
        }

        self.worker_nums = 0;
    }

    pub fn get_worker_nums(&self) -> usize {
        self.worker_nums
    }

}


