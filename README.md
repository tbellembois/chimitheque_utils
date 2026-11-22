[![Rust](https://github.com/tbellembois/chimitheque_utils/actions/workflows/rust.yml/badge.svg)](https://github.com/tbellembois/chimitheque_utils/actions/workflows/rust.yml) [![codecov](https://codecov.io/github/tbellembois/chimitheque_utils/graph/badge.svg?token=TC1CP3EVKO)](https://codecov.io/github/tbellembois/chimitheque_utils)

Utilities of [Chimithèque](https://github.com/tbellembois/chimitheque_back), a management application for chemical products, biological reagents and laboratory consumables.

- `casnumber`, `cenumber`: CAS registry and European Community number checks, including the check digit.
- `formula`: empirical formula of a formula, in the Hill order, e.g. `C2H6O` for `CH3CH2OH` or `CuH10O9S` for `CuSO4.5H2O`.
- `inchi`: `InChI` and `InChIKey` format checks.
- `smiles`: SMILES syntax check, following the OpenSMILES grammar.
- `string`: string cleaning (whitespaces and case).

## Casbin matcher

`examples/casbin.rs` prints the casbin matcher of the permissions, to copy into the `[matchers]` section of `chimitheque_back/src/casbin/policy.conf`:

```sh
cargo run --example casbin
```

Make changes in `examples/casbin.xlsx` too.
