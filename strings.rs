pub fn inttostr(mut num: i32) -> String {
    let mut res = String::from(" ");
    while num  != 0 {
        let last = num % 10;
        match last {
            1 => { res.push('1'); num = num / 10;},
            2 => { res.push('2'); num = num / 10;},
            3 => { res.push('3'); num = num / 10;},
            4 => { res.push('4'); num = num / 10;},
            5 => { res.push('5'); num = num / 10;},
            6 => { res.push('6'); num = num / 10;},
            7 => { res.push('7'); num = num / 10;},
            8 => { res.push('8'); num = num / 10;},
            9 => { res.push('9'); num = num / 10;},
            _ => {}
        }

    }
    res.chars().rev().collect::<String>()
} 