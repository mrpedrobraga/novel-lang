# The `novel` language

A language for writing movie and videogame scripts,
built for the digital era.

With `novel`, you can write scripts in a rich syntax
that can be understood by humans and computers,
thus supporting database-like introspection features.
It's highly inspired by `fountain`, `markdown` and `sol`.

`novel` is still in VERY early WIP, but you can read about
what it can do.

## Syntax & Features

Each file starts with the file name and maybe a subtitle.

```novel
MY MOVIE SCRIPT
by Pedro Braga
```

Before the file proper you may add some frontmatter, though.
The properties here can be used by other software to do
all sorts of arbitrary things.

```yaml
---
myprop: 123
otherprop: "456"
---
```

After that, time to write your scenes.

A scene starts with a scene header (four equal signs).

```novel
====
```

It may be given a name.

```novel
== Scene 01 ==
```

Immediately following a header, you may add arbitrary metadata,
in a syntax similar to the frontmatter.

```novel
== Scene 01 ==
Where: [Hiere Forest]
When: 12:00
Music: [OST Hiere Forest]
```

The properties here are truly arbitrary, but here are some that
I recommend using, inspired by `fountain`:

```
Where, When, Music, Characters, Tags, Board, Author
```

Inside a scene (after a scene header) you can write action lines
simply by typing your paragraph.

```novel
The early sun rises in the distance. Birds chirp quietly.
```

You can write additional nodes preceding your paragraph by //.
These paragraphs will be ignored by certain processes.

```novel
The early sun rises in the distance. Birds chirp quietly.
// Make sure to draw the sun rising from the east.
```

For style and emphasis, you can use formatting.

```novel
The fog looks **ominous** but not _too_ scary, moreso lonely.
```

With square brackets you can add rich references, also known
as wikilinks, which are references that can be followed through
or hovered over in editing software. You can refer to "things"
(to be explained later) by their name, id, path;
or to files or websites.

```novel
At the bed, [Maple] and [Casper] are sleeping.
```

Rich references are REALLY powerful.
You can use them to be clear about characters, locations,
items, other scripts (and specific scenes within them)
and other concepts that might be ambiguous. Software
that uses novel is aware of what files connect to each other
and can help with navigation or make all sorts of visualisations.

You can write dialogues by declaring a new current speaker,
and following it with dialogue lines.

A lone reference to a character in a line declares a new speaker.
After that, you may use dashes (-) to add lines or parentheticals.

```novel
[Maple]
- (yawns)
- What a beautiful day today...
- (looking at Casper)
- When do you wanna get up?

Casper rolls to look at her eyes, with a sleepy smile.

[Casper]
- Never...
```

You can add tagged directives using >.
They are pretty like action lines,
but they contain a tag that they can be filtered by.

```novel
> TRANS: FADE OUT.
> CHYRON: November 17.
> CAMERA: Orbit slowly around subject.
> SFX: Loud train horn.
> TODO: Write this scene better.
```

Lastly, you can write some special directives using
the at sign (@).

Usually scenes are considered to transition to the next
scene written below them. But in an interactive medium,
a scene might lead to many other scenes.

You can specify that with the `CONT` directive.

```novel
Eventually, the two of them get up.

// Look yourself in the mirror.
@CONT Scene 02

// Getting out of the door.
@CONT Scene 03
```

Or `IF` to add quick variations.

```novel
@IF the player opens the chest {
    A ball of light falls from the sky,
    bestowing you with an item.
}
```