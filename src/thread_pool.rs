#[cfg(test)]
mod test_thread_pool {
    use std::sync::mpsc::Receiver;
    ///实现了一个线程池：https://course.rs/advance-practice1/graceful-shutdown.html
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    #[test]
    fn test() {
        let pool = ThreadPool::new(5);
        for _ in 0..50 {
            pool.execute(|| println!("  hello"))
        }
        println!("Shutting down.");
        // thread::sleep(Duration::from_millis(1000));
    }
    type Job = Box<dyn FnOnce() -> () + Send + 'static>;
    struct Worker {
        id: usize,
        // thread:thread::JoinHandle<()>,
        thread: Option<thread::JoinHandle<()>>,
    }
    impl Worker {
        pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
            let thread = thread::spawn(move || {
                loop {
                    //死循环，没有任务时会阻塞在这里，接收到任务时执行。
                    let job = receiver.lock().unwrap().recv();
                    //关闭 sender 后，将关闭对应的 channel，意味着不会再有任何消息被发送。随后，所有的处于无限 loop 的接收端将收到一个错误
                    match job {
                        Ok(job) => {
                            println!("Worker{id} get a job ");
                            job();
                        }
                        Err(e) => {
                            println!("Worker {id} disconnected; shutting down.");
                            break;
                        }
                    }
                }
            });
            Worker {
                id,
                thread: Some(thread),
            }
        }
    }
    struct ThreadPool {
        workers: Vec<Worker>, //线程队列
        // sender:mpsc::Sender<Job>
        sender: Option<mpsc::Sender<Job>>, //消息发送端
    }
    impl ThreadPool {
        pub fn new(size: usize) -> Self {
            assert!(size > 0);
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                //每一个线程都会带着同一个接收器
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }
        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() -> () + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.take());
            // for mut worker in &mut self.workers {
            for mut worker in &mut self.workers {
                println!("Shutting down the Worker {}", worker.id);
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
                // worker.thread.join().unwrap();
            }
        }
    }
}
