use crate::{pre::*, asm::Asm};

#[test]
fn map_instrs() -> R<()> {
    let mut asm = Asm::new();
    asm.emit_fun0(
        "fun0",
        "
        xor eax, eax
        inc eax
        ret
        ",
    )?;

    let exe = asm.exe()?;
    let f = exe.fun0::<_, i32>("fun0")?;

    assert_eq!(1i32, f());

    Ok(())
}
