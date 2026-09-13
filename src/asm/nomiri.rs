use crate::{pre::*, asm::Asm};

#[test]
fn map_exec_fun0() -> R<()> {
    let mut asm = Asm::new();
    asm.emit_fun::<0, _, _>(
        "fun0",
        "
        xor eax, eax
        inc eax
        ret
        ",
    )?;

    let e = asm.exe()?;
    let f = unsafe { e.fun0::<_, i32>("fun0")? };

    assert_eq!(1i32, f());

    Ok(())
}

#[test]
fn map_exec_fun1() -> R<()> {
    let mut asm = Asm::new();
    asm.emit_fun::<1, _, _>(
        "inc",
        "
        inc edi
        mov eax, edi
        ret
        ",
    )?;

    let e = asm.exe()?;
    let f = unsafe { fun!(e, fn(i32) -> i32 = "inc") };

    assert_eq!(2i32, f(1));

    Ok(())
}
