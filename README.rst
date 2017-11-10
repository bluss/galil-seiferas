galil-seiferas
==============

General string search in constant space, linear time, for nonorderable alphabets.

Please read the `API documentation on docs.rs`__

__ https://docs.rs/galil-seiferas/

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/galil-seiferas.svg?branch=master
.. _build_status: https://travis-ci.org/bluss/galil-seiferas

.. |crates| image:: http://meritbadge.herokuapp.com/galil-seiferas
.. _crates: https://crates.io/crates/galil-seiferas

Recent Changes
--------------

- 0.1.0

  - Initial release


Benchmarks
----------

It's not fast — two way is a whole lot better. But we have one baseline, and that's
the "naive" brute force search which is quadratic in the size of the input, in
the worst case.

The repo https://github.com/bluss/scratchspace collects some string matching benchmarks,
and a lot of them are “pathologies” or cases that were known to be bad for some
algorithm. The result of one run are below. (Only subset available)

This run is just a quick one so it may be quite noisy. Unavoidably there are single
tests whose results are off.

The G-S algorithm is ``gs_find``; the KMP algorithm (in a simple implementation) is
``kmp_find``; the substring searcher in Rust is just ``find``::

  test naive_rev::bmh_find                    ... bench:          36 ns/iter (+/- 0) = 6944 MB/s
  test naive_rev::find                        ... bench:         383 ns/iter (+/- 20) = 652 MB/s
  test naive_rev::gs_find                     ... bench:         492 ns/iter (+/- 11) = 508 MB/s  // NOTE
  test naive_rev::kmp_find                    ... bench:         513 ns/iter (+/- 2) = 487 MB/s
  test naive_rev::memmem                      ... bench:          11 ns/iter (+/- 0) = 22727 MB/s
  test naive_rev::naive_search                ... bench:       1,457 ns/iter (+/- 69) = 171 MB/s
  test short_word1_long::bmh_find             ... bench:       1,505 ns/iter (+/- 94) = 1695 MB/s
  test short_word1_long::find                 ... bench:         743 ns/iter (+/- 43) = 3433 MB/s
  test short_word1_long::gs_find              ... bench:       7,608 ns/iter (+/- 482) = 335 MB/s  // NOTE
  test short_word1_long::kmp_find             ... bench:       6,489 ns/iter (+/- 175) = 393 MB/s
  test short_word1_long::memmem               ... bench:       1,612 ns/iter (+/- 45) = 1582 MB/s
  test short_word1_long::naive_search         ... bench:      15,386 ns/iter (+/- 842) = 165 MB/s


License
=======

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.


