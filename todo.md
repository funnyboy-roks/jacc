# To-Do

- [ ] Add clap for some command-line shenanigans
- [ ] Read from file, STDIN, or command-line argument
    - [ ] Each line should be interpreted as its own statement, unless
      the line ends with a `\`, in which case, it would just concatenate
      with the line below.
- [ ] The ability to customise the format of output:
    - `-x` for hex
    - `-b` for bin
- [ ] Add support for decimal numbers in the input (will require rework
  of the lexer's lexing of numbers)
- [ ] Highlight the token upon error
    - This will require us to pass `TokenSpan` when we parse the tokens
      into the `AstStatement`s, but I don't _think_ that should be too
      bad
- [ ] Clean up the error messages (partly combined with above)
