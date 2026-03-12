# nepal-phone

<!--
[![Crates.io](https://img.shields.io/crates/v/nepal-phone.svg)](https://crates.io/crates/nepal-phone)
[![CI status](https://github.com/your-org/nepal-phone/actions/workflows/rust.yml/badge.svg)](https://github.com/your-org/nepal-phone/actions)
[![Downloads](https://img.shields.io/crates/d/nepal-phone.svg)](https://crates.io/crates/nepal-phone)
[![docs.rs](https://docs.rs/nepal-phone/badge.svg)](https://docs.rs/nepal-phone) -->

`nepal-phone` is a Rust crate for parsing and validating Nepali phone numbers.

It detects whether a number is **mobile** or **landline**, identifies the mobile **operator**, and resolves the geographic **district / area** from the STD area code — covering all 77 districts across Nepal's 7 provinces.

## Example

```rust
use nepal_phone::parse;

let p = parse("9841234567").unwrap();
println!("{}", p);
// Mobile(9841234567, Nepal Telecom)

let map = p.to_map();
// {"type": "mobile", "number": "9841234567", "operator": "Nepal Telecom"}

let p = parse("+977-142314819").unwrap();
println!("{}", p);
// Landline(0142314819, code=01, areas=[Kathmandu, Lalitpur, Bhaktapur])
```

## Requirements

    Rust >= 1.70

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nepal-phone = "0.1"
```

## Features

1. [Phone Number](#phone-number)
   - [is_valid](#is_valid)
   - [is_mobile_number](#is_mobile_number)
   - [is_landline_number](#is_landline_number)
   - [parse](#parse)
   - [to_map](#to_map)
2. [Operators](#operators)
3. [Areas](#areas)

---

## Phone Number

```rust
use nepal_phone::{parse, is_valid, is_mobile_number, is_landline_number};
```

### is_valid

Checks whether the given string is a valid Nepali phone number of any kind.

```rust
nepal_phone::is_valid("9851377890");       // true
nepal_phone::is_valid("+977-142314819");   // true

nepal_phone::is_valid("8251377890");       // false
nepal_phone::is_valid("abc");              // false
```

### is_mobile_number

```rust
nepal_phone::is_mobile_number("9841234567");       // true
nepal_phone::is_mobile_number("+977-9842536789");  // true
nepal_phone::is_mobile_number("9779841234567");    // true

nepal_phone::is_mobile_number("0142314819");       // false
```

### is_landline_number

```rust
nepal_phone::is_landline_number("0142314819");     // true
nepal_phone::is_landline_number("+977-142314819"); // true

nepal_phone::is_landline_number("9841234567");     // false
```

### parse

Parses a phone number and returns a [`PhoneNumber`] on success, or `None` if the number is invalid or unrecognised.
Numbers with a `+977` / `977` country-code prefix and hyphens are handled automatically.

```rust
use nepal_phone::parse;

parse("9851377890");
// Some(Mobile { number: "9851377890", operator: NepalTelecom })

parse("+977-142314819");
// Some(Landline { number: "0142314819", area_code: "01", areas: [Kathmandu, Lalitpur, Bhaktapur] })

parse("abc");   // None
parse("12345"); // None
parse("");      // None
```

### to_map

Returns the parsed number as a `HashMap<&str, String>` — useful for serialisation or passing data across API boundaries.

**Mobile keys:** `type`, `number`, `operator`

```rust
let map = nepal_phone::parse("9851377890").unwrap().to_map();
// {
//   "type":     "mobile",
//   "number":   "9851377890",
//   "operator": "Nepal Telecom"
// }
```

**Landline keys:** `type`, `number`, `area_code`, `areas` (comma-separated)

```rust
let map = nepal_phone::parse("+977-142314819").unwrap().to_map();
// {
//   "type":      "landline",
//   "number":    "0142314819",
//   "area_code": "01",
//   "areas":     "Kathmandu, Lalitpur, Bhaktapur"
// }
```

---

## Operators

Mobile numbers are matched to one of five operators based on their prefix:

| Operator      | `Operator` variant       | Prefixes                |
| ------------- | ------------------------ | ----------------------- |
| Nepal Telecom | `Operator::NepalTelecom` | 984, 985, 986, 974, 975 |
| Ncell         | `Operator::Ncell`        | 980, 981, 982           |
| Smart Cell    | `Operator::SmartCell`    | 961, 962, 988           |
| UTL           | `Operator::Utl`          | 972                     |
| Hello Mobile  | `Operator::HelloMobile`  | 963                     |

`Operator` implements `Display`:

```rust
println!("{}", nepal_phone::Operator::NepalTelecom); // "Nepal Telecom"
println!("{}", nepal_phone::Operator::Ncell);        // "Ncell"
```

---

## Areas

Provides STD area-code-to-district mapping for all 77 districts across Nepal's 7 provinces.

```rust
use nepal_phone::{AREAS, areas_by_code, Area};
```

**Look up districts by area code**

```rust
nepal_phone::areas_by_code("01");
// [Bhaktapur (01), Kathmandu (01), Lalitpur (01)]

nepal_phone::areas_by_code("061");
// [Kaski (061)]

nepal_phone::areas_by_code("000"); // [] — unknown code
```

**Iterate all districts**

```rust
for area in nepal_phone::AREAS {
    println!("{area}"); // e.g. "Kathmandu (01)", "Kaski (061)", ...
}
```

`Area` implements `Display` as `"<name> (<area_code>)"`:

```rust
let a = nepal_phone::Area { name: "Pokhara Metro", area_code: "061" };
println!("{a}"); // "Pokhara Metro (061)"
```

---

## Contribution

Feedback and contributions are welcome. Please open an issue or pull request on GitHub.
