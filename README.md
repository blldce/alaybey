# alaybey
Alaybey is simple VM which operate basic instructions such as add,sub,div,mul and mod. its allow define variables and usage in operations.

Usage little endian byte ordiring first 3 bits reserved for opcode instructions.

example:

{

3 5 + 7 - 8 + 2 /;

10 5 - 4 * 8 + 4 /;

}


Save as give_name_what_you_want.alaybey

note : source file must end with ".alaybey" file

command : 
alaybey build give_name_what_you_want.alaybey

program will generated such as 'give_name_what_you_want.alaybeyvm'

then run command : 

alaybey run give_name_what_you_want.alaybeyvm

result will be printed.


Or define variables:

{

$ val1 : 3 5 + 7 - 8 + 2 /;

$ my_other_val : 10 5 - val1 * 8 + 4 /;

$ result : my_other_val val1 - val1 * 8 + 4 /;

}
