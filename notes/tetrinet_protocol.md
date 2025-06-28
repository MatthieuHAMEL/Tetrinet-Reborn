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

