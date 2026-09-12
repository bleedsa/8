use eight::asm::mmap_exec;

fn main() {
    unsafe {
        let map = mmap_exec(4096).unwrap();

        for i in 0..4096 {
            *map.add(i) = i as u8;
            assert_eq!(*map.add(i), i as u8);
        }
    }
}
