

```shell
$ cargo build --release
$ time target/release/mandelbrot_rayon mandel.png 4000x3000 -1.20,0.35 -1,0.20
target/release/mandelbrot_rayon mandel.png 4000x3000 -1.20,0.35 -1,0.20  2.85s user 0.12s system 1749% cpu 0.170 total

```