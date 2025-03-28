fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // rust的变量类型一旦申明,就不能再改变,所以这里的变量名不能改变
    let number = 3; // 这里不是变量遮蔽，变量遮蔽需要类型完全相同，这里的类型不同，所以不是变量遮蔽，这里创建新的变量，上一个变量内存释放
    println!("Number plus two is: {}", number + 2);
}
