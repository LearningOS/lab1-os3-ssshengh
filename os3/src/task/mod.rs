mod task_context;
mod task_status;
mod switch;

use lazy_static::lazy_static;
use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use crate::sync::UPSafeCell;
use crate::task::switch::__switch;
use crate::task::task_context::TaskContext;
pub use crate::task::task_status::TaskStatus;


pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<RawTaskManager>
}

pub struct RawTaskManager {
    tasks: [Task; MAX_APP_NUM],
    current_task: usize,
}


#[derive(Copy, Clone)]
pub struct Task {
    status: TaskStatus,
    cx: TaskContext
}


lazy_static!{
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [Task{
            status: TaskStatus::Uninit,
            cx: TaskContext::init_zero()
        }; MAX_APP_NUM];

        tasks.iter_mut().enumerate().for_each(|(i, t)| {
            t.status = TaskStatus::Ready;
            t.cx = TaskContext::init_with_ra_and_sp(init_app_cx(i));
        });
        let up_safe_inner = unsafe {
            UPSafeCell::new(
                RawTaskManager {
                    tasks,
                    current_task: 0
                }
            )
        };
        TaskManager {
            num_app,
            inner: up_safe_inner
        }
    };
}

impl TaskManager {
    pub fn run_first_task(&self) -> ! {
        let mut inner_mut = self.inner.exclusive_access();
        let first_task = &mut inner_mut.tasks[0];
        first_task.status = TaskStatus::Running;
        let p_cx = &mut first_task.cx;
        let next_task_ptr = p_cx as *const TaskContext;
        drop(inner_mut);
        let mut _unused = TaskContext::init_zero();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_ptr);
        }

        panic!("Run first task wrong!");
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].status = TaskStatus::Exit;
    }

    /// Find next task to run and return task id.
    ///
    /// In this case, we only return the first `Ready` task in task list.
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        // 这一句是往后的循环查找
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].status == TaskStatus::Ready)
    }

    /// Switch current `Running` task to the task we have found,
    /// or there is no `Ready` task and we can exit with all applications completed
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].cx as *const TaskContext;
            drop(inner);
            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            panic!("All applications completed!");
        }
    }

}

/// Run the first task in task list.
pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

/// Switch current `Running` task to the task we have found,
/// or there is no `Ready` task and we can exit with all applications completed
fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

/// Change the status of current `Running` task into `Ready`.
fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

/// Change the status of current `Running` task into `Exited`.
fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}


// pub fn get_task() -> TaskManager {
//     let num_app = get_num_app();
//     let mut tasks = [Task{
//         status: TaskStatus::Uninit,
//         cx: TaskContext::init_zero()
//     }; MAX_APP_NUM];
//
//     tasks.iter_mut().enumerate().for_each(|(i, t)| {
//         t.status = TaskStatus::Ready;
//         t.cx = TaskContext::init_with_ra_and_sp(init_app_cx(i));
//     });
//     let up_safe_inner = unsafe {
//         UPSafeCell::new(
//             RawTaskManager {
//                 tasks,
//                 current_task: 0
//             }
//         )
//     };
//     TaskManager {
//         num_app,
//         inner: up_safe_inner
//     }
// }

