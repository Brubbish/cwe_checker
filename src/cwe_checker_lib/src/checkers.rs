//! The implemented CWE checks.
//! See their module descriptions for detailed information about each check.
//!
//! Currently the **Memory** check is not contained in this module
//! but directly incorporated into the [`pointer_inference`](crate::analysis::pointer_inference) module.
//! See there for detailed information about this check.

pub mod cwe_119;//125：越界读；787：越界写
pub mod cwe_134;//格式化字符串
pub mod cwe_190;//整数溢出
pub mod cwe_215;//debug导致信息泄露
pub mod cwe_243;//Creation of chroot Jail Without Changing Working Directory
pub mod cwe_332;//PRNG信息熵不充分
pub mod cwe_367;//TOCTOU条件竞争
pub mod cwe_416;//416：UAF；415：Double Free
pub mod cwe_426;//不受信任的搜索路径
pub mod cwe_467;//对指针进行sizeof
pub mod cwe_476;//解引用空指针
pub mod cwe_560;//Use of umask() with chmod-style Argument
pub mod cwe_676;//危险函数
pub mod cwe_78;//命令注入
pub mod cwe_782;//Exposed IOCTL with Insufficient Access Control
pub mod cwe_789;//分配过大内存
