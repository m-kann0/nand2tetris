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

(INFINITE_LOOP)
@SCREEN
D=A
@pointer
M=D

// height = 256
@256
D=A
@height
M=D
// h = 0
@0
D=A
@h
M=D
(LOOP1)
// if h >= height then break
@h
D=M
@height
D=M-D
@INFINITE_LOOP
D;JLE

// width = 32
@32
D=A
@width
M=D
// w = 0
@0
D=A
@w
M=D
(LOOP2)
// if w >= width then break
@w
D=M
@width
D=M-D
@BREAK_LOOP2
D;JLE

// fill
@pointer
D=M
A=D
M=0
@KBD
D=M
@NO_INPUT
D;JEQ
@pointer
D=M
A=D
M=-1
(NO_INPUT)

// pointer++
@pointer
M=M+1

// w++
@w
M=M+1
@LOOP2
0;JEQ
(BREAK_LOOP2)

// h++
@h
M=M+1
@LOOP1
0;JEQ

// loop infinitely
@INFINITE_LOOP
0;JMP
