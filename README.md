# Rust-fuzz-test

## Existing unsafe cases

- split_at_mut case for string：当输入字符串包含 rust 子串且长度小于 5 （其实只有 rust 这一个 case）时，会因为分割 index > length 而 panic
- double free：当输入字符串有 rust 后缀，不包含整数，前四个字符字典序 > "test"，前三个字符 ascii 值的和 >= 350 时，试图返回该字符串的 `Vec<u8>` 时会因 double free 而 panic
- 悬垂引用：在满足正则的输入中，如果 `input:` 后面没有任何整数，只有长度大等于2的 `-` 字符时，会因为对空指针解引用而 panic。

## Possible Tools

- MIRAI
- SYMCC
- AFLGO
- Rust-fuzz
