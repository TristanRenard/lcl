# LCL - Live Coding Language

Langage de live coding avec une syntaxe dérivée de brainfuck.

## Syntaxe générale

Chaque instruction suit le format `<count?><opérateur>`. Le count est optionnel et vaut 1 par défaut.

## Référence des caractères

| Caractère | Nom | Description |
|-----------|-----|-------------|
| `.` | Play | Joue la note courante |
| `\|` | Séparateur | Sépare les opérations, permet de désambiguër |
| `+` | Octave up (note) / Pitch up (MIDI) | Contexte note : monte de n octaves. Contexte MIDI : ajoute n au pitch |
| `-` | Octave down (note) / Pitch down (MIDI) | Contexte note : descend de n octaves. Contexte MIDI : soustrait n au pitch |
| `$+` | Demi-ton up | Monte de n demi-tons (contexte note uniquement) |
| `$-` | Demi-ton down | Descend de n demi-tons (contexte note uniquement) |
| `>` | Volume up | Augmente le volume de n |
| `<` | Volume down | Diminue le volume de n |
| `=` | Volume set | Set le volume à n (défaut 5) |
| `,` | Silence | n temps de silence (défaut 1) |
| `~` | Timbre | Change la forme d'onde : 0 sine, 1 saw, 2 square, 3 triangle |
| `_` | Durée | Contexte durée : `4_[...]` toutes les notes durent 4 temps |
| `[` `]` | Boucle | `n[...]` = n itérations, `[...]` = boucle infinie |
| `*` | Contexte sample | `*[...]` = tout le bloc joue des samples. `*.` = un seul output en sample |
| `\` | Contexte synth | `\[...]` = tout le bloc joue en synth. `\.` = un seul output en synth |
| `{` `}` | Contexte MIDI | Les nombres deviennent des numéros MIDI |
| `#` | Dièse | Après une lettre de note : `c#4` |
| `` ` `` | Bémol | Après une lettre de note : ``d`4`` |
| `c d e f g a b` | Notes | Lettres de notes, octave 4 par défaut |

## Exemples

```
c4.d.e.             Do4, Ré4, Mi4
64+16-              En MIDI : pitch = 48
12|5=.              Set pitch 12, volume 5, joue
3[42*.42.]38.       3x (sample 42, synth 42) puis synth 38
*[12.]              Boucle infinie en sample, joue 12
12|1,.13,.          Pitch 12, silence 1, joue, silence 13, joue
c#4.d`3.            Do dièse 4, Ré bémol 3
{60.5+.}            MIDI : joue 60, joue 65
```

## Modèle d'exécution

- **Slot loop** : une boucle `[...]` évaluée tourne indéfiniment
- **Queue one-shot** : les expressions sans boucle s'accumulent en FIFO
- Les one-shots se jouent entre deux itérations de la loop
- Un nouveau `[...]` évalué remplace la loop en fin d'itération courante
- Mode par défaut : synth, octave 4, volume 5, durée 1, timbre sine