This is a client written in rust for https://thephilosophyforum.com/

The parsing of the HTML is poorly written but it gets the job done (some stuff
that I delegate to serde could be better done manually).

Controls:
- `Up, Down` moving on the overview
- `n, p` moving on the comment list
- `PgUp, PgDn` scrolling on the viewer
- `q` quit
- `0..9` vim-like multiplier
- `Esc` nullifies the action multiplier

TODO: Properly parse comment text. The forum allows for BBCodes so some endup
translating into annoying HTML tags sprinkled across the text.
