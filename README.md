# `cargo feature-set`

Extract the features for every compiled crate from the build plan.

Requires cargo nigthly.

It looks like this:

```
Crate                    Target Kind  Features
=====                    ========
serde:1.0.99             custom-build  default, derive, serde_derive, std
proc-macro2:1.0.2        custom-build  default, proc-macro
unicode-xid:0.2.0        lib           default
proc-macro2:1.0.2        lib           default, proc-macro
quote:1.0.2              lib           default, proc-macro
syn:1.0.5                custom-build  clone-impls, default, derive, parsing, printing, proc-macro, quote, visit
syn:1.0.5                lib           clone-impls, default, derive, parsing, printing, proc-macro, quote, visit
serde_derive:1.0.99      proc-macro    default
serde:1.0.99             lib           default, derive, serde_derive, std
itoa:0.4.4               lib           default, std
ryu:1.0.0                custom-build  
ryu:1.0.0                lib           
serde_json:1.0.40        lib           default
unicode-width:0.1.6      lib           default
tabwriter:1.1.0          lib           default
cargo-feature-set:0.1.0  bin           
```

## License

[MIT](LICENSE).
