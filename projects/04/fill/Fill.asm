// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(LOOP)

    @color
    M=0

    @SCREEN
    D=A

    @idx
    M=D

    @KBD
    D=M

    @DRAW
    D;JEQ

(BLACK)
    // color = -1 
    @color
    M=-1

(DRAW)

    @color
    D=M

    // A = idx
    @idx
    A=M

    // RAM[A] = color
    M=D

    // idx = idx + 1
    @idx
    M=M+1

    // if idx - 24576 < 0; goto DRAW
    @24576
    D=A
    @sub
    M=D
    @idx
    D=M
    @sub
    M=M-D

    @sub
    D=M

    @DRAW
    D;JGT

    // else
    @LOOP
    0;JMP
    