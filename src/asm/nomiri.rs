use asm_rs::{Arch, Assembler};

use std::mem::transmute;
use crate::{pre::*, asm::Asm};

#[test]
fn map_instrs() -> R<()> {
    let mut asm = Asm::new();
    let fun = asm.emit_fun(
        "exit",
        "
        .equ EXIT, 60
        mov eax, EXIT
        xor edi, edi
        syscall
        ",
    )?;

    let exe = asm.exe()?;
    let f = exe.fun0::<_, ()>("exit")?;
    f();

    Ok(())
}
