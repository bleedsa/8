#![feature(macro_metavar_expr)]

use eight::{pre::*, asm::Asm, vm::VM, mkasmfuns};

fn main() -> R<()> {
    let mut vm = VM::new();

    let add = mkasmfuns!(vm => [
        fn add(i32, i32) -> i32
        => "
           mov eax, edi
           add eax, esi
           ret
           ";
    ]);

    println!("{}", add(10, 5));

    Ok(())
}
