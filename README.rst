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

  test naive::bmh_find                     ... bench:         833 ns/iter (+/- 223) = 300 MB/s
  test naive::brute_force                  ... bench:       1,907 ns/iter (+/- 472) = 131 MB/s
  test naive::find                         ... bench:         530 ns/iter (+/- 15) = 471 MB/s
  test naive::gs_find                      ... bench:         615 ns/iter (+/- 6) = 406 MB/s  // NOTE
  test naive::kmp_find                     ... bench:         714 ns/iter (+/- 24)   350 MB/s
  test naive::memmem                       ... bench:         185 ns/iter (+/- 4) = 1351 MB/s
  test naive_longpat::bmh_find             ... bench:     325,798 ns/iter (+/- 5,490) = 306 MB/s
  test naive_longpat::brute_force          ... bench:   2,161,608 ns/iter (+/- 120,669) = 46 MB/s
  test naive_longpat::find                 ... bench:     191,133 ns/iter (+/- 7,939) = 523 MB/s
  test naive_longpat::gs_find              ... bench:     260,667 ns/iter (+/- 9,659) = 383 MB/s // NOTE
  test naive_longpat::memmem               ... bench:      55,119 ns/iter (+/- 3,846) = 1814 MB/s
  test naive_rev::bmh_find                 ... bench:          36 ns/iter (+/- 1) = 6944 MB/s
  test naive_rev::brute_force              ... bench:         281 ns/iter (+/- 204) = 889 MB/s
  test naive_rev::find                     ... bench:         394 ns/iter (+/- 47) = 634 MB/s
  test naive_rev::gs_find                  ... bench:         292 ns/iter (+/- 20) = 856 MB/s  // NOTE
  test naive_rev::kmp_find                 ... bench:         514 ns/iter (+/- 17)   486 MB/s
  test naive_rev::memmem                   ... bench:          11 ns/iter (+/- 0) = 22727 MB/s
  test short_word1_long::bmh_find          ... bench:       1,486 ns/iter (+/- 53) = 1716 MB/s
  test short_word1_long::brute_force       ... bench:       4,042 ns/iter (+/- 226) = 631 MB/s
  test short_word1_long::find              ... bench:         729 ns/iter (+/- 28) = 3499 MB/s
  test short_word1_long::gs_find           ... bench:       4,791 ns/iter (+/- 184) = 532 MB/s  // NOTE
  test short_word1_long::kmp_find          ... bench:       6,515 ns/iter (+/- 272)  391 MB/s
  test short_word1_long::memmem            ... bench:       1,670 ns/iter (+/- 374) = 1527 MB/s
  test short_word2_long::bmh_find          ... bench:       2,623 ns/iter (+/- 131) = 972 MB/s
  test short_word2_long::brute_force       ... bench:       3,865 ns/iter (+/- 212) = 660 MB/s
  test short_word2_long::find              ... bench:       1,169 ns/iter (+/- 40) = 2182 MB/s
  test short_word2_long::gs_find           ... bench:       4,245 ns/iter (+/- 196) = 600 MB/s  // NOTE
  test short_word2_long::memmem            ... bench:       1,729 ns/iter (+/- 150) = 1475 MB/s

License
=======

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.


