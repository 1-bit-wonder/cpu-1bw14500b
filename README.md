# CPU-1BW14500B (WIP)

1-bit microprocessor simulator based on the MC14500B

## MC14500B

A microprocessor for decision-oriented tasks. It is well-suited to the implementation of ladder logic, and thus could be used to replace relay systems and programmable logic controllers, also intended for serial data manipulation.

![MC14500B](https://github.com/1-bit-wonder/cpu-1bw14500b/blob/main/mc14500b-cpu.png?raw=true)
## Instruction set

0000 NOPO -> No change in registers<br/>
0001 LD   -> Load Result Reg. Data<br/>
0010 LDC  -> Load Complement Data<br/>
0011 AND  -> Logical AND<br/>
0100 ANDC -> Logical AND Compl<br/>
0101 OR   -> Logical OR<br/>
0110 ORC  -> Logical OR Compl<br/>
0111 XNOR -> Exclusive NOR<br/>
1000 STO  -> Store<br/>
1001 STOC -> Store Compl<br/>
1010 IEN  -> Input Enable<br/>
1011 OEN  -> Output Enable<br/>
1100 JMP  -> Jump<br/>
1101 RTN  -> Return<br/>
1110 SKZ  -> Skip next instruction if result reg=0<br/>
1111 NOPF -> No change in Registers<br/>

## Resources

https://en.wikichip.org/w/images/3/3e/Motorola_MC14500B_Industial_Control_Unit_Handbook.pdf<br/>
https://en.wikichip.org/w/images/5/5b/MC14500B_datasheet.pdf<br/>
https://en.wikipedia.org/wiki/Motorola_MC14500B<br/>
https://hackaday.com/2020/02/01/what-everyone-else-did-with-eight-bits-the-germans-did-with-only-one/<br/>
http://www.righto.com/2021/02/a-one-bit-processor-explained-reverse.html<br/>
