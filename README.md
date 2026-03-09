### Welcome
<p align="center">
<a href="https://github.com/r3ap3rpy/algorithms/actions?query=workflow%3ABuild+event%3Apush+branch%3Amain" target="_blank">
    <img src="https://github.com/r3ap3rpy/algorithms/actions/workflows/build.yml/badge.svg?event=push&branch=main" alt="Build">
</a>
    <a href="https://github.com/r3ap3rpy/algorithms/actions?query=workflow%3ATest+event%3Aworkflow_run+branch%3Amain" target="_blank">
    <img src="https://github.com/r3ap3rpy/algorithms/actions/workflows/test.yml/badge.svg?branch=main" alt="Test">
</a>
        <a href="https://github.com/r3ap3rpy/algorithms/actions?query=workflow%3APublish+event%3Aworkflow_run+branch%3Amain" target="_blank">
    <img src="https://github.com/r3ap3rpy/algorithms/actions/workflows/publish.yml/badge.svg?branch=main" alt="Publish">
</a>
</p>

I found this neat little python modul called [algorithms](https://pypi.org/project/algorithms) and thought I would rewrite it in Rust.


The official github repository can be found here at [algorithms](https://github.com/r3ap3rpy/algorithms.git).

If you want to install it directly from here use the following commands.

``` bash
git clone https://github.com/r3ap3rpy/algorithms.git
cd algorithms
cargo test
cargo install --path .
```

If you want to use it in your project use the following commands. It is published on [crates.io](https://crates.io/crates/algorithmz)

``` bash
cargo new <myproject>
cd myproject
cargo add algorithmz
```

##### Sorting

The following algorithms are implemented:
- bubble sort
- bead sort 
- heap sort
- counting sort 
- cycle sort
- shell sort
- quick sort
- bitonic sort
- bogo sort
- bucket sort
- cocktail shaker sort
- comb sort
- exchange sort
- gnome sort
- heap sort
- meeting rooms sort
- merge sort
- pancake sort
- pigeonhole sort
- radix sort
- selection sort
- color sort
- stooge sort
- wiggle sort

##### Arrays

The following algorithms are implemented:
- flatten
- delete nth
- delete nth naive
- garage
- Josephus
- limit 
- longest non repeat
- max ones index
- missing ranges
- move zeros

##### Set

The following is implemented:
- find keyboard row

> [!WARNING]
> This is still in progress.

