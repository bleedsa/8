use eight::{M::M, V, pre::*};

fn main() -> R<()> {
    #[cfg(miri)]
    eight::init();

    let o: M = V!("+", 1i32, 1i32).to();
    println!("{o:?}");

    let n = V!(".", &o, 1i32);
    println!("{n}");

    Ok(())
}
