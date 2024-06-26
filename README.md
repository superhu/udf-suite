# udf-suite

A collection of UDFs for MariaDB & MySQL, written using the rust [`udf`]
library. For instructions on how to use these libraries, jump to the
[Installation](#installation) section.

New function contributions are welcome!

[`udf`]: http://docs.rs/udf

## Included UDFs

The following UDFs are includes:

- [UUIDs](#uuid): generate and convert v1, v2, v6, and v7 UUIDs
- [Hash Algorithms](#hash-algorithms): run a wide variety of hash algorithms,
  including the following families: `blake`, `sha`, `keccak`, `sha3`, and
  `xxhash`
- [IP Functions](#ip-address-functions) for interop: `ip_validate`,
  `ip_to_canonical`, `ip_to_ipv4_mapped`
- [String Operations](#string-operations): Calculations such as Levenshtein
  edit distance, including limited and normalized versions.
- [Jsonify](#jsonify): convert any data to JSON
- [Lipsum](#lipsum): generate random text

See the relevant section for more information.

### UUID

Provide UUID functions similar to the Postges [`uuid-osp`] package:

- Generate v1 and v4 UUIDs (v3 & v5 coming soon)
- Generate the new v6 and v7 UUIDs
- Validate UUIDs
- Create namespace UUIDs
- `uuid_to_bin` and `uuid_from_bin`/`bin_to_uuid` functions, including bit
  rearranging options

See the [UUID Readme](/udf-uuid/README.md) for more information

#### Usage

```text
note: type uuid is type string
uuid_generate_v1() -> uuid
uuid_generate_v1mc() -> uuid
uuid_generate_v4() -> uuid
uuid_generate_v6([node_addr: string]) -> uuid
uuid_generate_v7() -> uuid
uuid_nil() -> uuid
uuid_max() -> uuid
uuid_ns_dns() -> string
uuid_ns_url() -> string
uuid_ns_oid() -> string
uuid_ns_x500() -> string
uuid_is_valid(uuid: uuid) -> boolean
uuid_to_bin(uuid: uuid) -> bytes
uuid_from_bin() -> uuid
bin_from_uuid() -> uuid
```

#### Examples

```text
MariaDB [(none)]> select uuid_generate_v6();
+--------------------------------------+
| uuid_generate_v6()                   |
+--------------------------------------+
| 1ede5b09-ea01-6208-bca8-8809c0dd8e70 |
+--------------------------------------+
1 row in set (0.000 sec)

MariaDB [(none)]> select hex(uuid_to_bin(uuid_generate_v4()));
+--------------------------------------+
| hex(uuid_to_bin(uuid_generate_v4())) |
+--------------------------------------+
| B1B3AB9D490A4D20BFBD026AB1C045FB     |
+--------------------------------------+
1 row in set (0.002 sec)
```

[`uuid-osp`]: https://www.postgresql.org/docs/current/uuid-ossp.html

### Hash Algorithms

This library provides the following functions:

  - `blake2b512`, `blake2s256`, `blake3`, `blake3_thd`. `blake3_thd` provides
    a multithreaded hasher that can be much faster for large data; per the docs,
    128 KiB is about the minimum size to see any signifcant improvement over
    `blake3`.
  - `sha224`, `sha256`, `sha384`, `sha512` (these are also built in)
  - `keccak224`, `keccak256`
  - `sha3_224`, `sha3_256`, `sha3_384`, `sha3_512`
  - `xxhash3`, `xxhash32`, `xxhash64`, `xxhash` (`xxhash` is an alias for
    `xxhash64`)

All of these return hex strings by default. `_bin` functions are also
provided that return the binary result without going through hexification,
suitable for storage in a `BINARY(X)` column.


#### Usage

```text
blake2b512(a: any [, ...]) -> string
blake2b512_bin(a: any [, ...]) -> bytes
blake2s512(a: any [, ...]) -> string
blake2s512_bin(a: any [, ...]) -> bytes
blake3(a: any [, ...]) -> string
blake3_bin(a: any [, ...]) -> bytes
blake3_thd(a: any [, ...]) -> string
blake3_thd_bin(a: any [, ...]) -> bytes
md5_u(a: any [, ...]) -> string
md5_u_bin(a: any [, ...]) -> bytes
sha1_u(a: any [, ...]) -> string
sha1_u_bin(a: any [, ...]) -> bytes
sha224(a: any [, ...]) -> string
sha224_bin(a: any [, ...]) -> bytes
sha256(a: any [, ...]) -> string
sha256_bin(a: any [, ...]) -> bytes
sha384(a: any [, ...]) -> string
sha384_bin(a: any [, ...]) -> bytes
sha512(a: any [, ...]) -> string
sha512_bin(a: any [, ...]) -> bytes
keccak224(a: any [, ...]) -> string
keccak224_bin(a: any [, ...]) -> bytes
keccak256(a: any [, ...]) -> string
keccak256_bin(a: any [, ...]) -> bytes
sha3_224(a: any [, ...]) -> string
sha3_224_bin(a: any [, ...]) -> bytes
sha3_256(a: any [, ...]) -> string
sha3_256_bin(a: any [, ...]) -> bytes
sha3_384(a: any [, ...]) -> string
sha3_384_bin(a: any [, ...]) -> bytes
sha3_512(a: any [, ...]) -> string
sha3_512_bin(a: any [, ...]) -> bytes
xxhash(a: any [, ...]) -> integer
xxhash3(a: any [, ...]) -> integer
xxhash32(a: any [, ...]) -> integer
xxhash64(a: any [, ...]) -> integer
```

#### Examples

```text
MariaDB [(none)]> select blake3("Hello, world!");
+------------------------------------------------------------------+
| blake3("Hello, world!")                                          |
+------------------------------------------------------------------+
| EDE5C0B10F2EC4979C69B52F61E42FF5B413519CE09BE0F14D098DCFE5F6F98D |
+------------------------------------------------------------------+
1 row in set (0.000 sec)

MariaDB [(none)]> select sha3_256("Hello, world!");
+------------------------------------------------------------------+
| sha3_256("Hello, world!")                                        |
+------------------------------------------------------------------+
| F345A219DA005EBE9C1A1EAAD97BBF38A10C8473E41D0AF7FB617CAA0C6AA722 |
+------------------------------------------------------------------+
1 row in set (0.000 sec)

MariaDB [(none)]> select blake3_bin("Hello, world!");
+----------------------------------+
| blake3_bin("Hello, world!")      |
+----------------------------------+
| ����.ė�i�/a�/��Q����M        ������                      |
+----------------------------------+
1 row in set (0.000 sec)
```


For all hash functions, multiple arguments are combined to produce a single hash output:

```text
MariaDB [(none)]> select xxhash('Hello, ', 0x77, 'orld', '!');
+--------------------------------------+
| xxhash('Hello, ', 0x77, 'orld', '!') |
+--------------------------------------+
|                  -755700219241327498 |
+--------------------------------------+
1 row in set (0.000 sec)
```

Note that in SQL, all integers are an `i64`, all floats are a `f64`, and all
decimals are represented as a string to the UDF API. This library hashes these
types as their little endian representation on all platforms. (You only need
to worry about this if you have very obscure platform compatibility
requirements. Strings and blobs are always unambiguous).

### String Operations

Provide the function `levenshtein`, which calculates the levenshtein edit
distance between two strings. There is also `levenshtein_normalized` that
returns a value between 0.0 (identical) and 1.0 (significantly different).

If a limit is provided as a third argument, the operation will terminate if
that limit is exceeded. This can help to improve performance if filtering
dissimilar strings.

These algorithms provide a _byte_ edit distance, rather than unicode chars or
graphemes. These options may be added in the future.

These algorithms are implemented by the [`rapidfuzz`] crate.

[`rapidfuzz`]: https://crates.io/crates/rapidfuzz

#### Usage

```text
levenshtein(a: str, b: str [, limit: integer]) -> integer;
levenshtein_normalized(a: str, b: str [, limit: real]) -> real;
```

#### Example

```text
MariaDB [(none)]> SELECT levenshtein('foo', 'moose'), levenshtein_normalized('foo', 'moos');
+-----------------------------+---------------------------------------+
| levenshtein('foo', 'moose') | levenshtein_normalized('foo', 'moos') |
+-----------------------------+---------------------------------------+
|                           3 |                                   0.5 |
+-----------------------------+---------------------------------------+
1 row in set (0.001 sec)

MariaDB [(none)]> SELECT levenshtein('foo', 'moose', 2), levenshtein_normalized('foo', 'moos', 0.3);
+--------------------------------+--------------------------------------------+
| levenshtein('foo', 'moose', 2) | levenshtein_normalized('foo', 'moos', 0.3) |
+--------------------------------+--------------------------------------------+
|                              2 |                                        0.3 |
+--------------------------------+--------------------------------------------+
1 row in set (0.001 sec)
```

### Jsonify

Provide the function `jsonify`, which quickly creates JSON output for any given
inputs.

#### Usage

```text
jsonify(a: any [, ...]) -> string
```

#### Examples

```text
MariaDB [db]> select jsonify(qty, cost, class) from t1 limit 4;
+-------------------------------------+
| jsonify(qty, cost, class)           |
+-------------------------------------+
| {"class":"a","cost":50.0,"qty":10}  |
| {"class":"c","cost":5.6,"qty":8}    |
| {"class":"a","cost":20.7,"qty":5}   |
| {"class":"b","cost":12.78,"qty":10} |
+-------------------------------------+
4 rows in set (0.000 sec)
```

Aliasing also works to change key names:

```text
MariaDB [db]> select jsonify(uuid() as uuid, qty as quantity, cost) from t1 limit 4;
+----------------------------------------------------------------------------+
| jsonify(uuid() as uuid, qty as quantity, cost)                             |
+----------------------------------------------------------------------------+
| {"cost":50.0,"quantity":10,"uuid":"45952863-5b4d-11ed-b214-0242ac110002"}  |
| {"cost":5.6,"quantity":8,"uuid":"4595291b-5b4d-11ed-b214-0242ac110002"}    |
| {"cost":20.7,"quantity":5,"uuid":"45952953-5b4d-11ed-b214-0242ac110002"}   |
| {"cost":12.78,"quantity":10,"uuid":"4595297a-5b4d-11ed-b214-0242ac110002"} |
+----------------------------------------------------------------------------+
4 rows in set (0.001 sec)
```

### Lipsum

Uses the [lipsum crate] to generate lipsum strings with a specified word count.

#### Usage

```text
lipsum(count: integer [, seed: integer]) -> string
```

#### Examples


```text
MariaDB [(none)]> select lipsum(10);
+------------------------------------------------------------------+
| lipsum(10)                                                       |
+------------------------------------------------------------------+
| Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do. |
+------------------------------------------------------------------+
1 row in set (0.000 sec)
```

[lipsum crate]: https://docs.rs/lipsum/latest/lipsum/

### IP Address Functions

We provide three IP functions:

- `ip_validate` which will return either `ipv4` or `ipv6` if the format is
  valid, `NULL` otherwise.
- `ip_to_ipv6_mapped` which converts ipv4 addresses to their ipv6 form (e.g.
  for interop with the `INET6` data type)
- `ip_to_canonical` which reverses the mapping operation

#### Usage

```text
ip_validate(ip: string) -> string
ip_to_canonical(ip: string) -> string
ip_to_ipv6_mapped(ip: string) -> string
```

#### Examples

```text
MariaDB [db]> select
    ->     input,
    ->     ip_validate(input),
    ->     ip_to_ipv6_mapped(input),
    ->     ip_to_canonical(input)
    -> from t1;
+--------------------------------------+--------------------+--------------------------------------+--------------------------------------+
| input                                | ip_validate(input) | ip_to_ipv6_mapped(input)             | ip_to_canonical(input)               |
+--------------------------------------+--------------------+--------------------------------------+--------------------------------------+
| 203.0.113.0                          | ipv4               | ::ffff:203.0.113.0                   | 203.0.113.0                          |
| 127.0.0.1                            | ipv4               | ::ffff:127.0.0.1                     | 127.0.0.1                            |
| ::ffff:127.0.0.1                     | ipv6               | ::ffff:127.0.0.1                     | 127.0.0.1                            |
| 2001:db8::1:0:0:1                    | ipv6               | 2001:db8::1:0:0:1                    | 2001:db8::1:0:0:1                    |
| 2001:db8:85a3:8d3:1319:8a2e:370:7348 | ipv6               | 2001:db8:85a3:8d3:1319:8a2e:370:7348 | 2001:db8:85a3:8d3:1319:8a2e:370:7348 |
| hello!                               | NULL               | NULL                                 | NULL                                 |
| NULL                                 | NULL               | NULL                                 | NULL                                 |
+--------------------------------------+--------------------+--------------------------------------+--------------------------------------+
7 rows in set (0.000 sec)
```

## Installation

Compiled library binaries can be downloaded from this library's [releases] page.
The desired files can be copied to the plugin directory (usually
`/usr/lib/mysql/plugin`) and selectively loaded:

```sql
-- **** Hash functions ****
CREATE OR REPLACE FUNCTION blake2b512 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION blake2s256 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION blake3 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION blake3_thd RETURNS string SONAME 'libudf_hash.so';
-- the md5 and sha functions have builtin versions, hence the `_u` suffix
CREATE OR REPLACE FUNCTION md5_u RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha1_u RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha224 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha256 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha384 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha512 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION keccak224 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION keccak256 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha3_224 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha3_256 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha3_384 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha3_512 RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION xxhash RETURNS integer SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION xxhash3 RETURNS integer SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION xxhash32 RETURNS integer SONAME 'libudf_hash.so';
-- `xxhash` and `xxhash64` are aliases
CREATE OR REPLACE FUNCTION xxhash64 RETURNS integer SONAME 'libudf_hash.so';

-- binary-returning versions of hash algorithms, as a convenience alternative to
-- `unhex(blake3(...))`
CREATE OR REPLACE FUNCTION blake2b512_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION blake2s256_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION blake3_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION blake3_thd_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION md5_u_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha1_u_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha224_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha256_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha384_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha512_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION keccak224_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION keccak256_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha3_224_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha3_256_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha3_384_bin RETURNS string SONAME 'libudf_hash.so';
CREATE OR REPLACE FUNCTION sha3_512_bin RETURNS string SONAME 'libudf_hash.so';

-- **** JSON creation function ****
CREATE OR REPLACE FUNCTION jsonify RETURNS string SONAME 'libudf_jsonify.so';

-- **** IP functions ****
CREATE OR REPLACE FUNCTION ip_validate RETURNS string SONAME 'libudf_net.so';
CREATE OR REPLACE FUNCTION ip_to_canonical RETURNS string SONAME 'libudf_net.so';
CREATE OR REPLACE FUNCTION ip_to_ipv6_mapped RETURNS string SONAME 'libudf_net.so';

-- **** string operation functions ****
CREATE OR REPLACE FUNCTION levenshtein RETURNS integer SONAME 'libudf_stringops.so'
CREATE OR REPLACE FUNCTION levenshtein_normalized RETURNS real SONAME 'libudf_stringops.so'

-- **** random string generation ****
CREATE OR REPLACE FUNCTION lipsum RETURNS string SONAME 'libudf_lipsum.so';

-- **** UUID interfaces ****
CREATE OR REPLACE FUNCTION uuid_generate_v1 RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_generate_v1mc RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_generate_v4 RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_generate_v6 RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_generate_v7 RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_nil RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_max RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_ns_dns RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_ns_url RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_ns_oid RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_ns_x500 RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_is_valid RETURNS integer SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_to_bin RETURNS string SONAME 'libudf_uuid.so';
CREATE OR REPLACE FUNCTION uuid_from_bin RETURNS string SONAME 'libudf_uuid.so';
-- `bin_to_uuid` and 'uuid_from_bin' are aliases
CREATE OR REPLACE FUNCTION bin_to_uuid RETURNS string SONAME 'libudf_uuid.so';
```

Note that Windows `.dll`s are built but have not been tested - please open an
issue if you encounter any errors.

[releases]: https://github.com/pluots/udf-suite/releases


### Building from Source

To build the binaries yourself, you can clone this repository and run:

```sh
cargo build --release
```

Which will produce the desired dynamic library files in `target/release`.
Specific functions can also be specified with `-p` (e.g.
`cargo build --release -p udf-uuid`).

This repository also comes with a docker file that simplifies getting an image
up and running:

```sh
# build the image
docker build . --tag mdb-udf-suite-img

# run it in the background
docker run --rm -d \
  -e MARIADB_ROOT_PASSWORD=example \
  --name mdb-udf-suite \
  mdb-udf-suite-img

# Enter a SQL shell
docker exec -it mdb-udf-suite mariadb -pexample

# Stop the server when done
docker stop mdb-udf-suite
```

The UDFs can then be loaded using the `CREATE FUNCTION` statements above.

This project has a MSRV of 1.65, but makes no commitment to uphold this.


## License

This work is dual-licensed under Apache 2.0 and GPL 2.0 (or any later version).
You can choose either of them if you use this work.

`SPDX-License-Identifier: Apache-2.0 OR GPL-2.0-or-later`
