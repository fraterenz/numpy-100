# Numpy

Using array to index returns a copy and not a view like with slicing.

# Implications of using Rust

- Cannot create arbitrary shaped arrays, so for instance if your function returns an dnarray (e.g. `ex9`), then you must know in advance its shape. I think it should be fine, and even better than python, because when you optimize you must know the dimensionality of your data and parameters. `ArrayBase` cannot grow or shrink. But I think you could use `ArrayD`, but not sure.

- Less packages, but found an automatic differentiation pkg available [here](https://github.com/raskr/rust-autograd)

- index arrays

- mask index arrays
