# STA tools for Openic

## block-based STA

[tatum sta](https://github.com/verilog-to-routing/tatum/tree/master/libtatum/tatum) rewritten in Rust.

### Migration Task Sheet

libtatum:

- /util/OsFormatGuard: Done (not effectively used in the project).
- /util/tatum_assert*: Ignore.
- /util/tatum_range: Done, merged into struct LinearMap.
- /util/tatum_strong: Done, using rust newtype, in *base.rs*.
- /util/tatum_math: Done.
- /util/linear_map: Done, in *base.rs*.
- /base/loop_detect*: Done, in *base.rs*.

TimingGraph:

- TimingGraphFwd: Done, all types are in *base.rs*, which saves a lot of code.
- TimingGraph: **Done**, can be proved.

Refactor code between /util and /base:

- all helpful functions move into /util (util.rs).
- all basic data types and structs move into /base (base.rs).

### Common Notings

- Ignore all `std::ostream& operator <<`, rust `println!` do not recognize by type.
- Ignore all *TATUM_ASSERT_*, use rust assert.

