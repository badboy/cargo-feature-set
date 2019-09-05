# `cargo feature-set`

Extract the features for every compiled crate from the build plan.

Requires cargo nigthly.

It looks like this:

```
Crate              Features
=====              ========
serde              default, derive, serde_derive, std
proc-macro2        default, proc-macro
unicode-xid        default
proc-macro2        default, proc-macro
quote              default, proc-macro
syn                clone-impls, default, derive, parsing, printing, proc-macro, quote, visit
syn                clone-impls, default, derive, parsing, printing, proc-macro, quote, visit
serde_derive       default
serde              default, derive, serde_derive, std
itoa               default, std
ryu                
ryu                
serde_json         default
unicode-width      default
tabwriter          default
cargo-feature-set  
```

## License

[MIT](LICENSE).
