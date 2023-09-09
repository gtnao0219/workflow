use std::{collections::HashMap, time::Instant};

struct FlowJob {
    task_jobs: HashMap<usize, Box<dyn TaskJob>>,
    dependencies: HashMap<usize, Vec<usize>>,
}
impl FlowJob {
    fn run(&mut self) -> () {
        loop {
            for (_, task_job) in self.task_jobs.iter_mut() {
                task_job.poll();
            }
            let mut ready_ids = Vec::new();
            let mut done = true;
            for (task_id, task_job) in self.task_jobs.iter() {
                if task_job.get_status() != TaskStatus::Done {
                    done = false;
                }
                if task_job.get_status() == TaskStatus::Pending {
                    if let Some(dependencies) = self.dependencies.get(task_id) {
                        let ready = if dependencies.is_empty() {
                            true
                        } else {
                            dependencies.iter().all(|dependency| {
                                self.task_jobs
                                    .get(dependency)
                                    .map(|task_job| task_job.get_status() == TaskStatus::Done)
                                    .unwrap_or(false)
                            })
                        };
                        if ready {
                            ready_ids.push(*task_id);
                        }
                    }
                }
            }
            for task_id in ready_ids {
                if let Some(task_job) = self.task_jobs.get_mut(&task_id) {
                    task_job.run();
                }
            }
            if done {
                break;
            }
        }
        println!("Done");
    }
}
trait TaskJob {
    fn run(&mut self);
    fn get_status(&self) -> TaskStatus;
    fn poll(&mut self);
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum TaskStatus {
    Pending,
    Running,
    Done,
}
struct EchoTaskJob {
    status: TaskStatus,
    value: String,
}
impl EchoTaskJob {
    fn new(value: &str) -> Self {
        EchoTaskJob {
            status: TaskStatus::Pending,
            value: value.to_string(),
        }
    }
}
struct SleepTaskJob {
    status: TaskStatus,
    secs: u64,
    start: Option<Instant>,
}
impl SleepTaskJob {
    fn new(value: u64) -> Self {
        SleepTaskJob {
            status: TaskStatus::Pending,
            secs: value,
            start: None,
        }
    }
}
impl TaskJob for EchoTaskJob {
    fn run(&mut self) {
        self.status = TaskStatus::Running;
        println!("{}", self.value);
        self.status = TaskStatus::Done;
    }
    fn get_status(&self) -> TaskStatus {
        self.status
    }
    fn poll(&mut self) -> () {}
}
impl TaskJob for SleepTaskJob {
    fn run(&mut self) {
        self.status = TaskStatus::Running;
        self.start = Some(Instant::now());
    }
    fn get_status(&self) -> TaskStatus {
        self.status
    }
    fn poll(&mut self) -> () {
        if let Some(start) = self.start {
            if start.elapsed().as_secs() >= self.secs {
                self.status = TaskStatus::Done;
            }
        }
    }
}

fn main() {
    let task_job1 = Box::new(EchoTaskJob::new("Hello"));
    let task_job2 = Box::new(SleepTaskJob::new(3));
    let task_job3 = Box::new(EchoTaskJob::new("World"));
    let mut task_jobs: HashMap<usize, Box<dyn TaskJob>> = HashMap::new();
    let mut dependencies = HashMap::new();
    task_jobs.insert(0, task_job1);
    task_jobs.insert(1, task_job2);
    task_jobs.insert(2, task_job3);
    dependencies.insert(0, vec![]);
    dependencies.insert(1, vec![0]);
    dependencies.insert(2, vec![1]);
    let mut flow_job = FlowJob {
        task_jobs,
        dependencies,
    };
    flow_job.run();
}
