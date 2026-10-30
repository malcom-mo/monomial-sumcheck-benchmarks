# Benchmark summary

Times are process-level Criterion medians. Intervals are 99% bootstrap
confidence intervals for the median across independent processes.
The p01 and p99 columns are empirical process-level quantiles; Min and
Max are the smallest and largest observed values.

## BB4 — claim-aware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 21.805 ms | [21.782, 21.818] ms | 21.783 ms | 21.818 ms |
| Unweighted | projective | 20 | 5 | 20.737 ms | [20.711, 20.748] ms | 20.711 ms | 20.748 ms |
| Unweighted | flipped | 20 | 5 | 20.277 ms | [20.257, 20.294] ms | 20.257 ms | 20.294 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 16.306 ms | [16.291, 16.322] ms | 16.292 ms | 16.322 ms |
| Unweighted | projective (delayed) | 20 | 5 | 15.299 ms | [15.274, 15.311] ms | 15.275 ms | 15.311 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **14.069 ms** | [14.049, 14.087] ms | 14.050 ms | 14.087 ms |
| Weighted | boolean | 20 | 5 | 31.645 ms | [31.599, 31.672] ms | 31.599 ms | 31.671 ms |
| Weighted | projective | 20 | 5 | 31.336 ms | [31.301, 31.362] ms | 31.301 ms | 31.361 ms |
| Weighted | projective-different-claims | 20 | 5 | 26.266 ms | [26.231, 26.344] ms | 26.231 ms | 26.342 ms |
| Weighted | flipped-different-claims | 20 | 5 | 25.509 ms | [25.477, 25.606] ms | 25.479 ms | 25.603 ms |
| Weighted | boolean (delayed) | 20 | 5 | 27.439 ms | [27.410, 27.455] ms | 27.410 ms | 27.454 ms |
| Weighted | projective (delayed) | 20 | 5 | 26.817 ms | [26.791, 26.902] ms | 26.791 ms | 26.899 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 20.375 ms | [20.348, 20.499] ms | 20.348 ms | 20.495 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **19.912 ms** | [19.868, 20.022] ms | 19.869 ms | 20.018 ms |

## BB4 — claim-unaware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 26.312 ms | [26.275, 26.318] ms | 26.276 ms | 26.318 ms |
| Unweighted | projective | 20 | 5 | 25.016 ms | [24.981, 25.030] ms | 24.981 ms | 25.029 ms |
| Unweighted | flipped | 20 | 5 | 25.417 ms | [25.395, 25.422] ms | 25.395 ms | 25.422 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 21.610 ms | [21.602, 21.640] ms | 21.602 ms | 21.640 ms |
| Unweighted | projective (delayed) | 20 | 5 | 20.255 ms | [20.238, 20.333] ms | 20.239 ms | 20.330 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **19.607 ms** | [19.595, 19.622] ms | 19.595 ms | 19.621 ms |
| Weighted | boolean | 20 | 5 | 36.579 ms | [36.528, 36.615] ms | 36.528 ms | 36.614 ms |
| Weighted | projective | 20 | 5 | 35.831 ms | [35.793, 35.863] ms | 35.794 ms | 35.862 ms |
| Weighted | projective-different-claims | 20 | 5 | 30.753 ms | [30.729, 30.960] ms | 30.730 ms | 30.952 ms |
| Weighted | flipped-different-claims | 20 | 5 | 30.785 ms | [30.761, 30.997] ms | 30.762 ms | 30.989 ms |
| Weighted | boolean (delayed) | 20 | 5 | 39.023 ms | [38.979, 39.056] ms | 38.979 ms | 39.055 ms |
| Weighted | projective (delayed) | 20 | 5 | 32.990 ms | [32.958, 33.017] ms | 32.959 ms | 33.016 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | **29.755 ms** | [29.737, 29.853] ms | 29.737 ms | 29.850 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | 30.476 ms | [30.441, 30.948] ms | 30.441 ms | 30.931 ms |

## BB5 — claim-aware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 25.756 ms | [25.713, 25.764] ms | 25.713 ms | 25.764 ms |
| Unweighted | projective | 20 | 5 | 25.040 ms | [25.004, 25.053] ms | 25.005 ms | 25.052 ms |
| Unweighted | flipped | 20 | 5 | 24.779 ms | [24.743, 24.790] ms | 24.744 ms | 24.790 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 25.225 ms | [25.200, 25.231] ms | 25.200 ms | 25.231 ms |
| Unweighted | projective (delayed) | 20 | 5 | 24.726 ms | [24.690, 24.733] ms | 24.690 ms | 24.733 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **24.104 ms** | [24.081, 24.113] ms | 24.081 ms | 24.113 ms |
| Weighted | boolean | 20 | 5 | 46.227 ms | [46.217, 46.290] ms | 46.217 ms | 46.290 ms |
| Weighted | projective | 20 | 5 | 47.052 ms | [46.990, 47.234] ms | 46.991 ms | 47.227 ms |
| Weighted | projective-different-claims | 20 | 5 | 35.851 ms | [35.789, 35.860] ms | 35.790 ms | 35.859 ms |
| Weighted | flipped-different-claims | 20 | 5 | 37.850 ms | [37.844, 37.907] ms | 37.844 ms | 37.907 ms |
| Weighted | boolean (delayed) | 20 | 5 | 39.625 ms | [39.586, 39.652] ms | 39.586 ms | 39.652 ms |
| Weighted | projective (delayed) | 20 | 5 | 39.085 ms | [39.073, 39.119] ms | 39.073 ms | 39.118 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 32.528 ms | [32.514, 32.572] ms | 32.514 ms | 32.572 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **31.030 ms** | [30.984, 31.049] ms | 30.984 ms | 31.049 ms |

## BB5 — claim-unaware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 31.560 ms | [31.529, 31.563] ms | 31.529 ms | 31.563 ms |
| Unweighted | projective | 20 | 5 | **30.685 ms** | [30.651, 30.692] ms | 30.652 ms | 30.692 ms |
| Unweighted | flipped | 20 | 5 | 31.178 ms | [31.122, 31.179] ms | 31.123 ms | 31.179 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 33.310 ms | [33.265, 33.318] ms | 33.267 ms | 33.318 ms |
| Unweighted | projective (delayed) | 20 | 5 | 32.341 ms | [32.307, 32.365] ms | 32.308 ms | 32.364 ms |
| Unweighted | flipped (delayed) | 20 | 5 | 32.154 ms | [32.109, 32.166] ms | 32.109 ms | 32.166 ms |
| Weighted | boolean | 20 | 5 | 52.527 ms | [52.481, 52.555] ms | 52.483 ms | 52.555 ms |
| Weighted | projective | 20 | 5 | 52.683 ms | [52.600, 52.931] ms | 52.604 ms | 52.922 ms |
| Weighted | projective-different-claims | 20 | 5 | 41.630 ms | [41.595, 41.647] ms | 41.595 ms | 41.647 ms |
| Weighted | flipped-different-claims | 20 | 5 | 41.644 ms | [41.605, 41.650] ms | 41.605 ms | 41.650 ms |
| Weighted | boolean (delayed) | 20 | 5 | 43.810 ms | [43.730, 43.815] ms | 43.730 ms | 43.815 ms |
| Weighted | projective (delayed) | 20 | 5 | 43.212 ms | [43.162, 43.223] ms | 43.163 ms | 43.222 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | **35.868 ms** | [35.852, 35.928] ms | 35.852 ms | 35.926 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | 36.288 ms | [36.225, 36.302] ms | 36.226 ms | 36.302 ms |

## BN254 — claim-aware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 158.141 ms | [157.907, 158.351] ms | 157.910 ms | 158.347 ms |
| Unweighted | projective | 20 | 5 | 139.458 ms | [139.279, 139.647] ms | 139.285 ms | 139.644 ms |
| Unweighted | flipped | 20 | 5 | 123.586 ms | [123.481, 123.713] ms | 123.484 ms | 123.711 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 99.357 ms | [99.280, 99.439] ms | 99.282 ms | 99.437 ms |
| Unweighted | projective (delayed) | 20 | 5 | 89.760 ms | [89.720, 89.869] ms | 89.721 ms | 89.865 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **78.202 ms** | [78.117, 78.342] ms | 78.118 ms | 78.339 ms |
| Weighted | boolean | 20 | 5 | 202.286 ms | [201.945, 202.535] ms | 201.947 ms | 202.526 ms |
| Weighted | projective | 20 | 5 | 185.574 ms | [185.510, 185.917] ms | 185.510 ms | 185.911 ms |
| Weighted | projective-different-claims | 20 | 5 | 162.835 ms | [162.801, 163.136] ms | 162.802 ms | 163.131 ms |
| Weighted | flipped-different-claims | 20 | 5 | 145.628 ms | [145.484, 145.934] ms | 145.487 ms | 145.928 ms |
| Weighted | boolean (delayed) | 20 | 5 | 145.736 ms | [145.686, 145.858] ms | 145.688 ms | 145.857 ms |
| Weighted | projective (delayed) | 20 | 5 | 131.712 ms | [131.612, 132.057] ms | 131.612 ms | 132.048 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 111.797 ms | [111.726, 112.229] ms | 111.728 ms | 112.215 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **97.958 ms** | [97.878, 98.382] ms | 97.880 ms | 98.369 ms |

## BN254 — claim-unaware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 186.256 ms | [186.071, 186.696] ms | 186.073 ms | 186.685 ms |
| Unweighted | projective | 20 | 5 | 166.151 ms | [166.030, 166.420] ms | 166.034 ms | 166.414 ms |
| Unweighted | flipped | 20 | 5 | 166.413 ms | [166.170, 166.488] ms | 166.178 ms | 166.486 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 109.828 ms | [109.699, 109.934] ms | 109.700 ms | 109.931 ms |
| Unweighted | projective (delayed) | 20 | 5 | **99.557 ms** | [99.544, 99.791] ms | 99.544 ms | 99.782 ms |
| Unweighted | flipped (delayed) | 20 | 5 | 100.964 ms | [100.852, 101.091] ms | 100.853 ms | 101.085 ms |
| Weighted | boolean | 20 | 5 | 229.894 ms | [229.693, 230.514] ms | 229.697 ms | 230.505 ms |
| Weighted | projective | 20 | 5 | 210.344 ms | [210.101, 210.984] ms | 210.109 ms | 210.966 ms |
| Weighted | projective-different-claims | 20 | 5 | 190.171 ms | [190.074, 191.050] ms | 190.077 ms | 191.022 ms |
| Weighted | flipped-different-claims | 20 | 5 | 190.283 ms | [190.235, 190.959] ms | 190.237 ms | 190.938 ms |
| Weighted | boolean (delayed) | 20 | 5 | 150.257 ms | [150.190, 150.362] ms | 150.192 ms | 150.361 ms |
| Weighted | projective (delayed) | 20 | 5 | 142.271 ms | [142.191, 142.912] ms | 142.192 ms | 142.886 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | **122.207 ms** | [122.191, 122.524] ms | 122.191 ms | 122.517 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | 126.725 ms | [126.549, 126.884] ms | 126.551 ms | 126.879 ms |

## BN254_bf — claim-aware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 118.014 ms | [117.973, 118.392] ms | 117.974 ms | 118.378 ms |
| Unweighted | projective | 20 | 5 | 107.540 ms | [107.456, 107.837] ms | 107.457 ms | 107.831 ms |
| Unweighted | flipped | 20 | 5 | 94.704 ms | [94.562, 95.114] ms | 94.563 ms | 95.104 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 92.267 ms | [92.220, 92.398] ms | 92.221 ms | 92.396 ms |
| Unweighted | projective (delayed) | 20 | 5 | 80.873 ms | [80.814, 81.072] ms | 80.815 ms | 81.067 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **69.502 ms** | [69.487, 69.727] ms | 69.487 ms | 69.718 ms |
| Weighted | boolean | 20 | 5 | 160.869 ms | [160.782, 161.102] ms | 160.784 ms | 161.096 ms |
| Weighted | projective | 20 | 5 | 147.342 ms | [147.229, 147.679] ms | 147.230 ms | 147.670 ms |
| Weighted | projective-different-claims | 20 | 5 | 126.857 ms | [126.772, 127.209] ms | 126.773 ms | 127.199 ms |
| Weighted | flipped-different-claims | 20 | 5 | 114.838 ms | [114.799, 115.174] ms | 114.800 ms | 115.166 ms |
| Weighted | boolean (delayed) | 20 | 5 | 132.713 ms | [132.674, 133.023] ms | 132.674 ms | 133.015 ms |
| Weighted | projective (delayed) | 20 | 5 | 121.389 ms | [121.265, 121.603] ms | 121.266 ms | 121.597 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 101.128 ms | [101.064, 102.148] ms | 101.065 ms | 102.110 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **89.396 ms** | [89.321, 90.210] ms | 89.322 ms | 90.181 ms |

## BN254_bf — claim-unaware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 136.109 ms | [136.060, 136.682] ms | 136.060 ms | 136.659 ms |
| Unweighted | projective | 20 | 5 | 130.191 ms | [130.104, 130.716] ms | 130.105 ms | 130.699 ms |
| Unweighted | flipped | 20 | 5 | 128.516 ms | [128.473, 128.633] ms | 128.474 ms | 128.632 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 102.902 ms | [102.860, 103.286] ms | 102.860 ms | 103.275 ms |
| Unweighted | projective (delayed) | 20 | 5 | **91.300 ms** | [91.241, 91.694] ms | 91.243 ms | 91.683 ms |
| Unweighted | flipped (delayed) | 20 | 5 | 91.364 ms | [91.330, 91.605] ms | 91.331 ms | 91.598 ms |
| Weighted | boolean | 20 | 5 | 175.996 ms | [175.903, 176.161] ms | 175.903 ms | 176.158 ms |
| Weighted | projective | 20 | 5 | 169.765 ms | [169.605, 170.118] ms | 169.606 ms | 170.106 ms |
| Weighted | projective-different-claims | 20 | 5 | 149.473 ms | [149.412, 149.846] ms | 149.413 ms | 149.837 ms |
| Weighted | flipped-different-claims | 20 | 5 | 150.070 ms | [150.027, 150.318] ms | 150.028 ms | 150.316 ms |
| Weighted | boolean (delayed) | 20 | 5 | 142.598 ms | [142.560, 142.927] ms | 142.561 ms | 142.920 ms |
| Weighted | projective (delayed) | 20 | 5 | 131.329 ms | [131.179, 131.731] ms | 131.182 ms | 131.718 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 112.008 ms | [111.954, 112.307] ms | 111.955 ms | 112.300 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **111.819 ms** | [111.764, 112.081] ms | 111.764 ms | 112.075 ms |

## BN254_upper — claim-aware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 111.124 ms | [111.039, 111.502] ms | 111.043 ms | 111.494 ms |
| Unweighted | projective | 20 | 5 | 100.983 ms | [100.941, 101.410] ms | 100.942 ms | 101.397 ms |
| Unweighted | flipped | 20 | 5 | 89.998 ms | [89.930, 90.587] ms | 89.932 ms | 90.566 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 79.035 ms | [78.989, 79.636] ms | 78.990 ms | 79.615 ms |
| Unweighted | projective (delayed) | 20 | 5 | 69.915 ms | [69.890, 70.309] ms | 69.890 ms | 70.297 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **58.674 ms** | [58.657, 58.868] ms | 58.657 ms | 58.863 ms |
| Weighted | boolean | 20 | 5 | 150.999 ms | [150.720, 151.447] ms | 150.729 ms | 151.439 ms |
| Weighted | projective | 20 | 5 | 144.002 ms | [143.788, 144.767] ms | 143.796 ms | 144.759 ms |
| Weighted | projective-different-claims | 20 | 5 | 124.028 ms | [123.855, 125.006] ms | 123.860 ms | 124.968 ms |
| Weighted | flipped-different-claims | 20 | 5 | 111.276 ms | [111.011, 112.522] ms | 111.020 ms | 112.516 ms |
| Weighted | boolean (delayed) | 20 | 5 | 120.771 ms | [120.624, 121.385] ms | 120.629 ms | 121.361 ms |
| Weighted | projective (delayed) | 20 | 5 | 112.803 ms | [112.600, 113.401] ms | 112.606 ms | 113.391 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 92.724 ms | [92.582, 92.749] ms | 92.583 ms | 92.748 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **78.698 ms** | [78.651, 79.272] ms | 78.651 ms | 79.253 ms |

## BN254_upper — claim-unaware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 134.585 ms | [134.546, 135.064] ms | 134.547 ms | 135.051 ms |
| Unweighted | projective | 20 | 5 | 130.136 ms | [130.070, 130.649] ms | 130.072 ms | 130.635 ms |
| Unweighted | flipped | 20 | 5 | 127.320 ms | [127.310, 128.009] ms | 127.310 ms | 127.989 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 88.368 ms | [88.324, 88.771] ms | 88.324 ms | 88.760 ms |
| Unweighted | projective (delayed) | 20 | 5 | **79.536 ms** | [79.515, 79.940] ms | 79.515 ms | 79.928 ms |
| Unweighted | flipped (delayed) | 20 | 5 | 80.672 ms | [80.667, 81.160] ms | 80.667 ms | 81.145 ms |
| Weighted | boolean | 20 | 5 | 178.587 ms | [178.488, 179.279] ms | 178.489 ms | 179.261 ms |
| Weighted | projective | 20 | 5 | 166.789 ms | [166.451, 167.434] ms | 166.463 ms | 167.408 ms |
| Weighted | projective-different-claims | 20 | 5 | 150.925 ms | [150.718, 152.396] ms | 150.719 ms | 152.386 ms |
| Weighted | flipped-different-claims | 20 | 5 | 149.245 ms | [148.991, 150.409] ms | 148.993 ms | 150.363 ms |
| Weighted | boolean (delayed) | 20 | 5 | 131.204 ms | [131.043, 131.799] ms | 131.044 ms | 131.777 ms |
| Weighted | projective (delayed) | 20 | 5 | 122.560 ms | [122.347, 134.498] ms | 122.355 ms | 134.064 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | **103.209 ms** | [103.053, 106.027] ms | 103.059 ms | 105.964 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | 107.390 ms | [107.130, 108.919] ms | 107.139 ms | 108.898 ms |

## Fp128 — claim-aware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 25.287 ms | [25.243, 25.327] ms | 25.244 ms | 25.326 ms |
| Unweighted | projective | 20 | 5 | 24.165 ms | [24.124, 24.211] ms | 24.125 ms | 24.210 ms |
| Unweighted | projective_1inf | 20 | 5 | 22.988 ms | [22.933, 23.016] ms | 22.934 ms | 23.016 ms |
| Unweighted | flipped | 20 | 5 | 20.798 ms | [20.785, 20.844] ms | 20.785 ms | 20.843 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 20.479 ms | [20.454, 20.539] ms | 20.454 ms | 20.538 ms |
| Unweighted | projective (delayed) | 20 | 5 | 18.451 ms | [18.422, 18.490] ms | 18.423 ms | 18.489 ms |
| Unweighted | projective_1inf (delayed) | 20 | 5 | 17.500 ms | [17.482, 17.536] ms | 17.482 ms | 17.535 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **15.797 ms** | [15.765, 15.829] ms | 15.765 ms | 15.828 ms |
| Weighted | boolean | 20 | 5 | 37.076 ms | [37.018, 37.085] ms | 37.018 ms | 37.084 ms |
| Weighted | projective | 20 | 5 | 34.959 ms | [34.935, 34.996] ms | 34.936 ms | 34.996 ms |
| Weighted | projective_1inf | 20 | 5 | 34.527 ms | [34.518, 34.562] ms | 34.518 ms | 34.562 ms |
| Weighted | projective-different-claims | 20 | 5 | 30.019 ms | [29.987, 30.036] ms | 29.987 ms | 30.036 ms |
| Weighted | flipped-different-claims | 20 | 5 | 26.630 ms | [26.576, 26.742] ms | 26.577 ms | 26.737 ms |
| Weighted | boolean (delayed) | 20 | 5 | 30.265 ms | [30.219, 30.402] ms | 30.220 ms | 30.397 ms |
| Weighted | projective (delayed) | 20 | 5 | 27.748 ms | [27.725, 27.788] ms | 27.726 ms | 27.787 ms |
| Weighted | projective_1inf (delayed) | 20 | 5 | 27.302 ms | [27.269, 27.354] ms | 27.270 ms | 27.352 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 23.578 ms | [23.521, 23.671] ms | 23.522 ms | 23.669 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **21.085 ms** | [21.074, 21.117] ms | 21.074 ms | 21.117 ms |

## Fp128 — claim-unaware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 30.526 ms | [30.480, 30.587] ms | 30.482 ms | 30.587 ms |
| Unweighted | projective | 20 | 5 | 29.357 ms | [29.317, 29.415] ms | 29.318 ms | 29.415 ms |
| Unweighted | projective_1inf | 20 | 5 | 28.687 ms | [28.683, 28.741] ms | 28.683 ms | 28.741 ms |
| Unweighted | flipped | 20 | 5 | 29.822 ms | [29.774, 29.887] ms | 29.776 ms | 29.887 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 23.186 ms | [23.170, 23.219] ms | 23.170 ms | 23.219 ms |
| Unweighted | projective (delayed) | 20 | 5 | 21.247 ms | [21.231, 21.310] ms | 21.231 ms | 21.310 ms |
| Unweighted | projective_1inf (delayed) | 20 | 5 | **19.678 ms** | [19.630, 19.703] ms | 19.631 ms | 19.702 ms |
| Unweighted | flipped (delayed) | 20 | 5 | 21.290 ms | [21.255, 21.404] ms | 21.257 ms | 21.401 ms |
| Weighted | boolean | 20 | 5 | 40.833 ms | [40.768, 40.863] ms | 40.769 ms | 40.863 ms |
| Weighted | projective | 20 | 5 | 39.337 ms | [39.273, 39.481] ms | 39.273 ms | 39.476 ms |
| Weighted | projective_1inf | 20 | 5 | 38.673 ms | [38.653, 38.723] ms | 38.653 ms | 38.723 ms |
| Weighted | projective-different-claims | 20 | 5 | 35.206 ms | [35.175, 35.248] ms | 35.176 ms | 35.248 ms |
| Weighted | flipped-different-claims | 20 | 5 | 35.090 ms | [35.039, 35.109] ms | 35.039 ms | 35.109 ms |
| Weighted | boolean (delayed) | 20 | 5 | 33.693 ms | [33.667, 33.720] ms | 33.668 ms | 33.720 ms |
| Weighted | projective (delayed) | 20 | 5 | 30.882 ms | [30.828, 30.918] ms | 30.829 ms | 30.917 ms |
| Weighted | projective_1inf (delayed) | 20 | 5 | 30.111 ms | [30.086, 30.145] ms | 30.086 ms | 30.145 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 26.179 ms | [26.143, 26.207] ms | 26.145 ms | 26.207 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **26.037 ms** | [25.980, 26.074] ms | 25.980 ms | 26.073 ms |

## GF128 — claim-aware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 7.586 ms | [7.578, 7.596] ms | 7.578 ms | 7.595 ms |
| Unweighted | projective | 20 | 5 | 7.477 ms | [7.470, 7.526] ms | 7.470 ms | 7.524 ms |
| Unweighted | flipped | 20 | 5 | 7.370 ms | [7.366, 7.384] ms | 7.366 ms | 7.383 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 6.671 ms | [6.629, 6.746] ms | 6.629 ms | 6.743 ms |
| Unweighted | projective (delayed) | 20 | 5 | 6.513 ms | [6.509, 6.526] ms | 6.509 ms | 6.526 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **5.876 ms** | [5.864, 5.932] ms | 5.865 ms | 5.930 ms |
| Weighted | boolean | 20 | 5 | 10.456 ms | [10.433, 10.468] ms | 10.434 ms | 10.468 ms |
| Weighted | projective | 20 | 5 | 10.347 ms | [10.337, 10.413] ms | 10.337 ms | 10.411 ms |
| Weighted | projective-different-claims | 20 | 5 | 8.942 ms | [8.927, 8.943] ms | 8.927 ms | 8.943 ms |
| Weighted | flipped-different-claims | 20 | 5 | 8.795 ms | [8.784, 8.823] ms | 8.784 ms | 8.822 ms |
| Weighted | boolean (delayed) | 20 | 5 | 9.158 ms | [9.151, 9.166] ms | 9.151 ms | 9.166 ms |
| Weighted | projective (delayed) | 20 | 5 | 9.048 ms | [9.042, 9.058] ms | 9.042 ms | 9.057 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 7.735 ms | [7.725, 7.743] ms | 7.725 ms | 7.742 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **7.340 ms** | [7.330, 7.343] ms | 7.330 ms | 7.343 ms |

## GF128 — claim-unaware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 8.905 ms | [8.899, 8.970] ms | 8.899 ms | 8.967 ms |
| Unweighted | projective | 20 | 5 | 8.788 ms | [8.784, 8.805] ms | 8.784 ms | 8.805 ms |
| Unweighted | flipped | 20 | 5 | 8.818 ms | [8.816, 8.838] ms | 8.816 ms | 8.837 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 7.180 ms | [7.172, 7.193] ms | 7.172 ms | 7.193 ms |
| Unweighted | projective (delayed) | 20 | 5 | 7.072 ms | [7.060, 7.075] ms | 7.060 ms | 7.075 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **7.069 ms** | [7.057, 7.075] ms | 7.057 ms | 7.075 ms |
| Weighted | boolean | 20 | 5 | 12.551 ms | [12.546, 12.567] ms | 12.546 ms | 12.567 ms |
| Weighted | projective | 20 | 5 | 12.766 ms | [12.758, 12.790] ms | 12.758 ms | 12.789 ms |
| Weighted | projective-different-claims | 20 | 5 | 10.459 ms | [10.443, 10.463] ms | 10.444 ms | 10.463 ms |
| Weighted | flipped-different-claims | 20 | 5 | 10.533 ms | [10.517, 10.541] ms | 10.518 ms | 10.541 ms |
| Weighted | boolean (delayed) | 20 | 5 | 10.469 ms | [10.459, 10.477] ms | 10.459 ms | 10.477 ms |
| Weighted | projective (delayed) | 20 | 5 | 10.214 ms | [10.206, 10.222] ms | 10.206 ms | 10.222 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 8.665 ms | [8.653, 8.672] ms | 8.653 ms | 8.672 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **8.580 ms** | [8.576, 8.587] ms | 8.576 ms | 8.587 ms |

## KB5 — claim-aware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 33.299 ms | [33.274, 33.335] ms | 33.275 ms | 33.335 ms |
| Unweighted | projective | 20 | 5 | 28.885 ms | [28.858, 28.911] ms | 28.858 ms | 28.911 ms |
| Unweighted | flipped | 20 | 5 | 27.544 ms | [27.508, 27.609] ms | 27.509 ms | 27.608 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 29.233 ms | [29.207, 29.258] ms | 29.208 ms | 29.258 ms |
| Unweighted | projective (delayed) | 20 | 5 | 27.173 ms | [27.141, 27.181] ms | 27.142 ms | 27.181 ms |
| Unweighted | flipped (delayed) | 20 | 5 | **25.294 ms** | [25.265, 25.307] ms | 25.265 ms | 25.307 ms |
| Weighted | boolean | 20 | 5 | 44.070 ms | [44.043, 44.098] ms | 44.043 ms | 44.098 ms |
| Weighted | projective | 20 | 5 | 41.099 ms | [41.085, 41.153] ms | 41.085 ms | 41.152 ms |
| Weighted | projective-different-claims | 20 | 5 | 37.056 ms | [37.043, 37.102] ms | 37.043 ms | 37.102 ms |
| Weighted | flipped-different-claims | 20 | 5 | 34.862 ms | [34.836, 34.897] ms | 34.836 ms | 34.897 ms |
| Weighted | boolean (delayed) | 20 | 5 | 44.449 ms | [44.437, 44.520] ms | 44.437 ms | 44.520 ms |
| Weighted | projective (delayed) | 20 | 5 | 43.373 ms | [43.359, 43.426] ms | 43.359 ms | 43.426 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | 35.008 ms | [34.973, 35.032] ms | 34.974 ms | 35.032 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | **31.675 ms** | [31.655, 31.696] ms | 31.655 ms | 31.696 ms |

## KB5 — claim-unaware

| Scope | Kernel | n | Processes | Median | 99% CI | p01 | p99 |
|---|---|---:|---:|---:|---:|---:|---:|
| Unweighted | boolean | 20 | 5 | 40.341 ms | [40.285, 40.416] ms | 40.286 ms | 40.413 ms |
| Unweighted | projective | 20 | 5 | 38.924 ms | [38.864, 38.949] ms | 38.864 ms | 38.948 ms |
| Unweighted | flipped | 20 | 5 | **34.611 ms** | [34.588, 34.643] ms | 34.589 ms | 34.643 ms |
| Unweighted | boolean (delayed) | 20 | 5 | 36.780 ms | [36.752, 36.927] ms | 36.753 ms | 36.921 ms |
| Unweighted | projective (delayed) | 20 | 5 | 35.294 ms | [35.274, 35.335] ms | 35.274 ms | 35.334 ms |
| Unweighted | flipped (delayed) | 20 | 5 | 35.181 ms | [35.161, 35.267] ms | 35.161 ms | 35.265 ms |
| Weighted | boolean | 20 | 5 | 52.575 ms | [52.525, 52.986] ms | 52.525 ms | 52.970 ms |
| Weighted | projective | 20 | 5 | 46.700 ms | [46.626, 47.074] ms | 46.627 ms | 47.060 ms |
| Weighted | projective-different-claims | 20 | 5 | 41.941 ms | [41.898, 41.976] ms | 41.898 ms | 41.976 ms |
| Weighted | flipped-different-claims | 20 | 5 | 41.948 ms | [41.907, 41.976] ms | 41.907 ms | 41.976 ms |
| Weighted | boolean (delayed) | 20 | 5 | 49.494 ms | [49.449, 49.507] ms | 49.450 ms | 49.507 ms |
| Weighted | projective (delayed) | 20 | 5 | 49.016 ms | [48.979, 49.036] ms | 48.979 ms | 49.035 ms |
| Weighted | projective-different-claims (delayed) | 20 | 5 | **41.610 ms** | [41.585, 41.617] ms | 41.585 ms | 41.617 ms |
| Weighted | flipped-different-claims (delayed) | 20 | 5 | 42.999 ms | [42.964, 43.035] ms | 42.965 ms | 43.034 ms |

