//! Implementation of [`TrapContext`]

use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
/// 整个 trap 过程中需要保存的寄存器的抽象, 需要注意几个地方:
/// 1. x 是 32 个通用寄存器, 其中约定 x[2] 是 sp, x[4] 是 tp
/// 2. sstatus 是特权级全局信息寄存器, 核心在 SPP 字段记录了 trap 前的特权级
/// 3. sepc 是特权级寄存器, 记录的是 trap 前执行到的地址, 因此还原回来之后需要往后挪一个地址才能正确执行
/// 4. 该结构体保存的是应用的寄存器信息
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    /// 设置栈指针
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    /// 初始化 app 的寄存器为空, 然后保存其栈地址以及上次执行到的命令地址
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}