// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).fold(1, |acc,x| acc*x)
    //fold 是一个迭代器方法，用于将一组值聚合为一个单一的值。
    //1 是 fold 的初始值，这里表示阶乘计算的初始值（即 0! 和 1! 都是 1）
    //|acc, x| acc * x 是一个闭包，接受两个参数：
      //acc：表示当前累积的结果。
     //x：表示当前迭代的值。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
