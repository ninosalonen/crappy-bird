# Crappy bird

A crappy verison of flappy bird with simple 2D graphics. This was a small course project where the idea was to get familiar with some different language or paradigm. I tried to experiment with closures, iterators, and pattern matching wherever possible, though pattern matching may didn't really fit here.

## Development

Use `cargo run` to run the development version.

## Production build

Use `cargo build` to build the production version. The production build needs the static assets folder in the same directory for sounds to work: `cp -r ./assets ./target/debug`.
