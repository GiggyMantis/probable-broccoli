# Document Purpose
This document details how to make .json files that the program can understand so you can create trees with Probable Broccoli.
Eventually, a languoid editor will be a standalone program bundled with Probable Broccoli, but currently, you have to make this manually.

# Terms
Terms used in this document:
"Serializing [a language]" - refers to the process of entering language data into a compatible .json file.
"Bitflag" - A binary integer where each bit refers to the presence or lack of a specific feature. These are used for cases, mood, and aspect. These are used instead of an array of values to increase the speed of the program.
"Count" - A normal integer that refers to the amount of different options. This is used, for example, for tense. Where `"tenses" = 2` means there are two tenses, without specifying what tenses those are.
"Enum" - A string (and possible sub-value) refering to a possible predeined value in the program. These are used for accent system and for noun classes.

# How To Guide

## Step 0: Research
Before you even get started on serializing, you should learn a bit about the language. You will need a language with the folloing things:
- Documented Phonology
- Documented Grammar
- Enough words to fill out the Leipzig-Jakarta list.*

*: If you do not have an attested form for terms on the list, reconstructed forms are preffered over empty strings.

## Step 1: Noun Classes
Create your noun classes in the `"noun_classes"` value of the grammar section. This should be in the form of an array, where each index in the array has a further array of *enum* values. (the `"t"` value of each category holds this).
### List of noun class categories
    None,
    Neuter,
    Feminine,
    Masculine,
    Epicene,
    Common,
    Animate,
    Inanimate,
    Count,
    Mass,
    Plural,
    Vegetable,
    Terrestrial,
    Celestial,
    Collective,
    Other(u8)

Note that each noun class is composed of 1+ of these categories.
For example, the *Common* gender in Dutch is written as 
```json
[
    {
      "t": "Masculine"
    },
    {
      "t": "Feminine"
    }
]
```

I reccomend making value 0 (the first value in the array) a `None` class, but the order is actually up to you. Just make sure you remember which index refers to which noun class, and you're golden.
You will do the same process for the noun classes used in pronouns, if any.
You will us these noun classes in a very simple way. If you wish to refer to the first defined noun class, your index is `0`, the second is `1`, and so on. You will use these to mark what noun class each term in the Leipzig-Jakrata list belongs to. If it is not a noun, or if it is the neutral term of any other class, please use the `None` class.

## Step 2: Serializing Terms in the Leipzig-Jakarta list
Add translations to each termk in the Leipzig-Jakarta list. There are 100 of these terms. Input the terms in the IPA, and the **index** of the noun class you defined in the previous step.

### IMPORTANT NOTE
The conventions for the inputted IPA are **very strict**.
Refrain from using diacritics whenever possible. The following common diacritical forms are completely banned:
- ◌̈, including ä
- ◌̽ 
- ◌͊
- ◌͋
- ◌ᵊ
- ◌͡◌
- ◌̯
- ◌̹ and ◌̜
- ◌̮
- ◌ˑ
- ◌̆
- ◌̬
- **all** bottom-versions of diacritcs that have above versions.
- **Do not** mark apical/laminality. Please use a sibilance/nonsibilance method instead, such as s/θ for Spanish [con la distinción] and Baqsue, or ʐ/ɻ̝ for Sinological transcription.

The following comon diacritical forms are allowed:
- ◌̚
- ◌̊
- ◌̝ (as in r̝, but not as in vowels)
- ◌ˀ

Every other diacritic is discouraged.

**Stress Marking**
Stress diacritics go **immediately** *before* the vowel they modify. 

**Affricates**
Tie-bar forms of affricates are not implemented. Please use the ligature forms. If you need to copy-paste those, here is a list:
ʧ
ʨ
ʦ
ꭧ
ʤ
ʥ
ʣ
ꭦ

**Linking diacritic**
Please refrain from using the linking diacritic. Instead, please use

**The period**
The period is not used to mark syllable boundaries, but is rather a spacing character that you can use whenever you want. It is deleted before the program parses it.

**Semivowels**
Please do not use "syllabic" vowels. Use the semivowel equivalent if it has a seperate IPA letter, and if not, use the vowel itself.

## Step 3: Grammar
Mark the grammar primary word order.
Examples of nonstandard word orders:
"VO" - for "verb object"
"OV" - for "object verb"
"XVX" - for "verb second"
"XXX" - for "completely free"

### "pronominal_cases"
This is a *bitflag* field. 

### "nominal_cases"
This is a *bitflag* field.

### "pronominal_numbers"
This is a *bitflag* field.

### "nominal_numbers"
This is a *bitflag* field.

### "finitivity"
This is a *bitflag* field.

### "declensions"
This is if the language is typically analyzed as having distinct declension groups. For example, Latin is while English is not.

### "formality"
This refers to a formality distinction in pronouns, *not* refering to if the language has formality generally.

### "tenses"
This is a *count* field.

### "aspect"
This is a *bitflag* field.

### "mood"
This is a *bitflag* field.

### "evidentials"
This is a *count* field.

## Step 4: Phonology
Finally, complete the language's phonologicl analysis.

### "tone_count"
This is a *count* field. Note that this counts tonemes, not pure tones. (i.e. Mandarin has `"tone_count" = 5`, not 2.)

### "vowel_length"
Refers to if there is phonemic vowel length. Modern English does not have phonemic vowel length, but it does have phonetic vowel length.

### "reduction"
Refers to if the language has a reduction system for [unstressed] vowels. For example, Egyptian Arabic *does* while Modern Standard Arabic *does not*. Reduction is the process of vowels moving towards either the center of the vowel space (as in English), a *lower* location in the vowel space (as in Latin), or the mean of two+ vowels (as in Bulgarian).

### "accent_system"
This is an *enum* value.

  None,
  ContrastivePitch,
  ContrastiveStress,
  SystemicPitch,
  SystemicStress

### "has_phonemic_diphthongs" and "has_phonetic_diphthongs"
These two values are very similar but the difference matters a lot.
Languages like Spanish have phonemic diphthongs but *not* phonetic diphthongs. This is because the only phonetic diphthongs they have are the realizations of phonemic diphthongs, as in *soy* /soj/. Languages like English have both phonemic and phonetic diphthongs. Phonemic as in *soy* /sɔj/, as well as phonetic as in *my* /maj/. Languages like German (in native words only) have phonetic diphthongs such as *haus* /haws/, which is phonetic because it is actually the realization of the metaphoneme |uː|. 

### "basic_vowel_count"
Represents the *count* of basic vowels, not including phonemic and metaphonemic vowel modifiers. For example, Spanish has 5, MSA has 3, Modern Dutch has 7, Hawaiian has 5, and Classical Tupi has 6. For languages with debated phonemic status, such as /ɨ/ in the East Slavic languages, it does not matter if you count them as long as you analyze them consistently.

### "has_vibrants"
Vibrants are any tapped or trilled sound.

### "gemination"
This refers to gemination of consonants, not vowels. For vowel gemination, see "vowel_length" earlier in the .json file.

### "emphatics"
Emphatics are uvularized/pharyngealized/epiglottalized consonants found mostly in Semitic languages but may exist as the realization of some continental Germanic fortis/lenis distinctions.

### "mora_timed"
This is a famously hard feature to analyze for languages, especially of one's own dialect. If you are unsure, put *false*.