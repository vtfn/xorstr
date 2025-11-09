# xorstr
A macro that performs compile time encryption and runtime decryption of strings using XOR.

# How to use
```rust
use xorstr::xor;

...

// Apply runtime decryption to the provided string
// The contents of the string is *static*
let s: &'static str = xor!("good morning america");
```
```asm
sub     rsp, 18h
mov     eax, dword ptr cs:xmmword_140003010
mov     [rsp+18h+var_8], eax
movaps  xmm0, cs:xmmword_140003000
movaps  [rsp+18h+var_18], xmm0
lea     rax, xmmword_140003000
mov     rcx, rsp
movaps  xmm0, cs:xmmword_140003000
xorps   xmm0, cs:xmmword_140002000
movaps  cs:xmmword_140003000, xmm0
movaps  xmm0, cs:xmmword_140003010
xorps   xmm0, cs:xmmword_140002010
movss   dword ptr cs:xmmword_140003010, xmm0
mov     qword ptr [rsp+18h+var_18], rax
mov     qword ptr [rsp+18h+var_18+8], 14h
mov     rax, rsp
add     rsp, 18h
```
