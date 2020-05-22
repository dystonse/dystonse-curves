# dystonse-curves
**This repository is a part of the multi-repository project `dystonse`. See the [main repository](https://github.com/dystonse/dystonse) for more information.**

A rust crate for storage and manipulation of probability curves with the following characteristics:

 * represents cumulative probabilities, therefore each curve starts with Y=0 and ends with Y=1 and is increasing montonously
 * the curve is approximated by a finite number of points. Values in between are computed by linear interpolation
 * the types for X and Y values can be chosen by generic type parameters

These curves will be used for many different purposes within [dystonse-gtfs-data](https://github.com/dystonse/dystonse-gtfs-data) and [dystonse-search-rust](https://github.com/dystonse/dystonse-search-rust).