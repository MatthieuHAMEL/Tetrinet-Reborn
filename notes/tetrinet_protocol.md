# The TetriNET Protocol Specification

I did not find any formal specification of the TetriNET protocol online. So here is what I understand, from the behavior of the classical TETRINET 1.13 Win32 application : 

## I - Connection and format of messages

The connection between a TetriNET client and a TetriNET server is a TCP connection. The server should accept connections on the port **31457**.

A TetriNET message ends with the `ÿ` separator (ASCII 255). So the TCP data stream should be processed with respect to this character. 


## II - Messages that the client can receive 

playerjoin, pline, field 

## II - Messages that the client can send 

