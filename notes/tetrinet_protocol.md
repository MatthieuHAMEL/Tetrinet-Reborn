# The TetriNET 1.13 Protocol Specification (Client-Side)

I did not find any formal specification of the TetriNET protocol online. So here is what I understand, from the behavior of the classical TETRINET 1.13 Win32 application : 

## I - Connection and format of messages

The connection between a TetriNET client and a TetriNET server is a TCP connection. The server should accept connections on the port **31457**.

A TetriNET message ends with the `ÿ` separator (ASCII 255). So the TCP data stream should be processed with respect to this character. 


## II - Messages that the client can RECEIVE 

Every message is ÿ-terminated. 

This is the exhaustive (TODO!!) list of possible message types:

winlist, playernum, playerjoin, team, f, playerlost, ingame, pause, pline, newgame, gmsg

### II.1. winlist : some list of players with their scores 

This can be sent by the server at any time.

The general format is : " winlist entry1 entry2 entry3 entry4 etc... "
where an entry consists of : " <t|p><Teamname|Playername>;<points> "
<t|p> : t=team-entry and p=player-entry .


### II.2. playernum : the number assigned to the client

This can be sent by the server at any time.
e.g. `playernum 1`

### II.3. playerjoin

This can be sent by the server at any time.

It gives the ID and the nickname of a new player. 
e.g. `playerjoin 4 babar`

### II.4. team

This can be sent by the server at any time.
When the client sends the same exact `team` message to the server to declare its team, the server propagates it to other clients. So if there are 4 players (including the Client), the Client will receive 3 team messages at least (TODO verify this)

### II.5. f (fields)

This can be sent by the server at any time.

```
f X <fieldstring>
```

Where X is the player number, and `fieldstring` a string of 264 (12 cols, 22 rows) chars representing the field.
The chars can be : 0,1,2,3,4,5,a,c,n,r,s,b,q,o : 

Char	What
0	blank/empty
1	blue block
2	yellow block
3	green block
4	purple block
5	red block
a	special a
c	special c
n	special n
r	special r
s	special s
b	special b
g	special g
q	special q
o	special o

## II.6. playerlost

When a player dies, including our client, the server broadcasts :
```
playerlost X
```

## II.7. ingame

If the game is currently active the server will send an `ingame` message.

## II.8. pause

The Server will tell the Client whether the game is paused or not :

e.g. `pause 0` / `pause 1`

## II.9. pline (0)

The server sends a message to a Client with pline 0, e.g. 

```
pline 0 Game has been stopped by Babar!
```

## II.10. newgame 

The server tells every client that a new game starts.

e.g. 
```
newgame 0 1 2 1 1 1 18 3333333333333355555555555555222222222222222444444444444446666666666666677777777777777111111111111111 1111111111111111111111111111111112222222222222222222234444444444444566666666666666678888889999999999 0 1
```

This number sequence represents the game settings : format is :

```
newgame stack startinglevel linesperlevel levelincrease linesperspecial specialadded specialcapacity blockstring specialstring averagelevels classicrules
```

The blockstring will consists of 100 chars (100%), each char represents a Tetromino, this allows to specify the occurency rate of every Tetromino in the game :

1	line/stick
2	square
3	left L
4	right L
5	left Z
6	right Z
7	halfcross

The same principle applies for the specialstring :

Char   What
1	addline (a)
2	clearline (c)
3	nukefield (n)
4	randomclear (r)
5	switchfield (s)
6	clearspecials (b)
7	gravity (g)
8	quakefield (q)
9	blockbomb (o)

## II.11. gmsg

Game Messages format is :
```
gmsg <nick> <message>
```

According to [TSRV.COM], the Server doesn't check the nick, it just displays it, so the following is possible :

```
gmsg <message>
```


## III - Messages that the client can SEND

In the following messages, <N> is the player number.

team, f, startgame, gmsg

### III.1. team

This can be sent by the client at any time. TODO : during games ?????  
The client sends its team to the server.

```
team <N> <teamname>
```

### III.2. pline (1)

This is for chatting with other players : e.g. :

```
pline <N> Good game everyone!
```

### III.3. plineact 

Like on IRC this is an action message (which is just displayed differently in the chat) : 
`plineact <N> myText`

TNET on Win32 : type "/me myText" in the chat bar. TODO : check which string is really sent to the server (wireshark)

### III.4. startgame 

The Client will request a game start / stop by sending :
```
startgame 1|0 <N>
```

where 1 = Start game ; 0 = Stop game and X = playernumber

### III.5. gmsg

C.f. §II.11. 

### III.6. f 

TODO field update rules

## IV - Establishment of the connection 

After the TCP CONNECT, the client should send an **encoded**, ÿ-terminated string. The **unencoded** string has the following format : 

```
<command> <player_name> <tnet_version>
```

- `<command>` is always `tetrisstart` in the scope of this project.
- `<tnet_version>` is always **1.13** for the scope of this project. 

The whole string must be encoded as follows, integrating the server IP: (See Annex 1 for the decoding flow at Server side)

[TSRV.COM] gives the procedure in Perl : 

```perl
sub tnet_encrypt {
  my ($nickname, @ip) = @_;

  my @s = "tetrisstart $nickname 1.13";
  
  my @h = split(//, $ip[0]*54 + $ip[1]*41 + $ip[2]*29 + $ip[3]*17);
  my $dec = int rand 256; # salt, a random number between 0 and 255
  my $encrypted = sprintf("%02X", $dec & 0xFF);
  
  #sprintf: Returns a string formatted by the usual printf conventions of the C library function sprintf. 
  #         %X   an unsigned integer, in hexadecimal
  #         So actually what this does is converting $dec to hex with two characters

  for (my $i=0; $i<@s; $i++) {
    $dec = (($dec + ord($s[$i])) % 255) ^ ord($h[$i % @h]);
    
    #ord(EXPR): Returns the numeric value (of the first character) of EXPR (= reverse of chr(EXPR) )
    # ^ : Returns its operands XORed together bit by bit.
    
    $encrypted .= sprintf("%02X", $dec & 0xFF);
  }

  return $encrypted;
}
```



## Annexes

### Annex 1: how should the server decode the encoded tetrisstart command ?

As an example, consider the message `2D97C40EB529A42F96C10CB7E211429030A32E45B8EE187197FC`

It will be decoded as : `tetrisstart DieterDH 1.13` by the following procedure : 

#### 1. Take the string characters 2 by 2, those are bytes represented in hexa. Convert each character pair to the corresponding integer value in [| 1, 255 |].
   The resulting array will be reffered to as `int_code`.

In Perl, from [TNET.SRV] doc : 

```perl
$cmd = 'tetrisstart';

my @int_msg;
for (my $i = 0; $i < length($msg); $i += 2) {
  push(@dec, hex substr($msg, $i, 2));
}
```

#### 2. Get a ```int_tstart``` array whose values are the ASCII codes of every character in the `tetrisstart` command, namely [116, 101, 116, 114, 105, 115, 115, 116, 97, 114, 116].

```perl
my @int_tstart = map {ord $_} split(//, $cmd);
```

```
 T    E    T    R    I    S    S    T   A    R    T 
 v    v    v    v    v    v    v    v   v    v    v 
116, 101, 116, 114, 105, 115, 115, 116, 97, 114, 116
```

#### 3. Create a hash array
```perl
my @h;
for (my $i = 0; $i < @int_tstart; $i++) { # i.e. from 0 to 11 (excl) in our case
  push(@h, (($int_tstart[$i] + $int_msg[$i]) % 255) ^ $int_msg[$i + 1]);
}
```

#### 4. Try to determine the length of a repeated pattern in h

```perl
my $h_length = 5;
for (my $i = 5; $i == $h_length and $i>0; $i--) {
  for (my $j = 0; $j < @data - $h_length; $j++) {
    $h_length-- if $h[$j] != $h[$j + $h_length];
  }
}
die if $h_length == 0;
```

h has something to do with the server IP. (TODO give more context...)

#### 5. Decode the user command

```perl
my $decrypted = '';
for (my $i = 1; $i < @dec; $i++) {
  $decrypted .= chr((($dec[$i] ^ $h[($i - 1) % $h_length]) + 255 - $dec[$i - 1]) % 255);
}
```

Source : [TNET.SRV] doc. 

