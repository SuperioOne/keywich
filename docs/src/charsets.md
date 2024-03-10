# Charsets

The actual generated password is a fixed-size binary blob based on target length [^charset], which is then converted to
text within a given character range by charset. This is useful when a site or application does not allow certain
characters.

By default, three character sets are created:

- Numeric `0..9`
- Alpha Numeric `a..zA..Z0..9`
- Alpha `a..zA..Z`

## Charset pattern rules

1. The order of range and characters do not matter. For example, `_-!a..zA..Z` and `A..Z_-a..z!` will produce the
   same charset.
2. Duplicate values and ranges are ignored. For instance, `__A..Z__A..Z__A..Z__` is parsed as `A..Z_`
3. The left-hand side of a range should be smaller than the right-hand side. Ranges such as `k..a` and `9..1` are not
   accepted.
4. Ranges are case-sensitive. Avoid mixing cases like `A..z`.

Example charset pattern with complex symbols:

```
A..Za..z0..9#-_'`"><)(%.,!$€£*+~:;{}[]&
```

Example charset pattern with German alphabet:

```
A..Za..z0..9ÄäÖöẞßÜü
```

## Managing charsets

1. Open **Settings** > **Charsets** and click the **Create** button to create a new charset.
2. Enter a unique charset name.
3. Enter the charset pattern.
4. (Optional) Add a description for the charset.
5. Confirm.
6. New charset will be available when modifying or creating keys.

<div class="warning">

> Deleting a charset and creating a different one with the identical name does not affect existing keys. They will
> continue using the prior one to prevent any unintended password changes.

</div>

## Syntax definition

> *ALPHA_LC* = ASCII lowercase characters\
> *ALPHA_UC* = ASCII uppercase characters\
> *NUMERIC* = ASCII Digits\
> *UTF8* = Any valid UTF8 character\
> *range* = (`ALPHA_LC` ".." `ALPHA_LC`) / (`ALPHA_UP` ".." `ALPHA_UP`) / (`NUMERIC` ".." `NUMERIC`)\
> *content* = `range` / `UTF8`\
> *charset* = 1*`content`


[^charset]: [Target length](./key_parameters.md#target-length) is a key parameter.