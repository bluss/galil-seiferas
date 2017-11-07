galil-seiferas
==============

**Not published: no docs available**

**Please read the comments in the code**

Please read the `API documentation on docs.rs`__

__ https://docs.rs/galil-seiferas/

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/galil-seiferas.svg?branch=master
.. _build_status: https://travis-ci.org/bluss/galil-seiferas

.. |crates| image:: http://meritbadge.herokuapp.com/galil-seiferas
.. _crates: https://crates.io/crates/galil-seiferas

Recent Changes
--------------

- *

  - Initial release.


Benchmarks
----------

It's not fast — two way is a whole lot better. But we have one baseline, and that's
the "naive" brute force search which is quadratic in the size of the input, in
the worst case.

The repo https://github.com/bluss/scratchspace collects some string matching benchmarks,
and a lot of them are “pathologies” or cases that were known to be bad for some
algorithm. The result of one run are below. The ones called short are a bit
more realistic.

This run is just a quick one so it may be quite noisy. Unavoidably there are single
tests whose results are off.

The G-S algorithm is ``gs_find``; the KMP algorithm (in a simple implementation) is
``kmp_find``; the substring searcher in Rust is just ``find``::

  test aaab_in_aab::bmh_find                  ... bench:     511,929 ns/iter (+/- 20,876) = 586 MB/s
  test aaab_in_aab::find                      ... bench:     720,849 ns/iter (+/- 43,276) = 416 MB/s
  test aaab_in_aab::find_bytes                ... bench:     660,491 ns/iter (+/- 31,992) = 454 MB/s
  test aaab_in_aab::gs_find                   ... bench:     938,791 ns/iter (+/- 50,575) = 319 MB/s
  test aaab_in_aab::kmp_find                  ... bench:     523,729 ns/iter (+/- 34,090) = 572 MB/s
  test aaab_in_aab::memmem                    ... bench:     435,755 ns/iter (+/- 27,670) = 688 MB/s
  test aaab_in_aab::naive_search              ... bench:   1,997,863 ns/iter (+/- 93,199) = 150 MB/s
  test aaab_in_aab::regex_find                ... bench:   1,021,153 ns/iter (+/- 38,811) = 293 MB/s
  test aaab_in_aab::rfind                     ... bench:     532,718 ns/iter (+/- 12,408) = 563 MB/s
  test aaab_in_aab::twoway_find               ... bench:     711,942 ns/iter (+/- 43,576) = 421 MB/s
  test aaab_in_aab::twoway_rfind              ... bench:     524,567 ns/iter (+/- 7,290) = 571 MB/s
  test aaabbb::bmh_find                       ... bench:      18,886 ns/iter (+/- 1,133) = 15884 MB/s
  test aaabbb::find                           ... bench:     557,561 ns/iter (+/- 32,750) = 538 MB/s
  test aaabbb::find_bytes                     ... bench:     566,149 ns/iter (+/- 78,303) = 529 MB/s
  test aaabbb::gs_find                        ... bench:     843,925 ns/iter (+/- 33,336) = 355 MB/s
  test aaabbb::kmp_find                       ... bench:     526,483 ns/iter (+/- 27,616) = 569 MB/s
  test aaabbb::memmem                         ... bench:      20,259 ns/iter (+/- 1,242) = 14808 MB/s
  test aaabbb::naive_search                   ... bench:   2,060,424 ns/iter (+/- 106,046) = 145 MB/s
  test aaabbb::regex_find                     ... bench:     996,220 ns/iter (+/- 36,479) = 301 MB/s
  test aaabbb::rfind                          ... bench:     423,429 ns/iter (+/- 38,490) = 708 MB/s
  test aaabbb::twoway_find                    ... bench:     492,209 ns/iter (+/- 27,689) = 609 MB/s
  test aaabbb::twoway_rfind                   ... bench:     412,707 ns/iter (+/- 23,704) = 726 MB/s
  test allright::bmh_find                     ... bench:      99,899 ns/iter (+/- 38,613) = 1801 MB/s
  test allright::find                         ... bench:     271,687 ns/iter (+/- 23,108) = 662 MB/s
  test allright::find_bytes                   ... bench:     259,911 ns/iter (+/- 12,236) = 692 MB/s
  test allright::gs_find                      ... bench:     512,203 ns/iter (+/- 16,645) = 351 MB/s
  test allright::kmp_find                     ... bench:     341,468 ns/iter (+/- 11,982) = 527 MB/s
  test allright::memmem                       ... bench:     103,128 ns/iter (+/- 2,563) = 1745 MB/s
  test allright::naive_search                 ... bench:   1,099,276 ns/iter (+/- 51,405) = 163 MB/s
  test allright::regex_find                   ... bench:     607,484 ns/iter (+/- 21,478) = 296 MB/s
  test allright::rfind                        ... bench:     475,221 ns/iter (+/- 108,470) = 378 MB/s
  test allright::twoway_find                  ... bench:     242,348 ns/iter (+/- 10,842) = 742 MB/s
  test allright::twoway_rfind                 ... bench:     439,077 ns/iter (+/- 24,868) = 409 MB/s
  test bb_in_aa::bmh_find                     ... bench:       2,533 ns/iter (+/- 93) = 39478 MB/s
  test bb_in_aa::find                         ... bench:       1,350 ns/iter (+/- 90) = 74074 MB/s
  test bb_in_aa::find_bytes                   ... bench:       1,388 ns/iter (+/- 85) = 72046 MB/s
  test bb_in_aa::gs_find                      ... bench:     240,393 ns/iter (+/- 27,390) = 415 MB/s
  test bb_in_aa::kmp_find                     ... bench:     201,278 ns/iter (+/- 11,441) = 496 MB/s
  test bb_in_aa::memmem                       ... bench:       5,658 ns/iter (+/- 979) = 17674 MB/s
  test bb_in_aa::naive_search                 ... bench:     726,439 ns/iter (+/- 35,251) = 137 MB/s
  test bb_in_aa::regex_find                   ... bench:     331,917 ns/iter (+/- 21,098) = 301 MB/s
  test bb_in_aa::rfind                        ... bench:       1,263 ns/iter (+/- 51) = 79176 MB/s
  test bb_in_aa::twoway_find                  ... bench:       1,395 ns/iter (+/- 45) = 71684 MB/s
  test bb_in_aa::twoway_rfind                 ... bench:       1,249 ns/iter (+/- 82) = 80064 MB/s
  test bbbaaa::bmh_find                       ... bench:      43,783 ns/iter (+/- 2,084) = 6851 MB/s
  test bbbaaa::find                           ... bench:     398,017 ns/iter (+/- 15,467) = 753 MB/s
  test bbbaaa::find_bytes                     ... bench:     387,012 ns/iter (+/- 17,621) = 775 MB/s
  test bbbaaa::gs_find                        ... bench:     795,579 ns/iter (+/- 29,419) = 377 MB/s
  test bbbaaa::kmp_find                       ... bench:     553,335 ns/iter (+/- 33,628) = 542 MB/s
  test bbbaaa::memmem                         ... bench:     247,719 ns/iter (+/- 14,713) = 1211 MB/s
  test bbbaaa::naive_search                   ... bench:   2,006,718 ns/iter (+/- 80,150) = 149 MB/s
  test bbbaaa::regex_find                     ... bench:     991,502 ns/iter (+/- 40,058) = 302 MB/s
  test bbbaaa::rfind                          ... bench:     569,564 ns/iter (+/- 20,864) = 526 MB/s
  test bbbaaa::twoway_find                    ... bench:     396,464 ns/iter (+/- 26,015) = 756 MB/s
  test bbbaaa::twoway_rfind                   ... bench:     490,274 ns/iter (+/- 22,108) = 611 MB/s
  test gllright::bmh_find                     ... bench:      98,502 ns/iter (+/- 2,089) = 1827 MB/s
  test gllright::find                         ... bench:     361,512 ns/iter (+/- 21,553) = 497 MB/s
  test gllright::find_bytes                   ... bench:     364,932 ns/iter (+/- 24,329) = 493 MB/s
  test gllright::gs_find                      ... bench:     566,646 ns/iter (+/- 34,713) = 317 MB/s
  test gllright::kmp_find                     ... bench:     442,838 ns/iter (+/- 25,873) = 406 MB/s
  test gllright::memmem                       ... bench:     183,251 ns/iter (+/- 11,522) = 982 MB/s
  test gllright::naive_search                 ... bench:   1,154,938 ns/iter (+/- 48,384) = 155 MB/s
  test gllright::regex_find                   ... bench:     621,018 ns/iter (+/- 40,889) = 289 MB/s
  test gllright::rfind                        ... bench:     303,336 ns/iter (+/- 17,914) = 593 MB/s
  test gllright::twoway_find                  ... bench:     340,511 ns/iter (+/- 19,469) = 528 MB/s
  test gllright::twoway_rfind                 ... bench:     289,872 ns/iter (+/- 14,021) = 620 MB/s
  test naive::bmh_find                        ... bench:         825 ns/iter (+/- 8) = 303 MB/s
  test naive::find                            ... bench:         535 ns/iter (+/- 33) = 467 MB/s
  test naive::find_bytes                      ... bench:         540 ns/iter (+/- 36) = 462 MB/s
  test naive::gs_find                         ... bench:         741 ns/iter (+/- 36) = 337 MB/s
  test naive::kmp_find                        ... bench:         733 ns/iter (+/- 46) = 341 MB/s
  test naive::memmem                          ... bench:         187 ns/iter (+/- 10) = 1336 MB/s
  test naive::naive_search                    ... bench:         884 ns/iter (+/- 60) = 282 MB/s
  test naive::regex_find                      ... bench:         890 ns/iter (+/- 55) = 280 MB/s
  test naive::rfind                           ... bench:         387 ns/iter (+/- 23) = 645 MB/s
  test naive::twoway_find                     ... bench:         534 ns/iter (+/- 30) = 468 MB/s
  test naive::twoway_rfind                    ... bench:         392 ns/iter (+/- 15) = 637 MB/s
  test naive_longpat::bmh_find                ... bench:     328,899 ns/iter (+/- 14,568) = 304 MB/s
  test naive_longpat::find                    ... bench:     192,001 ns/iter (+/- 6,414) = 520 MB/s
  test naive_longpat::find_bytes              ... bench:     190,674 ns/iter (+/- 7,042) = 524 MB/s
  test naive_longpat::gs_find                 ... bench:     288,661 ns/iter (+/- 17,138) = 346 MB/s
  test naive_longpat::kmp_find                ... bench:     272,293 ns/iter (+/- 6,673) = 367 MB/s
  test naive_longpat::memmem                  ... bench:      55,775 ns/iter (+/- 3,491) = 1792 MB/s
  test naive_longpat::naive_search            ... bench:     411,384 ns/iter (+/- 11,976) = 243 MB/s
  test naive_longpat::regex_find              ... bench:     330,955 ns/iter (+/- 9,471) = 302 MB/s
  test naive_longpat::rfind                   ... bench:     120,060 ns/iter (+/- 3,392) = 832 MB/s
  test naive_longpat::twoway_find             ... bench:     190,758 ns/iter (+/- 7,510) = 524 MB/s
  test naive_longpat::twoway_rfind            ... bench:     120,497 ns/iter (+/- 7,545) = 829 MB/s
  test naive_longpat_reversed::bmh_find       ... bench:       2,502 ns/iter (+/- 126) = 39968 MB/s
  test naive_longpat_reversed::find           ... bench:     121,744 ns/iter (+/- 5,624) = 821 MB/s
  test naive_longpat_reversed::find_bytes     ... bench:     118,384 ns/iter (+/- 4,997) = 844 MB/s
  test naive_longpat_reversed::gs_find        ... bench:     227,998 ns/iter (+/- 13,047) = 438 MB/s
  test naive_longpat_reversed::kmp_find       ... bench:     201,961 ns/iter (+/- 9,709) = 495 MB/s
  test naive_longpat_reversed::memmem         ... bench:       2,400 ns/iter (+/- 105) = 41666 MB/s
  test naive_longpat_reversed::naive_search   ... bench:     689,989 ns/iter (+/- 36,250) = 144 MB/s
  test naive_longpat_reversed::regex_find     ... bench:     338,866 ns/iter (+/- 19,104) = 295 MB/s
  test naive_longpat_reversed::rfind          ... bench:     195,322 ns/iter (+/- 10,888) = 511 MB/s
  test naive_longpat_reversed::twoway_find    ... bench:     114,840 ns/iter (+/- 3,720) = 870 MB/s
  test naive_longpat_reversed::twoway_rfind   ... bench:     195,190 ns/iter (+/- 7,991) = 512 MB/s
  test naive_rev::bmh_find                    ... bench:          36 ns/iter (+/- 1) = 6944 MB/s
  test naive_rev::find                        ... bench:       1,170 ns/iter (+/- 803) = 213 MB/s
  test naive_rev::find_bytes                  ... bench:       1,130 ns/iter (+/- 34) = 221 MB/s
  test naive_rev::gs_find                     ... bench:       1,927 ns/iter (+/- 30) = 129 MB/s
  test naive_rev::kmp_find                    ... bench:       1,791 ns/iter (+/- 18) = 139 MB/s
  test naive_rev::memmem                      ... bench:          11 ns/iter (+/- 0) = 22727 MB/s
  test naive_rev::naive_search                ... bench:       4,124 ns/iter (+/- 2,282) = 60 MB/s
  test naive_rev::regex_find                  ... bench:         872 ns/iter (+/- 38) = 286 MB/s
  test naive_rev::rfind                       ... bench:         519 ns/iter (+/- 21) = 481 MB/s
  test naive_rev::twoway_find                 ... bench:         358 ns/iter (+/- 14) = 698 MB/s
  test naive_rev::twoway_rfind                ... bench:         534 ns/iter (+/- 25) = 468 MB/s
  test pathological_two_way::bmh_find         ... bench:       1,465 ns/iter (+/- 34) = 40955 MB/s
  test pathological_two_way::find             ... bench:     119,807 ns/iter (+/- 3,298) = 500 MB/s
  test pathological_two_way::find_bytes       ... bench:     103,934 ns/iter (+/- 4,038) = 577 MB/s
  test pathological_two_way::gs_find          ... bench:     147,139 ns/iter (+/- 4,396) = 407 MB/s
  test pathological_two_way::kmp_find         ... bench:     118,058 ns/iter (+/- 2,147) = 508 MB/s
  test pathological_two_way::memmem           ... bench:       1,432 ns/iter (+/- 54) = 41899 MB/s
  test pathological_two_way::naive_search     ... bench:     279,244 ns/iter (+/- 10,211) = 214 MB/s
  test pathological_two_way::regex_find       ... bench:     204,889 ns/iter (+/- 11,956) = 292 MB/s
  test pathological_two_way::rfind            ... bench:      11,412 ns/iter (+/- 663) = 5257 MB/s
  test pathological_two_way::twoway_find      ... bench:      81,895 ns/iter (+/- 2,332) = 732 MB/s
  test pathological_two_way::twoway_rfind     ... bench:      11,444 ns/iter (+/- 1,247) = 5242 MB/s
  test pathological_two_way_rev::bmh_find     ... bench:      65,745 ns/iter (+/- 2,245) = 912 MB/s
  test pathological_two_way_rev::find         ... bench:      11,044 ns/iter (+/- 434) = 5432 MB/s
  test pathological_two_way_rev::find_bytes   ... bench:      11,761 ns/iter (+/- 508) = 5101 MB/s
  test pathological_two_way_rev::gs_find      ... bench:     168,757 ns/iter (+/- 2,345) = 355 MB/s
  test pathological_two_way_rev::kmp_find     ... bench:     125,297 ns/iter (+/- 4,663) = 478 MB/s
  test pathological_two_way_rev::memmem       ... bench:      43,686 ns/iter (+/- 1,364) = 1373 MB/s
  test pathological_two_way_rev::naive_search ... bench:     267,962 ns/iter (+/- 9,787) = 223 MB/s
  test pathological_two_way_rev::regex_find   ... bench:     202,667 ns/iter (+/- 8,781) = 296 MB/s
  test pathological_two_way_rev::rfind        ... bench:     120,396 ns/iter (+/- 3,663) = 498 MB/s
  test pathological_two_way_rev::twoway_find  ... bench:      11,741 ns/iter (+/- 681) = 5110 MB/s
  test pathological_two_way_rev::twoway_rfind ... bench:     131,424 ns/iter (+/- 5,232) = 456 MB/s
  test periodic2::bmh_find                    ... bench:      95,734 ns/iter (+/- 11,591) = 208 MB/s
  test periodic2::find                        ... bench:      25,153 ns/iter (+/- 2,237) = 795 MB/s
  test periodic2::find_bytes                  ... bench:      24,028 ns/iter (+/- 1,615) = 832 MB/s
  test periodic2::gs_find                     ... bench:     228,936 ns/iter (+/- 10,813) = 87 MB/s
  test periodic2::kmp_find                    ... bench:     104,922 ns/iter (+/- 71,222) = 190 MB/s
  test periodic2::memmem                      ... bench:      41,358 ns/iter (+/- 27,718) = 483 MB/s
  test periodic2::naive_search                ... bench:     168,561 ns/iter (+/- 322,760) = 118 MB/s
  test periodic2::regex_find                  ... bench:     168,068 ns/iter (+/- 109,335) = 118 MB/s
  test periodic2::rfind                       ... bench:      24,088 ns/iter (+/- 51,385) = 830 MB/s
  test periodic2::twoway_find                 ... bench:      73,701 ns/iter (+/- 1,599) = 271 MB/s
  test periodic2::twoway_rfind                ... bench:      23,943 ns/iter (+/- 49,186) = 835 MB/s
  test periodic5::bmh_find                    ... bench:      51,822 ns/iter (+/- 206) = 154 MB/s
  test periodic5::find                        ... bench:      10,423 ns/iter (+/- 317) = 767 MB/s
  test periodic5::find_bytes                  ... bench:      10,432 ns/iter (+/- 452) = 766 MB/s
  test periodic5::gs_find                     ... bench:      63,548 ns/iter (+/- 7,569) = 125 MB/s
  test periodic5::kmp_find                    ... bench:      14,424 ns/iter (+/- 950) = 554 MB/s
  test periodic5::memmem                      ... bench:       6,556 ns/iter (+/- 13,716) = 1220 MB/s
  test periodic5::naive_search                ... bench:      55,735 ns/iter (+/- 1,645) = 143 MB/s
  test periodic5::regex_find                  ... bench:      28,166 ns/iter (+/- 922) = 284 MB/s
  test periodic5::rfind                       ... bench:       9,960 ns/iter (+/- 455) = 803 MB/s
  test periodic5::twoway_find                 ... bench:      10,557 ns/iter (+/- 478) = 757 MB/s
  test periodic5::twoway_rfind                ... bench:       9,977 ns/iter (+/- 102) = 801 MB/s
  test short_1let_cy::bmh_find                ... bench:       9,260 ns/iter (+/- 306) = 554 MB/s
  test short_1let_cy::find                    ... bench:       2,770 ns/iter (+/- 146) = 1852 MB/s
  test short_1let_cy::find_bytes              ... bench:       2,968 ns/iter (+/- 172) = 1729 MB/s
  test short_1let_cy::gs_find                 ... bench:      19,185 ns/iter (+/- 594) = 267 MB/s
  test short_1let_cy::kmp_find                ... bench:      16,518 ns/iter (+/- 743) = 310 MB/s
  test short_1let_cy::memmem                  ... bench:       2,836 ns/iter (+/- 86) = 1809 MB/s
  test short_1let_cy::naive_search            ... bench:      27,256 ns/iter (+/- 1,167) = 188 MB/s
  test short_1let_cy::regex_find              ... bench:      18,450 ns/iter (+/- 936) = 278 MB/s
  test short_1let_cy::rfind                   ... bench:       4,327 ns/iter (+/- 131) = 1186 MB/s
  test short_1let_cy::twoway_find             ... bench:       3,058 ns/iter (+/- 127) = 1678 MB/s
  test short_1let_cy::twoway_rfind            ... bench:       3,818 ns/iter (+/- 122) = 1344 MB/s
  test short_1let_long::bmh_find              ... bench:          81 ns/iter (+/- 171) = 31493 MB/s
  test short_1let_long::find                  ... bench:       6,478 ns/iter (+/- 21) = 393 MB/s
  test short_1let_long::find_bytes            ... bench:         206 ns/iter (+/- 1) = 12383 MB/s
  test short_1let_long::gs_find               ... bench:      19,232 ns/iter (+/- 125) = 132 MB/s
  test short_1let_long::kmp_find              ... bench:      15,519 ns/iter (+/- 6,056) = 164 MB/s
  test short_1let_long::memmem                ... bench:          65 ns/iter (+/- 1) = 39246 MB/s
  test short_1let_long::naive_search          ... bench:      25,640 ns/iter (+/- 17,299) = 99 MB/s
  test short_1let_long::regex_find            ... bench:       6,765 ns/iter (+/- 194) = 377 MB/s
  test short_1let_long::rfind                 ... bench:       4,573 ns/iter (+/- 3,667) = 557 MB/s
  test short_1let_long::twoway_find           ... bench:       2,128 ns/iter (+/- 44) = 1198 MB/s
  test short_1let_long::twoway_rfind          ... bench:       5,490 ns/iter (+/- 3,706) = 464 MB/s
  test short_2let_common::bmh_find            ... bench:      15,117 ns/iter (+/- 7,948) = 168 MB/s
  test short_2let_common::find                ... bench:       4,722 ns/iter (+/- 136) = 540 MB/s
  test short_2let_common::find_bytes          ... bench:       4,460 ns/iter (+/- 117) = 571 MB/s
  test short_2let_common::gs_find             ... bench:      22,713 ns/iter (+/- 1,502) = 112 MB/s
  test short_2let_common::kmp_find            ... bench:       6,739 ns/iter (+/- 13,132) = 378 MB/s
  test short_2let_common::memmem              ... bench:       6,847 ns/iter (+/- 11,453) = 372 MB/s
  test short_2let_common::naive_search        ... bench:      35,640 ns/iter (+/- 1,639) = 71 MB/s
  test short_2let_common::regex_find          ... bench:      20,640 ns/iter (+/- 974) = 123 MB/s
  test short_2let_common::rfind               ... bench:       3,821 ns/iter (+/- 133) = 667 MB/s
  test short_2let_common::twoway_find         ... bench:       4,700 ns/iter (+/- 159) = 542 MB/s
  test short_2let_common::twoway_rfind        ... bench:       4,627 ns/iter (+/- 3,174) = 551 MB/s
  test short_2let_cy::bmh_find                ... bench:       5,630 ns/iter (+/- 11,151) = 911 MB/s
  test short_2let_cy::find                    ... bench:       2,514 ns/iter (+/- 141) = 2041 MB/s
  test short_2let_cy::find_bytes              ... bench:       7,258 ns/iter (+/- 391) = 707 MB/s
  test short_2let_cy::gs_find                 ... bench:      48,263 ns/iter (+/- 40,304) = 106 MB/s
  test short_2let_cy::kmp_find                ... bench:      17,967 ns/iter (+/- 912) = 285 MB/s
  test short_2let_cy::memmem                  ... bench:       8,280 ns/iter (+/- 16,434) = 619 MB/s
  test short_2let_cy::naive_search            ... bench:     107,397 ns/iter (+/- 924) = 47 MB/s
  test short_2let_cy::regex_find              ... bench:      57,357 ns/iter (+/- 1,227) = 89 MB/s
  test short_2let_cy::rfind                   ... bench:       7,775 ns/iter (+/- 232) = 660 MB/s
  test short_2let_cy::twoway_find             ... bench:       6,596 ns/iter (+/- 6,318) = 778 MB/s
  test short_2let_cy::twoway_rfind            ... bench:       2,731 ns/iter (+/- 122) = 1879 MB/s
  test short_2let_rare::bmh_find              ... bench:       4,421 ns/iter (+/- 245) = 577 MB/s
  test short_2let_rare::find                  ... bench:       1,266 ns/iter (+/- 55) = 2015 MB/s
  test short_2let_rare::find_bytes            ... bench:       1,249 ns/iter (+/- 82) = 2042 MB/s
  test short_2let_rare::gs_find               ... bench:       6,769 ns/iter (+/- 450) = 376 MB/s
  test short_2let_rare::kmp_find              ... bench:       5,487 ns/iter (+/- 322) = 464 MB/s
  test short_2let_rare::memmem                ... bench:       9,598 ns/iter (+/- 10,462) = 265 MB/s
  test short_2let_rare::naive_search          ... bench:      12,486 ns/iter (+/- 32,433) = 204 MB/s
  test short_2let_rare::regex_find            ... bench:      21,019 ns/iter (+/- 9,253) = 121 MB/s
  test short_2let_rare::rfind                 ... bench:       3,050 ns/iter (+/- 2,526) = 836 MB/s
  test short_2let_rare::twoway_find           ... bench:       3,743 ns/iter (+/- 137) = 681 MB/s
  test short_2let_rare::twoway_rfind          ... bench:       3,291 ns/iter (+/- 28) = 775 MB/s
  test short_3let_cy::bmh_find                ... bench:      11,988 ns/iter (+/- 322) = 428 MB/s
  test short_3let_cy::find                    ... bench:       6,373 ns/iter (+/- 3,744) = 805 MB/s
  test short_3let_cy::find_bytes              ... bench:       5,747 ns/iter (+/- 1,564) = 892 MB/s
  test short_3let_cy::gs_find                 ... bench:      67,776 ns/iter (+/- 12,552) = 75 MB/s
  test short_3let_cy::kmp_find                ... bench:      18,796 ns/iter (+/- 954) = 273 MB/s
  test short_3let_cy::memmem                  ... bench:      10,807 ns/iter (+/- 170) = 474 MB/s
  test short_3let_cy::naive_search            ... bench:     107,777 ns/iter (+/- 21,564) = 47 MB/s
  test short_3let_cy::regex_find              ... bench:      57,206 ns/iter (+/- 2,026) = 89 MB/s
  test short_3let_cy::rfind                   ... bench:       7,944 ns/iter (+/- 1,702) = 646 MB/s
  test short_3let_cy::twoway_find             ... bench:       5,875 ns/iter (+/- 231) = 873 MB/s
  test short_3let_cy::twoway_rfind            ... bench:       6,933 ns/iter (+/- 581) = 740 MB/s
  test short_3let_long::bmh_find              ... bench:       9,638 ns/iter (+/- 267) = 264 MB/s
  test short_3let_long::find                  ... bench:       3,102 ns/iter (+/- 96) = 822 MB/s
  test short_3let_long::find_bytes            ... bench:       3,012 ns/iter (+/- 143) = 846 MB/s
  test short_3let_long::gs_find               ... bench:      22,509 ns/iter (+/- 1,855) = 113 MB/s
  test short_3let_long::kmp_find              ... bench:      19,197 ns/iter (+/- 708) = 132 MB/s
  test short_3let_long::memmem                ... bench:       9,992 ns/iter (+/- 786) = 255 MB/s
  test short_3let_long::naive_search          ... bench:      39,604 ns/iter (+/- 2,358) = 64 MB/s
  test short_3let_long::regex_find            ... bench:      20,736 ns/iter (+/- 601) = 123 MB/s
  test short_3let_long::rfind                 ... bench:       2,496 ns/iter (+/- 33) = 1022 MB/s
  test short_3let_long::twoway_find           ... bench:       3,096 ns/iter (+/- 88) = 823 MB/s
  test short_3let_long::twoway_rfind          ... bench:       2,863 ns/iter (+/- 79) = 891 MB/s
  test short_short::bmh_find                  ... bench:         215 ns/iter (+/- 3) = 260 MB/s
  test short_short::find                      ... bench:         171 ns/iter (+/- 6) = 327 MB/s
  test short_short::find_bytes                ... bench:         156 ns/iter (+/- 3) = 358 MB/s
  test short_short::gs_find                   ... bench:         494 ns/iter (+/- 19) = 113 MB/s
  test short_short::kmp_find                  ... bench:         455 ns/iter (+/- 14) = 123 MB/s
  test short_short::memmem                    ... bench:         111 ns/iter (+/- 9) = 504 MB/s
  test short_short::naive_search              ... bench:         844 ns/iter (+/- 24) = 66 MB/s
  test short_short::regex_find                ... bench:         649 ns/iter (+/- 488) = 86 MB/s
  test short_short::rfind                     ... bench:         152 ns/iter (+/- 90) = 368 MB/s
  test short_short::twoway_find               ... bench:         170 ns/iter (+/- 5) = 329 MB/s
  test short_short::twoway_rfind              ... bench:         151 ns/iter (+/- 102) = 370 MB/s
  test short_word1_long::bmh_find             ... bench:       4,280 ns/iter (+/- 2,900) = 596 MB/s
  test short_word1_long::find                 ... bench:       2,269 ns/iter (+/- 2,283) = 1124 MB/s
  test short_word1_long::find_bytes           ... bench:       2,141 ns/iter (+/- 28) = 1191 MB/s
  test short_word1_long::gs_find              ... bench:      22,056 ns/iter (+/- 349) = 115 MB/s
  test short_word1_long::kmp_find             ... bench:      19,211 ns/iter (+/- 349) = 132 MB/s
  test short_word1_long::memmem               ... bench:       4,950 ns/iter (+/- 114) = 515 MB/s
  test short_word1_long::naive_search         ... bench:      15,326 ns/iter (+/- 30,917) = 166 MB/s
  test short_word1_long::regex_find           ... bench:      20,651 ns/iter (+/- 368) = 123 MB/s
  test short_word1_long::rfind                ... bench:       2,379 ns/iter (+/- 49) = 1072 MB/s
  test short_word1_long::twoway_find          ... bench:       2,294 ns/iter (+/- 32) = 1112 MB/s
  test short_word1_long::twoway_rfind         ... bench:         644 ns/iter (+/- 120) = 3961 MB/s
  test short_word2_long::bmh_find             ... bench:       2,687 ns/iter (+/- 176) = 949 MB/s
  test short_word2_long::find                 ... bench:       1,222 ns/iter (+/- 54) = 2087 MB/s
  test short_word2_long::find_bytes           ... bench:       1,240 ns/iter (+/- 2,164) = 2057 MB/s
  test short_word2_long::gs_find              ... bench:      21,476 ns/iter (+/- 433) = 118 MB/s
  test short_word2_long::kmp_find             ... bench:      18,501 ns/iter (+/- 355) = 137 MB/s
  test short_word2_long::memmem               ... bench:       5,229 ns/iter (+/- 46) = 487 MB/s
  test short_word2_long::naive_search         ... bench:      44,810 ns/iter (+/- 19,111) = 56 MB/s
  test short_word2_long::regex_find           ... bench:       6,745 ns/iter (+/- 287) = 378 MB/s
  test short_word2_long::rfind                ... bench:       3,504 ns/iter (+/- 2,131) = 728 MB/s
  test short_word2_long::twoway_find          ... bench:       3,069 ns/iter (+/- 2,206) = 831 MB/s
  test short_word2_long::twoway_rfind         ... bench:       2,883 ns/iter (+/- 23) = 884 MB/s

License
=======

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.


