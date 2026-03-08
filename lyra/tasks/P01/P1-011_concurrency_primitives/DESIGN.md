# Design — P1-011 Concurrency Primitives

The executable Stage 0 surface stays within existing parser capabilities by using builtin call forms rather than introducing detached syntax. The checker is responsible for schedule summaries, channel-operation traces, and the no-linear-capture rule.
