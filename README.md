# Renamer

Renamer is a small app that swaps file names by moving them around. This can be handy when backing up shows and the files on the disc are not in the correct order.

## Commands

There are 3 commands to `renamer`:
- *swap*
- *map*
- *name*

```bash
renamer swap -a <A> -b <B>
renamer swaplist -m mapfile.txt
renamer remap -m mapfile.txt
renamer name -p Show_Name_S01E -s 12 -f files*.mkv
```

## Swap

The `swap` command takes two files and renames them to the other name. I.e.:
```
a -> tmp
b -> a
tmp -> b
```

## Swaplist

The `swaplist` command takes a `mapfile` that contains the swapping map list. This list is a simple list of A <=> B name swaps. Each item in this list should only occur once.

## Remap

The `remap` command operates similarly to the `swaplist` command except that each line in the maplist file A:B means A will be renamed to B.

### Mapfile

The mapfile can be constructed by concatenating two filenames to be swapped with a `:`. Below is an example mapfile:

```
a.mkv:b.mkv
c.mkv:d.mkv
```

This will rename `b.mkv` to `a.mkv` and `a.mkv` to `b.mkv`, and the same for `c.mkv` and `d.mkv`.

## Name

The `name` command will take a list of files and rename them with a specific pattern. It can also now automatically detect what number to start at!

```bash
$ ls *
Show_Name_S01E01.mkv  Show_Name_S01E02.mkv  Show_Name_S01E03.mkv

dir:
a_01.mkv  a_02.mkv  a_03.mkv

$ renamer name -p Show_Name_S01E -f dir/a_0*
dir/a_01.mkv => Show_Name_S01E04.mkv
dir/a_02.mkv => Show_Name_S01E05.mkv
dir/a_03.mkv => Show_Name_S01E06.mkv
```
