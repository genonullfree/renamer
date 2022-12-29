# Renamer

Renamer is a small app that swaps file names by moving them around. This can be handy when backing up shows and the files on the disc are not in the correct order.

Renamer currently only works with a 1:1 renaming scheme (i.e.: a.mkv and b.mkv are oppositely named and need to be swapped; every rename resolves to a pair A <=> B).

## Commands

There are 3 commands to `renamer`:
- *swap*
- *map*
- *name*

```bash
renamer swap -a <A> -b <B>
renamer map -m mapfile.txt
renamer name -p Show_Name_S01E -s 12 -f files*.mkv
```

## Swap

The `swap` command takes two files and renames them to the other name. I.e.:
```
a -> tmp
b -> a
tmp -> b
```

## Map

The `map` command takes a `mapfile` that contains the swapping map list. This list is a simple list of A <=> B name swaps.

### Mapfile

The mapfile can be constructed by concatenating two filenames to be swapped with a `:`. Below is an example mapfile:

```
a.mkv:b.mkv
c.mkv:d.mkv
```

This will rename `b.mkv` to `a.mkv` and `a.mkv` to `b.mkv`, and the same for `c.mkv` and `d.mkv`.

## Name

The `name` command will take a list of files and rename them with a specific pattern.

```bash
$ renamer name -p Show_Name_S01E -s 12 -f file1.mkv, file2.mkv, file3.mkv
$ ls -l
Show_name_S01E12.mkv
Show_name_S01E13.mkv
Show_name_S01E14.mkv
```
