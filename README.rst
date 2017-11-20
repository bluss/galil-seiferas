galil-seiferas
==============

General string search in constant space, linear time, for nonorderable alphabets.
Also known as exact string matching.

Please read the `API documentation on docs.rs`__

__ https://docs.rs/galil-seiferas/

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/galil-seiferas.svg?branch=master
.. _build_status: https://travis-ci.org/bluss/galil-seiferas

.. |crates| image:: http://meritbadge.herokuapp.com/galil-seiferas
.. _crates: https://crates.io/crates/galil-seiferas

Recent Changes
--------------

- 0.1.3

  - Update the algorithm to keep the position in the pattern during search
    passes. (Makes fewer comparisons with periodict patterns.)
  - Expression tweaks that improve benchmarks without affecting the algorithm

- 0.1.2

  - Cleanup code, better code comments, refactor, more tests and no memcmp use
  - The library is now always ``no_std``.

- 0.1.1

  - Fix bug in decompose (`#1`_)

- 0.1.0

  - Initial release

.. _#1: https://github.com/bluss/galil-seiferas/pull/1


Benchmarks
----------

Here are some comparisons as a plain byte string searcher. This isn't the use
case for this algorithm, its use case is for nonorderable alphabets. But we
use this to have wide comparisons. Keep in mind that the byte string search
is characterized by its very cheap comparison operation.

It's not fast — two way is a whole lot better, if you can use it. But we have
one baseline, and that's the "naive" brute force search which is quadratic in
the size of the input, in the worst case.

The repo https://github.com/bluss/scratchspace collects some string matching benchmarks,
and a lot of them are “pathologies” or cases that were known to be bad for some
algorithm. The result of one run are below. (Only subset available)

This run is just a quick one so it may be quite noisy. Unavoidably there are single
tests whose results are off.

The G-S algorithm is ``gs_find``; the KMP algorithm (in a simple implementation) is
``kmp_find``; the substring searcher in Rust is just ``find``::

  test naive::bmh_find                        ... bench:         826 ns/iter (+/- 7)    302 MB/s
  test naive::brute_force                     ... bench:       1,908 ns/iter (+/- 73)   131 MB/s
  test naive::find                            ... bench:         516 ns/iter (+/- 100)  484 MB/s
  test naive::gs_find                         ... bench:         624 ns/iter (+/- 20)   400 MB/s // NOTE
  test naive::kmp_find                        ... bench:         714 ns/iter (+/- 24)   350 MB/s
  test naive::memmem                          ... bench:         187 ns/iter (+/- 9)   1336 MB/s
  test naive_rev::bmh_find                    ... bench:          36 ns/iter (+/- 1)   6944 MB/s
  test naive_rev::brute_force                 ... bench:         279 ns/iter (+/- 3)    896 MB/s
  test naive_rev::find                        ... bench:         444 ns/iter (+/- 43)   563 MB/s
  test naive_rev::gs_find                     ... bench:         364 ns/iter (+/- 6)    686 MB/s // NOTE
  test naive_rev::kmp_find                    ... bench:         514 ns/iter (+/- 17)   486 MB/s
  test naive_rev::memmem                      ... bench:          11 ns/iter (+/- 0)  22727 MB/s
  test short_word1_long::bmh_find             ... bench:       1,490 ns/iter (+/- 77)  1712 MB/s
  test short_word1_long::brute_force          ... bench:       4,064 ns/iter (+/- 115)  627 MB/s
  test short_word1_long::find                 ... bench:         652 ns/iter (+/- 47)  3912 MB/s
  test short_word1_long::gs_find              ... bench:       4,945 ns/iter (+/- 167)  515 MB/s // NOTE
  test short_word1_long::kmp_find             ... bench:       6,515 ns/iter (+/- 272)  391 MB/s
  test short_word1_long::memmem               ... bench:       1,594 ns/iter (+/- 25)  1600 MB/s
  test short_word2_long::bmh_find             ... bench:       2,620 ns/iter (+/- 159)  973 MB/s
  test short_word2_long::brute_force          ... bench:       3,724 ns/iter (+/- 407)  685 MB/s
  test short_word2_long::find                 ... bench:       1,077 ns/iter (+/- 54)  2368 MB/s
  test short_word2_long::gs_find              ... bench:       4,723 ns/iter (+/- 141)  540 MB/s // NOTE
  test short_word2_long::memmem               ... bench:       1,708 ns/iter (+/- 52)  1493 MB/s

License
=======

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.


