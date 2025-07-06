use 5.40.0;

# Source : TSRV.COM specification

# For example
my $msg = "A221BE04439A369E26B11CA7F2558E3493CA16B6E32369ADD5";


sub tnet_decrypt {
  my ($msg, $cmd) = @_;
  $cmd = 'tetrisstart' unless defined $cmd;
  #terisstart is used for TetriNET, terisfaster is used for TetriFAST 
  
# for EXAMPLE $msg = 2D97C40EB529A42F96C10CB7E211429030A32E45B8EE187197FC
#             $cmd = tetrisstart

# Convert the string ($msg) to an array (@dec) of integers

  my @dec;
  for (my $i=0; $i<length($msg); $i+=2) {
    push(@dec, hex substr($msg, $i, 2));
    #push(ARRAY,LIST) : pushes the values of LIST onto the end of ARRAY
    #hex(EXPR) : Interprets EXPR as a hex string and returns the corresponding value.
  }

# Create an array (@data) of the first token ($cmd or tetrisstart for TetriNET/ tetrifaster for TetriFast ). 

  my @data = map {ord $_} split(//, $cmd);

 # map(EXPR) : translates a list of numbers to the corresponding characters
 # ord(EXPR) : Returns the numeric ascii value of the first character of EXPR. If EXPR is omitted, uses $_.


  my @h;
  for (my $i=0; $i<@data; $i++) {
    push(@h, (($data[$i] + $dec[$i]) % 255) ^ $dec[$i + 1]);
    
  }
  my $h_length = 5;
  for (my $i=5; $i==$h_length and $i>0; $i--) {
    for (my $j=0; $j<@data-$h_length; $j++) {
      $h_length-- if $h[$j] != $h[$j + $h_length];
    }
  }

  return undef if $h_length == 0;
    
  #Now decode the string ($msg) with @dec and @h

  my $decrypted = '';
  for (my $i=1; $i<@dec; $i++) {
    $decrypted .= chr((($dec[$i] ^ $h[($i - 1) % $h_length]) + 255 - $dec[$i - 1]) % 255);
  }
  
  # chr(NUMBER) : Returns the character represented by that NUMBER in the character set.  

  my $zero = chr(0);
  my $replace = chr(255);
  $decrypted =~ s/$zero/$replace/g;

  # s/$zero/$replace/g : this will replace all the $zero-chars by $replace-chars in the $decrypted-string
  
  say $decrypted;

  #Return the decrypted string ($decrypted)
  return $decrypted;
}

tnet_decrypt($msg);

