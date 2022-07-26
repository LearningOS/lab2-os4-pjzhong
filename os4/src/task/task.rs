use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,  //未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}
