# js-parser-rs

This GitHub project serves as an experiment to utilize the capabilities of ChatGPT to create a JavaScript parser. The goal is to explore the potential of using the model to improve developer productivity in real-world projects. The results of this experiment will be shared and evaluated in the repository.

## Lexer

- ChatGPT generated the code that doesn't advance the scanner to get the next token.
- The initial code doesn't handle the peek_n token correctly, only given Nth token instead of the current cursor.
- The test cases are actually pretty good and usable.
- Generated tests have some type mismatch errors that must be manually fixed, but they are very useful for adding tests!

## Token

- The generated token enum has incorrect keywords.

## Parser

- I have to ask the chatgpt to generate the AstNode first and then the parser, but the generated parser is not using correct token type.
- Second, I ask chatgpt to create js parser based on existing token and astNode, it rejected the prompt, I have to trick it by saying it's a tutorial for students and let it generate the parser.
- Once the conversation goes to long, I noticed that chatgpt is losing the context and unable to generate the correct code. I have to restart the conversation and ask it to generate the parser again.
- Always a good idea to ask chatgpt to generate tests for the parser.
