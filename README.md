# Renamer

Renamer is a small app that swaps file names by moving them around. This can be handy when backing up shows and the files on the disc are not in the correct order.

Renamer currently only works with a 1:1 renaming scheme (i.e.: a.mkv and b.mkv are oppositely named and need to be swapped; every rename resolves to a pair A <=> B).

```
a -> tmp
b -> a
tmp -> b
```

## Usage

```bash
renamer swap -a <A> -b <B>
```
or
```bash
renamer map -m mapfile.txt
```

## Mapfile

The mapfile can be constructed by concatenating two filenames to be swapped with a `:`. Below is an example mapfile:

```
a.mkv:b.mkv
c.mkv:d.mkv
```

This will rename `b.mkv` to `a.mkv` and `a.mkv` to `b.mkv`, and the same for `c.mkv` and `d.mkv`.
