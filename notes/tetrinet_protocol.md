# The TetriNET 1.13 Protocol Specification (Client-Side)

I did not find any formal specification of the TetriNET protocol online. So here is what I understand, from the behavior of the classical TETRINET 1.13 Win32 application : 

## I - Connection and format of messages

The connection between a TetriNET client and a TetriNET server is a TCP connection. The server should accept connections on the port **31457**.

A TetriNET message ends with the `ÿ` separator (ASCII 255). So the TCP data stream should be processed with respect to this character. 


## II - Messages that the client can RECEIVE 

playerjoin, pline, field, winlist, playernum, gmsg

## III - Messages that the client can SEND

team, f, startgame, gmsg

## IV - Establishment of the connection 

After the TCP CONNECT, the client should send an **encoded**, ÿ-terminated string. The unencoded string has the following format : 

```
tetrisstart <player_name> <tnet_version>
```

`tnet_version` is **1.13** for the scope of this project. 

The whole string must be encoded as follows: 

### TEMP - How to decode it 

As an example, consider 2D97C40EB529A42F96C10CB7E211429030A32E45B8EE187197FC
1. Take the string characters 2 by 2, those are bytes represented in hexa. Convert each character pair to the corresponding integer value in [| 1, 255 |]
2. Get an array whose values are the ASCII codes of every character in the `tetrisstart` command, namely [116, 101, 116, 114, 105, 115, 115, 116, 97, 114, 116].

```
 T    E    T    R    I    S    S    T   A    R    T 
 v    v    v    v    v    v    v    v   v    v    v 
116, 101, 116, 114, 105, 115, 115, 116, 97, 114, 116
```



