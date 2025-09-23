// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass! 
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 让测试通过的版本 (使用两个相等的值)
        assert_eq!(2 + 2, 4);
        
        // 如果你想让测试失败，取消下面这行的注释
        // assert_eq!(2 + 2, 5);
    }
}