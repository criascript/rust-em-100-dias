
pub fn while_tradicional() {
    let mut i = 0;
    while i < 10 {
        println!("while tradicional");
        i += 1;
    }
}

pub fn for_tradicional() {
    let range = 0..10;
    for i in range {
        println!("for tradicional");
    }
}

pub fn loop_ate_10() {
    let mut i = 0;
    loop {
        i += 1;
        
        if i == 10 {
            break;
        }

        println!("i: {}", i);
        
    }
}
