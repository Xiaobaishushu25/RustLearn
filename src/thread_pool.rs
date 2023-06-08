#[cfg(test)]
mod test_thread_pool{
    use std::sync::{Arc, mpsc, Mutex};
    use std::sync::mpsc::Receiver;
    use std::thread;
    use std::thread::Thread;
    use std::time::Duration;

    #[test]
    fn test(){
        let pool = ThreadPool::new(5);
        for _ in 0..50{
            pool.execute(||{
                println!("  hello")
            })
        }
        thread::sleep(Duration::from_millis(1000));
    }
    struct Worker{
        id:usize,
        thread:thread::JoinHandle<()>,
    }
    impl Worker{
        pub fn new(id:usize,receiver:Arc<Mutex<Receiver<Job>>>)->Self{
            let thread = thread::spawn(move || {
                loop {
                    //死循环，没有任务时会阻塞在这里，接收到任务时执行。
                    let job = receiver.lock().unwrap().recv().unwrap();
                    print!("Worker{id} get a job ");
                    job();
                }
            });
            Worker{id,thread}
        }

    }
    // struct Job;
    type Job = Box<dyn FnOnce() ->() +Send +'static>;
    struct ThreadPool{
        workers:Vec<Worker>,
        sender:mpsc::Sender<Job>
    }
    impl ThreadPool{
        pub fn new(size:usize) -> Self{
            assert!(size>0);
            let (sender,receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            for id in 0..size{
                workers.push(Worker::new(id,Arc::clone(&receiver)));
            };
            ThreadPool{workers,sender}
        }
        pub fn execute<F>(&self,f:F)
        where F:FnOnce() ->() +Send +'static,{
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }
}