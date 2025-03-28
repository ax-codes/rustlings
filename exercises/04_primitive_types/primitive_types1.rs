// Booleans (`bool`)

fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // TODO: 在下面的 `if` 语句之前定义一个名称为 `is_evening` 的布尔变量。
    // 变量的值应该是 `is_morning` 的否定值（相反值）。
    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
