use eight::{M::M, V, pre::*, asm::Asm};

fn main() -> R<()> {
    let mut asm = Asm::new();
    let fun = asm.emit_fun0(
        "exit",
        "
        .equ EXIT, 60
        mov eax, EXIT
        mov edi, 0
        syscall
        ",
    )?;

    let exe = asm.exe()?;
    let f: extern "C" fn() = exe.fun0("exit")?;
    f();

    Ok(())
}
