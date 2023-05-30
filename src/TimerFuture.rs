#[cfg(test)]
mod test_timer_future{
    use std::fs::FileType;
    use std::future::Future;
    use std::io::Take;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
    use std::task::{Context, Poll, Waker};
    use std::thread;
    use std::thread::ThreadId;
    use std::time::Duration;
    use futures::future::{BoxFuture, ok, Shared};
    use futures::FutureExt;
    use futures::task::{ArcWake, waker_ref};

    struct SharedState{
        completed:bool,
        waker:Option<Waker>
    }
    struct TimerFuture{
        shared_state:Arc<Mutex<SharedState>>
    }

    impl Future for TimerFuture {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut shared_stated = self.shared_state.lock().unwrap();
            if shared_stated.completed{
                Poll::Ready(())
            }else {
                shared_stated.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
    impl TimerFuture{
        pub fn new(duration:Duration) -> Self{
            let shared_state = Arc::new(Mutex::new(SharedState {
                completed: false,
                waker: None
            }));
            let thread_shared_state = shared_state.clone();
            thread::spawn(move||{
                thread::sleep(duration);
                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.completed = true;
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake()
                }
            });
            TimerFuture{shared_state}
        }
    }
    struct Executor{
        ready_queue:Receiver<Arc<Task>>
    }
    struct Spawner{
        task_sender:SyncSender<Arc<Task>>
    }

    impl Spawner {
        fn spawn<T>(&self,future:T)
            where
                T:Future<Output = ()>+'static+Send {
            let future_box = future.boxed();
            let task = Arc::new(Task {
                future: Mutex::new(Some(future_box)),
                task_sender: self.task_sender.clone()
            });
            self.task_sender.send(task).expect("任务队列已满");
        }
    }
    struct Task{
        future:Mutex<Option<BoxFuture<'static,()>>>,
        /// 可以将该任务自身放回到任务通道中，等待执行器的poll
        task_sender: SyncSender<Arc<Task>>,
    }

    impl ArcWake for Task{
        fn wake_by_ref(arc_self: &Arc<Self>) {
            let arc = arc_self.clone();
            arc_self.task_sender.send(arc).expect("任务队列已满");
        }
    }

    impl Executor {
        fn run(&self){
            while let Ok(task) = self.ready_queue.recv() {
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take(){
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&*waker);
                    if future.as_mut().poll(context).is_pending(){
                        *future_slot = Some(future)
                    }
                }
            }
        }
    }
    fn new_executor_and_spawner() -> (Executor,Spawner){
        const MAX_QUEUE_TASKS:usize = 10000;
        let (task_sender,ready_queue) = sync_channel(MAX_QUEUE_TASKS);
        (Executor{ready_queue},Spawner{task_sender})
    }
    
    #[test]
    fn test(){
        let (executor, spawner) = new_executor_and_spawner();

        // 生成一个任务
        spawner.spawn(async {
            println!("howdy!");
            // 创建定时器Future，并等待它完成
            TimerFuture::new(Duration::new(2, 0)).await;
            println!("done!");
        });

        // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
        drop(spawner);

        // 运行执行器直到任务队列为空
        // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
        executor.run();
    }
}