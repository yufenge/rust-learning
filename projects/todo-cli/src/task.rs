use chrono::{Local, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::storage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    content: String,
    status: TaskStatus,
    create_time: i64,
    done_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    /**
     * 带进行
     */
    PENDING,
    /**
     * 已完成
     */
    DONE,
}

#[derive(Default)]
pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self) {
        self.tasks = storage::load::<Vec<Task>>().unwrap_or_default();
        self.next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    }

    fn filter(&self, _status: Option<TaskStatus>) -> Vec<&Task> {
        match _status {
            Some(s) => self.tasks.iter().filter(|o| o.status == s).collect(),
            None => self.tasks.iter().collect(),
        }
    }

    fn save(&self) {
        let res = storage::save(&self.tasks);
        if res.is_err() {
            eprintln!("持久化失败:{:?}", res.err());
        }
    }

    pub fn count(&self, _status: Option<TaskStatus>) -> usize {
        self.filter(_status).len()
    }

    pub fn add(&mut self, _content: &str) {
        let id = self.next_id;
        let task = Task {
            id,
            content: _content.to_string(),
            status: TaskStatus::PENDING,
            create_time: Local::now().timestamp(),
            done_time: None,
        };

        self.tasks.push(task);
        self.next_id += 1;

        self.save();
        println!("添加成功:id:{}", id)
    }

    pub fn list(&self, _status: Option<TaskStatus>) {
        print_tasks_pretty(self.filter(_status));
    }

    pub fn done(&mut self, id: u32) {
        match self.tasks.iter_mut().find(|o| o.id == id) {
            Some(task) => {
                task.status = TaskStatus::DONE;
                println!("[{id}]已完成")
            }
            None => {
                println!("未查找到id为[{id}]的数据")
            }
        }
        self.save();
    }

    pub fn remove(&mut self, id: u32) {
        match self.tasks.iter().position(|o| o.id == id) {
            Some(index) => {
                self.tasks.remove(index);
                println!("[{id}]已移除")
            }
            None => {
                println!("未查找到id为[{id}]的数据");
            }
        }
        self.save();
    }

    pub fn clear(&mut self) {
        self.tasks.clear();
        print!("任务列表已清除");

        self.save();
    }
}

fn print_tasks_pretty(tasks: Vec<&Task>) {
    if tasks.is_empty() {
        return println!("\t没有更多数据");
    }
    for task in tasks {
        let Task {
            id,
            status,
            content,
            create_time,
            ..
        } = task;
        let status_icon = match status {
            TaskStatus::DONE => "✅",
            TaskStatus::PENDING => "⏳",
        };
        let create_time_display = Utc
            .timestamp_millis_opt(*create_time)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S");

        println!("#{id} {status_icon}  {content}  {create_time_display}")
    }
}
