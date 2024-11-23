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
Unit = Value | Variable | Expression that will need further evaluation e.g. ":steps / 3" (/src/parser/unit)

## Evaluator
Then, I evaluate them (/src/evaluator) from the abstract tree:
- Turtle (/src/evaluator/turtle) is object that memorize current turtle properties (position, angle, pen) and history of drawn lines; 
history of lines will then be used to create image.
- In the evaluation process, we hold Env (/src/evaluator/environment) with variables' values and functions' definitions (wrapped HashMaps)
Evaluation (/src/evaluator/eval) is done on structure levels:
- unit evaluation: calculating numeric value of unit
- command evaluation: updating turtle model
- block evaluation: more advanced evaluation logic
- program evaluation: program is just a list of blocks, so nothing special here

## Draw
Drawing history of lines to .svg file using svg library; images are saved in /images directory.

## Tests
Note to myself: next time create tests in /src subdirectories, to avoid a lot of imports and public objects :<.
However, everything is tested at list for mvp.
/tests
