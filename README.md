## md5inrust
This is a simple implementation of the md5 message digest algorithm
(RFC 1321) in the Rust programming language. I wrote it while learning
Rust.

To try it out, first clone the repo

```bash
git clone https://github.com/oscar-franzen/md5inrust
```

Enter the directory and compile the code
```bash
cd md5inrust

# uses Rust's cargo to compile the source file
cargo -r --release
```

### How does the speed compare with `md5sum`?
`md5sum` is a utility shipped with most Linux distributions (part of
coreutils), and it is very fast. I will here compare the execution speed:


```bash
# from coreutils
time md5sum sratoolkit.2.9.2-ubuntu64.tar.gz
```

The output was:

```bash
8234f444f449c403e08d186fc56139c2  sratoolkit.2.9.2-ubuntu64.tar.gz
md5sum sratoolkit.2.9.2-ubuntu64.tar.gz  0.14s user 0.02s system 99% cpu 0.160 total

```

Trying out md5inrust:

```bash
time ./target/release/md5inrust sratoolkit.2.9.2-ubuntu64.tar.gz
Loading sratoolkit.2.9.2-ubuntu64.tar.gz...
8234f444f449c403e08d186fc56139c2
./target/release/md5inrust sratoolkit.2.9.2-ubuntu64.tar.gz  0.20s user 0.01s system 99% cpu 0.205 total

```

### Contact
- OF, <p.oscar.franzen@gmail.com>

### References
- RFC 1321, https://www.rfc-editor.org/rfc/rfc1321.txt)
