# Logo evaluator to .svg file
Currently minimal viable product version.

## Parser
Parser built with nom library.
First, instructions are parsed (/src/parser) to the abstract syntax tree with structure:
Program [
    Block[
        Command[Unit, Unit, ...],
        Command[...],
        Command[...],
        ...
        ]
    Block[...],
    Block[...],
    ...
]
Program = vector of blocks
Block = REPEAT loop | IF block | function definition | function call (/src/parser/block)
Command = LEFT | RIGHT | etc. (/src/parser/command)
Unit (/src/parser/unit) =  Value 
        | Variable 
        | Expression that will need further evaluation e.g. ":steps / 3" 
        | Random (a little tricky, but makes sense to me - it is a numeric value, but will be chosen in eval)

## Evaluator
Then, I evaluate them (/src/evaluator) from the abstract tree:
- Turtle (/src/evaluator/turtle) is object that memorize current turtle properties (position, angle, pen) and history of drawn lines; 
history of lines will then be used to create image.
- In the evaluation process, we hold Env (/src/evaluator/environment) with variables' values and functions' definitions (wrapped HashMaps)
Evaluation (/src/evaluator/eval) is done on structure levels:
- unit evaluation: calculating numeric value of unit
- command evaluation: updating turtle model
- block evaluation: more advanced evaluation logic
- program evaluation: program is just a list of blocks, so nothing special here.
Important thing is the fact, that on every level of evaluation we pass Option<> from lower level, because of Stop() logic (if command
is stop, I pass None instead of Some to upper levels and stop execution)

## Draw
Drawing history of lines to .svg file using svg library; images are saved in /images directory.

## Tests
/tests
Note to myself: next time create tests in /src subdirectories, to avoid a lot of imports and public objects :<.
However, everything is tested at least for mvp.


# Todos
- evaluation of more advanced expressions e.g. repcount * repcount / 30 (no idea how to parse it, for now i have workaround with changing to decimal)
- pick [ red orange yellow green blue violet ] (parsing list of strings will destroy my unit parser)
- clean it, especially converting back and forth between u32 and f64 (just joking, i won't do it)
- recursive functions: restoring history of environment is the problem, cuz at the end of function i just delete them;
I can probably workaround with history of every variable in stack and pop of it but i rather have 2 points less, than spend 2 hours more on this XD


