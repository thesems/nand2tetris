// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.

    // sum = 0
    @R0
    D=M
    @sum
    M=0

(LOOP)
    // R2 = R2 + R0
    @R0
    D=M
    @sum
    M=M+D

    // R1 = R1 - 1
    @R1
    M=M-1

    // if R1 != 0; go to LOOP
    @R1
    D=M
    @LOOP
    D;JNE

    // R2 = accum
    @sum
    D=M
    @R2
    M=D

(END)
    // infinite loop
    @END
    0;JMP
