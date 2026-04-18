#![allow(non_snake_case)]

/// 两数相加
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 两数相减
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// 两数相乘
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// 两数相除（带错误处理）
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(0, 5), -5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 3), -6);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(10, 3), Some(3));
        assert_eq!(divide(10, 0), None);
    }
}
