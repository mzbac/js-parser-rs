# js-parser-rs

This GitHub project serves as an experiment to utilize the capabilities of ChatGPT to create a JavaScript parser. The goal is to explore the potential of using the model to improve developer productivity in real-world projects. The results of this experiment will be shared and evaluated in the repository.

## Lexer
- ChatGPT generated the code that doesn't advance the scanner to get the next token.
- The initial code doesn't handle the peek_n token correctly, only given Nth token instead of the current cursor.
- The test cases are actually pretty good and usable.
