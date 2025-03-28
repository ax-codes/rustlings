fn bigger(a: i32, b: i32) -> i32 {
    // TODO: 完成此函数以返回较大的数字！
    // 如果两个数字相等，可以返回其中任何一个。
    // 不要使用
    // - 另一个函数调用
    // - 附加变量

    if a > b {
        return a;
    } else {
        return b;
    }
}

fn main() {
    // You can optionally experiment here.
    bigger(3,4);
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
