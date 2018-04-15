example traces generated with trace2html from [catapult project](https://github.com/catapult-project/catapult)

I only get the generated html to work in chrome
# Example traces rustc incremental builds on a 32 threads cpu
* [build of ripgrep](ripgrep.html)
* [release build of ripgrep](ripgrep_release.html)
# Example traces rustdoc on a 32 threads cpu
this is the code that is tested in rustdoc and the example is repeated 181 times
```rust
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, add_one(5));
    /// # fn add_one(x: i32) -> i32 {
    /// #     x + 1
    /// # }
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
```
* [default (RUST_TEST_THREADS=32)](rustdoc32.html)
* [RUST_TEST_THREADS=6](rustdoc6.html)