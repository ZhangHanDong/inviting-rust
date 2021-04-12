use super::*;

pub struct Task {
    future: Mutex<Option<BoxFuture<'static, io::Result<()>>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("failed to send");
    }
}

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let mut context = Context::from_waker(&*waker);
                if let Poll::Pending = future.as_mut().poll(&mut context) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, fut: impl Future<Output = io::Result<()>> + 'static + Send) {
        let fut = fut.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(fut)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("failed to send");
    }
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (task_sender, ready_queue) = sync_channel(10000);
    (Executor { ready_queue }, Spawner { task_sender })
}