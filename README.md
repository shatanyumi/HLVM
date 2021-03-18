# HLVM
   学习rust的一个开源项目，用rust写一个基于寄存器级别的Language VM
### Summary
​	原开源作者：https://blog.subnetzero.io/post/building-language-vm-part-00/

​	要求对计算机硬件理解比较深刻，寄存器、汇编器、CPU Pipeline等基本理论，跟着大佬学习顺便学习 rust。

​	We’re going to write an application that pretends to be a CPU, and executes programs we write for it. Which, of course, means we’ll have to invent a language too. But we’ll get to all that later. You should now have enough basic knowledge to go on to the next section.

### Instruction 设计

| Opcode(8 bits) |                     |                     |                    |
| :------------: | :-----------------: | :-----------------: | :----------------: |
| Opcode(8 bits) | Operand 1 (24 bits) |                     |                    |
| Opcode(8 bits) | Operand 1 (8 bits)  | Operand 2 (16 bits) |                    |
| Opcode(8 bits) |  Operand 1 (8bits)  | Operand 2 (8 bits)  | Operand 3 (8 bits) |



### VM设计（updating）

```rust
pub struct VM {
    /// 32bit registers
    registers: [i32; 32],
    /// program counter
    pc: usize,
    /// vector to store a byte
    program: Vec<u8>,
    remainder: u32,
}
```

