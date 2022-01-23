# wordle-rs

Very basic Wordle solver in Rust. Takes clues from command line prompts and uses them to filter a word list.

A clue looks like this: `h ?e !l !l o` 

* `?` means the letter appears somewhere, ie a yellow clue
* `!` means the letter appears nowhere, ie a grey clue
* no modifier means the letter appears exactly there, ie a green clue
