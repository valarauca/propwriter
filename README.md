propwriter
---

This is an extremely simple prop writer for win32.
It is used to write media tags so you can better organize/search
your media.

If you need help see `--help`

# Sample Usage

```
propwriter.exe \
  C:\valarauca\My Videos\AttackOnTitan-S04E16.mp4 \
  --title 'Above And Beyond' \
  --year '2021'
  --season '4'
  --episode '16'
  --series 'attack on titan'
  --genre 'anime' 'action' 'fiction' 'apocalyptic' 'post-apocalyptic' \
          'dark fanasty' 'mecha' 'drama' \
  --producers 'Tomoko Hiramuki' 'Teruyuki Omine' 'Jun Shishido' \
  --writers 'Faure Benjamin' \
  --artists 'Yûki Kaji' 'Ayane Sakura' 'Manami Numakura' \
            'Mitsuki Saiga' 'Romi Park' 'Takehito Koyasu'
```

Not all tags exactly align with roles. Get mad at Microsoft not me.

The time it takes to run is related to file size, updating metadata requires
reading the whole file from disk, and writing it back.
