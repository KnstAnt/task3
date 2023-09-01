# task3
Test problem solution.
Console application iterates over integers starting from 1,
calculates the sha256 hash for each of the numbers, and
displays the hash and the original number to the console
if the hash digest (character representation of the hash)
ends in N-characters of zero. The F parameter determines
how many hash values the command should find.

usage example: 
  hash_finder -N 5 -F 3
  
Options:
  -N, --nulls       quantity of nulls at the end of hash
  -F, --hashes      quantity of hashes to find
  --help            display usage information


