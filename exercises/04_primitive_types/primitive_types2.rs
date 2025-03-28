// Characters (`char`)

fn main() {
    // 注意其中的 _single_ 引号，它们不同于双引号。
    // 你经常看到的双引号。
    let my_first_initial: char = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: 与之前的例子类似，在下面声明一个名为 `your_character` 的变量。
    // 的变量。
    // 尝试一个字母，尝试一个数字（单引号），尝试一个特殊字符，尝试一个字符
    // 试试与你的语言不同的字符，试试表情符号 😉
    // let your_character = ''；

    let your_character = '😉';
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
