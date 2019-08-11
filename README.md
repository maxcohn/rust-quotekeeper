# rust-quotekeeper

I'm awful with names...

This is a rewrite of Flask based web app I wrote a little while back. I figured
this would be a good way to help me learn Rust. This is essentially a direct port
to Rust with the exception of the added options to filter input.

I'm still new to Rust, but this was a great way to interact with some new crates,
become more comfortable with the borrow checker, and learn more of the standard
library.

The main source of the bugs I ran into while writing this was `rusqlite`. More
documentation and examples would benefit the crate as a whole. I guess I should
start contributing!

## What I learned from this

1. Rusqlite feels overly verbose. It's a solid library, but it feels clunky and
overly verbose to do something as simple as query with a parameter.

2. Rocket is fantastic. Rocket is probably the crate I'm most excited about, because
it offers the simplisitc syntax of Flask (which is another web framework I love), while
giving access to the power of Rust.

