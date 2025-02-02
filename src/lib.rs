//! `peroxide` is comprehensive numerical library for Rust.
//!
//! ## Components
//!
//! `peroxide` has various components for scientific computation.
//!
//! * Linear Algebra (with BLAS & LAPACK)
//!     * [Matrix](structure/matrix/index.html) operations
//!         * `+,-,*,/`
//!         * LU Decomposition, Determinant, Inverse
//!         * QR Decomposition (`O3` feature needed)
//!         * Singular Value Decomposition (`O3` feature needed)
//!         * Cholesky Decomposition (`O3` feature needed)
//!         * Reduced Row Echelon Form
//!     * [Vector](structure/vector/index.html) operations
//!     * [Eigenvalue, Eigenvector](numerical/eigen/index.html) algorithms
//! * Statistics
//!     * [Statistical operations](statistics/stat/index.html)
//!         * `mean, var, sd`
//!         * `factorial, P, C, H`
//!     * [Distributions](statistics/dist/index.html)
//!         * Bernoulli
//!         * Uniform
//!         * Binomial
//!         * Normal
//!         * Gamma
//!         * Beta
//!         * Student's t
//! * [Special functions](special/function/index.html) (Using `puruspe` crate)
//!     * Gaussian
//!     * Gamma
//!     * Beta
//!     * Error
//!     * Incomplete Gamma
//!     * Incomplete Beta
//! * Automatic Differentiation
//!     * [Taylor mode forward AD](structure/ad/index.html)
//! * Numerical Utils
//!     * [Interpolation](numerical/interp/index.html)
//!     * [Spline](numerical/spline/index.html)
//!     * [Polynomial](structure/polynomial/index.html)
//!     * [Lanczos Approximation](special/lanczos/index.html)
//!     * [Numerical Integrations](numerical/integral/index.html)
//! * [Optimization](numerical/optimize/index.html)
//!     * Gradient Descent
//!     * Levenberg-Marquardt
//! * [Root Finding](numerical/root/index.html)
//!     * Bisection
//!     * False Position (Regula falsi)
//!     * Secant
//!     * Newton
//! * [Differential Equations](numerical/ode/index.html)
//!     * Explicit
//!         * Runge-Kutta 4th order
//!         * Euler methods
//!     * Implicit
//!         * Backward Euler
//!         * Gauss-Legendre 4th order
//! * Communication with Python
//!     * [Plot with `matplotlib`](util/plot/index.html)
//! * [DataFrame](structure/dataframe/index.html)
//!     * Read & Write with `parquet` or `netcdf` or `csv` format
//! * Macros
//!     * [R macros](macros/r_macro/index.html)
//!     * [Matlab macros](macros/matlab_macro/index.html)
//!     * [Julia macros](macros/julia_macro/index.html)
//!
//! And all these things are built on mathematical traits.
//!
//! * Traits
//!     * [Functional Programming tools](traits/fp/index.html)
//!     * [General algorithms](traits/general/index.html)
//!     * [Mathematics](traits/math/index.html)
//!     * [Mutable tools](traits/mutable/index.html)
//!     * [Number & Real](traits/num/index.html)
//!     * [Pointer](traits/pointer/index.html)
//!     * [Stable](traits/stable/index.html)
//!
//! ## Quick Start
//!
//! ### Cargo.toml
//!
//! * Run below commands in your project directory
//!
//! 1. Default
//!     ```bash
//!     cargo add peroxide
//!     ```
//! 2. OpenBLAS
//!     ```bash
//!     cargo add peroxide --features O3
//!     ```
//! 3. Plot
//!     ```bash
//!     cargo add peroxide --features plot
//!     ```
//! 4. NetCDF dependency for DataFrame
//!     ```bash
//!     cargo add peroxide --features nc
//!     ```
//! 5. CSV dependency for DataFrame
//!     ```bash
//!     cargo add peroxide --features csv
//!     ```
//! 6. Parquet dependency for DataFrame
//!     ```bash
//!     cargo add peroxide --features parquet
//!     ```
//! 7. All features
//!     ```bash
//!     cargo add peroxide --features "O3 plot nc csv parquet"
//!     ```
//!
//! ## Import all at once
//!
//! Peroxide has two options.
//!
//! * [`prelude`](prelude/index.html) : To simple use
//! * [`fuga`](fuga/index.html) : To control numerical algorithms
//!
//! To see differences, follow above two links.
//!
//! You can import all functions & structures at once
//!
//! * `prelude`
//! ```
//! #[macro_use]
//! extern crate peroxide;
//! use peroxide::prelude::*;
//!
//! fn main() {
//!     // Write what you want
//! }
//! ```
//!
//! * `fuga`
//! ```
//! #[macro_use]
//! extern crate peroxide;
//! use peroxide::fuga::*;
//!
//! fn main() {
//!     // Write what you want
//! }
//! ```
//!
//! ## Useful tips for features
//!
//! * After `0.28.0`, `dataframe` feature is replaced by `nc` feature.
//! * If you want to use *QR* or *SVD* or *Cholesky Decomposition* then should use `O3` feature (there are no implementations for these decompositions in `default`)
//! * If you want to write your numerical results, then use `parquet` or `nc` features (corresponding to `parquet` or `netcdf` format. (It is much more effective than `csv` and `json`.)
//! * After `0.23.0`, there are two options - `fuga`, `prelude`. Choose proper option for your computations.
//! * To plot, use `parquet` or `nc` feature to export data as parquet or netcdf format and use python to draw plot.
//!     * `plot` feature has limited plot abilities.
//!     * To read parquet file in python, use `pandas` & `pyarrow` libraries.
//!     * There is a template of python code for netcdf. - [Socialst](https://github.com/Axect/Socialst/blob/master/Templates/PyPlot_Template/nc_plot.py)

#[cfg(feature = "O3")]
extern crate blas;

#[cfg(feature = "O3")]
extern crate lapack;

#[cfg(feature = "plot")]
extern crate pyo3;

#[cfg(feature = "serde")]
extern crate serde;

extern crate rand;

// extern crate json;

extern crate order_stat;

extern crate puruspe;

extern crate matrixmultiply;

#[cfg(feature = "nc")]
extern crate netcdf;

extern crate peroxide_ad;

#[macro_use]
pub mod macros;

pub mod fuga;
pub mod ml;
pub mod numerical;
pub mod prelude;
pub mod special;
pub mod statistics;
pub mod structure;
pub mod traits;
pub mod util;
