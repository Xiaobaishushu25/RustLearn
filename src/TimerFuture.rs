#[cfg(test)]
mod test_timer_future {
    use futures::future::{ok, BoxFuture, Shared};
    use futures::task::{waker_ref, ArcWake};
    use futures::FutureExt;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Waker};
    use std::thread;
    use std::time::Duration;

    ///https://rustcc.cn/article?id=720e246e-4378-47c2-ad7d-76ba05a1b05e

    struct SharedState {
        completed: bool,
        waker: Option<Waker>,
    }
    struct TimerFuture {
        shared_state: Arc<Mutex<SharedState>>,
    }

    impl Future for TimerFuture {
        type Output = ();
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut shared_stated = self.shared_state.lock().unwrap();
            if shared_stated.completed {
                Poll::Ready(())
            } else {
                //没有准备好就保存当前上下文的waker，用于下一次wake
                shared_stated.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
    impl TimerFuture {
        pub fn new(duration: Duration) -> Self {
            let shared_state = Arc::new(Mutex::new(SharedState {
                completed: false,
                //自己初始化的时候不提供waker，这个waker是由执行器创建并在context中传入的
                waker: None,
            }));
            let thread_shared_state = shared_state.clone();
            thread::spawn(move || {
                thread::sleep(duration);
                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.completed = true;
                //任务完成了，调用wake让执行器来poll自己 获取结果
                //调用 waker 的 wake 方法，注意到在前面我们为 Task 实现了 ArcWake Trait；
                //因此调用 waker 的 wake 方法后，会执行 Task 对应的方法
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake()
                }
            });
            TimerFuture { shared_state }
        }
    }
    struct Executor {
        //执行器，从ready队列取出任务执行
        ready_queue: Receiver<Arc<Task>>,
    }
    struct Spawner {
        //把一个任务包装好发送到执行器
        task_sender: SyncSender<Arc<Task>>,
    }

    fn new_executor_and_spawner() -> (Executor, Spawner) {
        const MAX_QUEUE_TASKS: usize = 10000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUE_TASKS);
        (Executor { ready_queue }, Spawner { task_sender })
    }

    impl Spawner {
        //包装一个future发送到执行器的ready队列
        fn spawn<T>(&self, future: T)
        where
            T: Future<Output = ()> + 'static + Send,
        {
            let future_box = future.boxed();
            let task = Arc::new(Task {
                future: Mutex::new(Some(future_box)),
                task_sender: self.task_sender.clone(),
            });
            println!("发送任务到执行器");
            self.task_sender.send(task).expect("任务队列已满");
        }
    }
    struct Task {
        future: Mutex<Option<BoxFuture<'static, ()>>>,
        // 可以将该任务自身发送到任务通道中，等待执行器的poll
        task_sender: SyncSender<Arc<Task>>,
    }

    impl ArcWake for Task {
        //调用wake时，说明有进步或者准备完成了，把自己发送到任务通道等待poll
        fn wake_by_ref(arc_self: &Arc<Self>) {
            let arc = arc_self.clone();
            println!("调用wake_by_ref");
            arc_self.task_sender.send(arc).expect("任务队列已满");
        }
    }

    impl Executor {
        ///通过 take 将 Task的所有权取出到future变量中，针对Task创建一个 Waker，并传入 Context 中；
        ///最后，调用 future 的 poll 函数，推动任务执行，并且如果执行 poll后仍然是 Pending 状态，
        /// 则再次将 future 放回 Executor 的队列中去；
        fn run(&self) {
            //ready_queue.recv()会在没有消息时阻塞，并在有消息到来执行，他会从队列里面取出来有所有权的Task
            //当有准备好的task时，poll它
            while let Ok(task) = self.ready_queue.recv() {
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    let waker = waker_ref(&task);
                    //生成一个上下文用于轮询
                    let context = &mut Context::from_waker(&waker);
                    if future.as_mut().poll(context).is_pending() {
                        *future_slot = Some(future)
                    }
                }
            }
        }
    }

    #[test]
    fn test() {
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
