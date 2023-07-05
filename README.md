# outscale

outscale is a simple excercise I did to learn Rust. It takes as input a musical note and a scale (heptatonic major or minor only right now), and gives as output the tones of that scale.

The notes that represents the same sounds are written together, separated by a forward slash as in "D#/Eb". They must also be specified in the same way when requesting a scale.

The scale type can be requested in many different ways:

| Major | Minor |
| ----- | ----- |
| Major | Minor |
| M     | m     |
| MAJOR | MINOR |
| MAJ   | MIN   |
| Maj   | Min   |
| maj   | min   |
| major | minor |

The scale must be requested with the structure:

`<keynote> <scale type>`

```
write the scale you would like to know:
G min
["G", "A", "A#/Bb", "C", "D", "D#/Eb", "F", "G"]

write the scale you would like to know:
D#/Eb m
["D#/Eb", "F", "F#/Gb", "G#/Ab", "A#/Bb", "B", "C#/Db", "D#/Eb"]
```

If the input is not valid for any reason (key not found, scale type not found, wrong syntax ecc...) the program will stop and display an error.

```
write the scale you would like to know:
W min
Can't compute this scale, please check your request

write the scale you would like to know:
F wrong
Can't compute this scale, please check your request
```

I tried to use custom enums and structs as exercise but I quite messed it up. I just couldn't figure out how I could keep the informations of the scales and the notes inside their structs and enums and so I just made some constants to help me.

I feel this thing can be done much cleaner but I keep missing something.

Nevertheless, it works :P
