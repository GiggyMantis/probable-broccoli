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
The `Other(u8)` value is for use in noun classes with no particular themes. It should be avoided whenever possible, but if not, that's okay.
The `None` class avoids all distance comparison. It should be used for nongendered objects (such as verbs) as well as in languages where there is not enough information to determine noun class (see Tocharian). 

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
- ◌̩
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
Stress diacritics go **immediately** *before* the vowel they modify. Do *not* mark stress on one-syllable words.

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
```rust
pub const _CASE_ADESSIVE: u64 = 1;
pub const _CASE_ANTESSIVE: u64 = 2;
pub const _CASE_ADUPESSIVE: u64 = 4;
pub const _CASE_INESSIVE: u64 = 8;
pub const _CASE_INTRATIVE: u64 = 16;
pub const _CASE_LOCATIVE: u64 = 32;
pub const _CASE_PERTINGENT: u64 = 64;
pub const _CASE_POSTESSIVE: u64 = 128;
pub const _CASE_SUBESSIVE: u64 = 256;
pub const _CASE_SUPERESSIVE: u64 = 512;
pub const _CASE_ABLATIVE: u64 = 1024;
pub const _CASE_ADELATIVE: u64 = 2048;
pub const _CASE_DELATIVE: u64 = 4096;
pub const _CASE_EGRESSIVE: u64 = 8192;
pub const _CASE_ELATIVE: u64 = 16384;
pub const _CASE_INITIATIVE: u64 = 32768;
pub const _CASE_POSTELATIVE: u64 = 65536;
pub const _CASE_ALLATIVE: u64 = 131072;
pub const _CASE_ILLATIVE: u64 = 262144;
pub const _CASE_LATIVE: u64 = 524288;
pub const _CASE_SUBLATIVE: u64 = 1048576;
pub const _CASE_SUPERLATIVE: u64 = 2097152;
pub const _CASE_TERMINATIVE: u64 = 4194304;
pub const _CASE_PERLATIVE: u64 = 8388608;
pub const _CASE_PROLATIVE: u64 = 16777216;
pub const _CASE_ACCUSATIVE: u64 = 33554432;
pub const _CASE_ESSIVE: u64 = 67108864;
pub const _CASE_LIMITATIVE: u64 = 134217728;
pub const _CASE_TEMPORAL: u64 = 268435456;
pub const _CASE_ABSOLUTIVE: u64 = 536870912;
pub const _CASE_AGENTIVE: u64 = 1073741824;
pub const _CASE_DIRECT: u64 = 2147483648;
pub const _CASE_ERGATIVE: u64 = 4294967296;
pub const _CASE_INSTRUCTIVE: u64 = 8589934592;
pub const _CASE_INSTRUMENTAL: u64 = 17179869184;
pub const _CASE_NOMINATIVE: u64 = 34359738368;
pub const _CASE_OBLIQUE: u64 = 68719476736;
pub const _CASE_INTRANSITIVE: u64 = 137438953472;
pub const _CASE_PEGATIVE: u64 = 274877906944;
pub const _CASE_AVERSISVE: u64 = 549755813888;
pub const _CASE_BENEFACTIVE: u64 = 1099511627776;
pub const _CASE_CARITATIVE: u64 = 2199023255552;
pub const _CASE_CAUSAL: u64 = 4398046511104;
pub const _CASE_COMITATIVE: u64 = 8796093022208;
pub const _CASE_DATIVE: u64 = 17592186044416;
pub const _CASE_DISTRIBUTIVE: u64 = 35184372088832;
pub const _CASE_GENITIVE: u64 = 70368744177664;
pub const _CASE_ORNATIVE: u64 = 140737488355328;
pub const _CASE_POSSESSED: u64 = 281474976710656;
pub const _CASE_POSSESSIVE: u64 = 562949953421312;
pub const _CASE_ABESSIVE: u64 = 1125899906842624;
pub const _CASE_SEMBLATIVE: u64 = 2251799813685248;
pub const _CASE_SOCIATIVE: u64 = 4503599627370496;
pub const _CASE_SUBSTITUTIVE: u64 = 9007199254740992;
pub const _CASE_PARTITIVE: u64 = 18014398509481984;
pub const _CASE_ADPOSITIONAL: u64 = 36028797018963968;
pub const _CASE_VOCATIVE_OR_EMPHATIC: u64 = 72057594037927936;
pub const _CASE_ADVERBIAL: u64 = 144115188075855872;
pub const _CASE_COMPARATIVE: u64 = 288230376151711744;
pub const _CASE_EXESSIVE: u64 = 576460752303423488;
pub const _CASE_ORIENTATIVE: u64 = 1152921504606846976;
pub const _CASE_REVERTIVE: u64 = 2305843009213693952;
pub const _CASE_TRANSLATIVE: u64 = 4611686018427387904;
pub const _CASE_REFLEXIVE: u64 = 9223372036854775808;
```

### "nominal_cases"
This is a *bitflag* field.
See [### "pronominal_cases"](###-"pronominal_cases") for the case list.

### "pronominal_numbers"
This is a *bitflag* field.
```rust
pub const _NUM_SINGULAR: u8 = 1;
pub const _NUM_DUAL: u8 = 2;
pub const _NUM_TRIAL: u8 = 4;
pub const _NUM_PAUCAL: u8 = 8;
pub const _NUM_PLURAL: u8 = 16;
pub const _NUM_QUADRAL: u8 = 32;
pub const _NUM_SUPERPLURAL: u8 = 64;
pub const _NUM_DISTRIBUTIVE_PLURAL: u8 = 128;
```

### "nominal_numbers"
This is a *bitflag* field.
See [### "pronominal_numbers"](###-"pronominal_numbers") for the case list.

### "finitivity"
This is a *bitflag* field.
```rust
pub const _FINITIVITY_HAS_ARTICLES: u8 = 1;
pub const _FINITIVITY_DEFINITE: u8 = 2;
pub const _FINITIVITY_INDEFINITE: u8 = 4;
pub const _FINITIVITY_PARTITIVE: u8 = 8;
pub const _FINITIVITY_NEGATIVE: u8 = 16;
```

### "declensions"
This is if the language is typically analyzed as having distinct declension groups. For example, Latin is while English is not.

### "formality"
This refers to a formality distinction in pronouns, *not* refering to if the language has formality generally.

### "tenses"
This is a *count* field.

### "aspect"
This is a *bitflag* field.
```rust
pub const _ASPECT_PERFECTIVE: u32 = 1;
pub const _ASPECT_MOMENTANE: u32 = 2;
pub const _ASPECT_PERFECT: u32 = 4;
pub const _ASPECT_RECENT_PERFECT: u32 = 8;
pub const _ASPECT_IMPERFECT: u32 = 16;
pub const _ASPECT_PROSPECTIVE: u32 = 32;
pub const _ASPECT_IMPERFECTIVE: u32 = 64;
pub const _ASPECT_HABITUAL: u32 = 128;
pub const _ASPECT_CONTINUOUS: u32 = 256;
pub const _ASPECT_PROGRESSIVE: u32 = 512;
pub const _ASPECT_STATIVE: u32 = 1024;
pub const _ASPECT_GNOMIC: u32 = 2048;
pub const _ASPECT_EPISODIC: u32 = 4096;
pub const _ASPECT_CONTINUATIVE: u32 = 8192;
pub const _ASPECT_INGRESSIVE: u32 = 16384;
pub const _ASPECT_INCHOACTIVE: u32 = 32768;
pub const _ASPECT_CESSATIVE: u32 = 65536;
pub const _ASPECT_DEFECTIVE: u32 = 131072;
pub const _ASPECT_PAUSATIVE: u32 = 262144;
pub const _ASPECT_RESUMPTIVE: u32 = 524288;
pub const _ASPECT_PUNCTUAL: u32 = 1048576;
pub const _ASPECT_DURATIVE: u32 = 2097152;
pub const _ASPECT_PROTRACTIVE: u32 = 4194304;
pub const _ASPECT_ITERATIVE: u32 = 8388608;
pub const _ASPECT_FREQUENTIVE: u32 = 16777216;
pub const _ASPECT_EXPERENTIAL: u32 = 33554432;
pub const _ASPECT_INTENTIONAL: u32 = 67108864;
pub const _ASPECT_ACCIDENTAL: u32 = 134217728;
pub const _ASPECT_INTENSIVE: u32 = 268435456;
pub const _ASPECT_MODERATIVE: u32 = 536870912;
pub const _ASPECT_ATTENUATIVE: u32 = 1073741824;
pub const _ASPECT_SEGMENTATIVE: u32 = 2147483648;
```

### "mood"
This is a *bitflag* field.
```rust
pub const _MOOD_INDICATIVE: u32 = 1;
pub const _MOOD_ENERGETIC: u32 = 2;
pub const _MOOD_DECLARATIVE: u32 = 4;
pub const _MOOD_SUBJUNCTIVE: u32 = 8;
pub const _MOOD_CONDITIONAL: u32 = 16;
pub const _MOOD_OPTATIVE: u32 = 32;
pub const _MOOD_JUSSIVE: u32 = 64;
pub const _MOOD_POTENTIAL: u32 = 128;
pub const _MOOD_IMPERATIVE: u32 = 256;
pub const _MOOD_PROHIBITIVE: u32 = 512;
pub const _MOOD_DESIDERATIVE: u32 = 1024;
pub const _MOOD_DUBITATIVE: u32 = 2048;
pub const _MOOD_HYPOTHETICAL: u32 = 4096;
pub const _MOOD_PRESUMPTIVE: u32 = 8192;
pub const _MOOD_PERMISSIVE: u32 = 16384;
pub const _MOOD_ADMIRATIVE: u32 = 32768;
pub const _MOOD_HORTATIVE: u32 = 65536;
pub const _MOOD_PRECATIVE_VOLITIVE: u32 = 131072;
pub const _MOOD_INFERENTIAL: u32 = 262144;
pub const _MOOD_NECESSITATIVE: u32 = 524288;
pub const _MOOD_INTERROGATIVE: u32 = 1048576;
pub const _MOOD_BENEDICTIVE: u32 = 2097152;
pub const _MOOD_CONCESSIVE: u32 = 4194304;
pub const _MOOD_PRESCRIPTIVE: u32 = 8388608;
```

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

# Congratulations
After all that hard work, you are done. Please note that this is subject to change as the program evolves.