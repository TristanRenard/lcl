# LCL - Live Coding Language

Langage de live coding dérivé de brainfuck avec une syntaxe `<count><opérateur>`.

## Syntaxe générale

Un programme est une séquence d'instructions. Chaque instruction est un nombre optionnel (count) suivi d'un opérateur. Sans count, la valeur par défaut dépend de l'opérateur.

Le séparateur `|` permet de désambiguïser entre un set pitch et un count. `12.` set le pitch à 12 et joue. `12|.` fait la même chose. `12,.` c'est 12 temps de silence puis joue.

## Pitch

### Contexte note (défaut)

Par défaut le langage est en notation musicale. Les notes sont exprimées par une lettre (`c d e f g a b`), une altération optionnelle, et une octave optionnelle.

- `c4` = do central (MIDI 60)
- `c#4` = do dièse 4
- `` d`4 `` = ré bémol 4
- `c` = do en octave par défaut (4)

Opérateurs relatifs en contexte note :
- `<n>+` / `<n>-` : monte/descend de n octaves (défaut 1)
- `<n>$+` / `<n>$-` : monte/descend de n demi-tons (défaut 1)

### Contexte MIDI `{}`

Entre `{}`, les nombres deviennent des numéros MIDI directement.

- `{60.}` = do central, équivalent à `c4.`
- `{60.5+.}` = joue 60 puis 65

Opérateurs relatifs en contexte MIDI :
- `<n>+` / `<n>-` : monte/descend de n demi-tons (défaut 1)

## Sortie

- `<n>.` : joue le pitch courant n fois (défaut 1)

## Volume

- `<n>=` : set le volume à n (défaut 5)
- `<n>>` : augmente le volume de n (défaut 1)
- `<n><` : diminue le volume de n (défaut 1)

`=` sans nombre devant met le volume à 5.

## Silence

- `<n>,` : n temps de silence (défaut 1)

## Durée

- `<n>_` : contexte de durée, toutes les notes du bloc durent n temps
- `<n>_[...]` : durée appliquée à un bloc

## Timbre (synthé uniquement)

- `0~` : sine
- `1~` : saw
- `2~` : square
- `3~` : triangle

Persiste jusqu'au prochain changement.

## Boucles

- `[...]` : boucle infinie
- `<n>[...]` : boucle n fois
- `3[c4.d4.e4.]` : joue do ré mi 3 fois

## Contextes audio

Par défaut le mode est synthé. `.` joue en synthé.

- `*` : force le contexte sample
- `\` : force le contexte synthé

Applicable à un seul output ou à un bloc :
- `*.` : joue en sample (one-shot)
- `\.` : joue en synthé (one-shot)
- `*[...]` : tout le bloc en sample
- `\[...]` : tout le bloc en synthé

Les contextes s'imbriquent : `*[12. \[14.] 8.]` = sample 12, synthé 14, sample 8.

## Runtime live coding

### Slot loop

Une seule boucle tourne en permanence. Évaluer une nouvelle boucle `[...]` (Ctrl+Enter) remplace l'ancienne à la fin de l'itération en cours.

### Queue one-shot (FIFO)

Les expressions sans boucle évaluées pendant l'exécution s'accumulent dans une queue. À la fin de chaque itération de la loop, le moteur vide toute la queue dans l'ordre (FIFO), puis reprend la loop.

### Point de synchronisation

Tout se passe à la frontière entre deux itérations : swap de loop, exécution des one-shots. Jamais d'interruption mid-loop.

## Exemples

```
12.=.4+3>.
```
Joue pitch 12, joue pitch 12 à volume 5, joue pitch 16 à volume 8.

```
{[64+.-12.]}
```
Boucle infinie en MIDI : ajoute 64 et joue, soustrait 12 et joue.

```
*[c4.d4.\[e4.]]
```
Boucle infinie : sample do4, sample ré4, synthé mi4.

```
3[c4.2,d4.]
```
3 fois : joue do4, 2 temps de silence, joue ré4.