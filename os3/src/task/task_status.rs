#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    Uninit,
    Ready,
    Running,
    Exit,
}