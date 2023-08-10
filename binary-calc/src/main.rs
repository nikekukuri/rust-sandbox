use std::mem::size_of;

fn print_bin(target: i32) {
    let char_bit = 4;
    let mut mask = 1 << (size_of::<i32>() * char_bit - 1);

    print!("{}", target & mask);
    loop {
        mask = mask >> 1;
        if mask == 0 {
            break;
        }

        let flag = if target & mask != 0 {
            '1'
        } else {
            '0'
        };
        print!("{}", flag);
    }
}
fn main() {
    let a: i32 = 9;
    println!("{}", size_of::<i32>());

    print_bin(a);
}
