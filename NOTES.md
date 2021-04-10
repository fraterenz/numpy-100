# Guidelines

**create new arrays:** always specify the data type. Try to use always specialized methods if possible (`np.zeros` or `Array::zeros`), but you can also first create an array with ones and the change some values.

**handle floating points correctly:** rust cares a lot about this, try to use some operations that are specific to float whenever possible [here](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.mul_add).

**try to avoid useless intermediate variables:** intermediate variables are usually not required and can be removed to avoid wasting memory (`np.add(a, b, out=a)` and `dy + &y`), such that `a` and `dy` are consumed and updated with the result of the addition. 

# Numpy

Using array to index returns a copy and not a view like with slicing.

## Memory layout
To create new array you need the shape and the data type, 

An instance of class `ndarray` consists of a contiguous one-dimensional segment of computer memory (owned by the array, or by some other object), combined with an indexing scheme that maps `N` integers into the location of an item in the block. An array is mostly a contiguous block of memory whose parts can be accessed using an indexing scheme. Such indexing scheme is in turn defined by a shape and a data type. How many bytes each item takes and how the bytes are interpreted is defined by the data-type object associated with the array.

**data type:** an instance of the data type class `numpy.dtype` describes several things
- type of data (e.g. integer, float)
- number of bytes needed to store an instance of a type of data `numpy.dtype.itemsize`
- byte order that is little-endian or big-endian
- fields in case of structured data types (similar to pandas)

**shape:** specifies the indexing scheme to access the memory owned by an array. Since a block of memory is 1 dimensional, need to define a scheme to create arrays of many dimensions, this scheme is the shape. Think of indexing as retrieving data with pointers according to the scheme defined by `shape` of the array.

**strided indexing scheme:** defines how to relate the integers used as index by the user to the memory owned by the array. This scheme defines an offset (i.e. the mapping between the index and the piece of memory used by numpy to retrieve data) which is the product of the k stride times the index, see [numpy official](https://numpy.org/doc/stable/reference/arrays.ndarray.html#internal-memory-layout-of-an-ndarray), [ipython strides](https://ipython-books.github.io/46-using-stride-tricks-with-numpy/) and [from python to numpy](https://www.labri.fr/perso/nrougier/from-python-to-numpy/#anatomy-of-an-array). Strides describe how the items of a multidimensional array are organized in the data buffer, that is how many bytes we need to jump over in the data buffer to go from one item to the next. There are two strides, one for the columns and one for the rows. They are defined as the product of `numpy.dtype.itemsize` times `a.shape[0]` for the row, and `numpy.dtype.itemsize` for the column (if the array is in the row-major order aka C order, else you need to transpose that).

## Copying vs slicing
In python every thing is an object and when you assign variables you just bound a name to an object. When objects are mutable (such as arrays in numpy or lists), the assignment is a shallow copy, and thus if you modify one variable it will change also the other variable. To enforce deep copy in numpy you can either 1. use functions that returns copies (`flatten` instead of `ravel`) 2. use fancy indexing see [here](https://www.labri.fr/perso/nrougier/from-python-to-numpy/#direct-and-indirect-access). 

# Implications of using Rust

- Cannot create arbitrary shaped arrays, so for instance if your function returns an dnarray (e.g. `ex9`), then you must know in advance its shape. I think it should be fine, and even better than python, because when you optimize you must know the dimensionality of your data and parameters. `ArrayBase` cannot grow or shrink. But I think you could use `ArrayD`, but not sure.

- Less packages, but found an automatic differentiation pkg available [here](https://github.com/raskr/rust-autograd)

- index arrays

- mask index arrays
