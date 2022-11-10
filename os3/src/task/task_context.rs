/// 应用在 S 级进行切换时需要保存的所有信息
/// 1. 用于函数返回的寄存器 ra
/// 2. 记录 app 单独的栈的 sp
/// 3. 函数调用寄存器
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    x: [usize; 12]
}

impl TaskContext {
    /// 所有地址均初始化为 0
    pub fn init_zero() -> Self {
        Self {
            ra: 0,
            sp: 0,
            x: [0; 12]
        }
    }
    /// 初始化应用, 设置 switch 之后返回地址为 __restore, 设置其专用内核栈
    pub fn init_with_ra_and_sp(kstack_address: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_address,
            x: [0; 12],
        }
    }
}

