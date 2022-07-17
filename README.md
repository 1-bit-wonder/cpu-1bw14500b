# CPU-1BW14500B (WIP)

1-bit microprocessor simulator based on the MC14500B

## MC14500B

A microprocessor for decision-oriented tasks. It is well-suited to the implementation of ladder logic, and thus could be used to replace relay systems and programmable logic controllers, also intended for serial data manipulation.

![MC14500B](https://github.com/1-bit-wonder/cpu-1b14500b/blob/main/mc14500b-cpu.png?raw=true)
## Instruction set

0000 NOPO -> No change in registers
0001 LD   -> Load Result Reg. Data
0010 LDC  -> Load Complement Data
0011 AND  -> Logical AND
0100 ANDC -> Logical AND Compl
0101 OR   -> Logical OR
0110 ORC  -> Logical OR Compl
0111 XNOR -> Exclusive NOR
1000 STO  -> Store
1001 STOC -> Store Compl
1010 IEN  -> Input Enable
1011 OEN  -> Output Enable
1100 JMP  -> Jump
1101 RTN  -> Return
1110 SKZ  -> Skip next instruction if result reg=0
1111 NOPF -> No change in Registers

## Resources

https://en.wikichip.org/w/images/3/3e/Motorola_MC14500B_Industial_Control_Unit_Handbook.pdf
https://en.wikichip.org/w/images/5/5b/MC14500B_datasheet.pdf
https://en.wikipedia.org/wiki/Motorola_MC14500B
https://hackaday.com/2020/02/01/what-everyone-else-did-with-eight-bits-the-germans-did-with-only-one/
http://www.righto.com/2021/02/a-one-bit-processor-explained-reverse.html
