use bitflags::bitflags;
pub mod compare;

fn main() {
    println!("Hello, world!");
}

struct Languoid {
    languoid_name: String,
    year: i16,
    leipzig_jakarta_list: [String, 100],
    grammar: Grammar,
    phonology: Phonology
}

struct Grammar {
    predicate_word_order: PredicateWordOrder,
    adjective_before_noun: bool,
    prepositions: bool,
    pronominal_cases: u64,
    nominal_cases: u64,
    finitivity: Finitivity,
    determiner_before_noun: bool,
    word_classes: Vec<String>,
    leipzig_jakarta_word_classes: [String, 100],
}

struct Phonology {
    tone_count: u8,
    vowel_length: bool,
    nasal_vowels: bool,
    rhotic_vowels: bool,
    front_rounded_vowels: bool,
    back_unrounded_vowels: bool,
    schwa: bool,
    reduction: bool,
    accent_system: Accent,
    plosive_series_count: u8,
    fricative_voicedness_distinction: bool,
    nasal_voicedness_distinction: bool,
    has_laterals: bool,
    has_phonemic_diphthongs: bool,
    has_phonetic_diphthongs: bool,
    basic_vowel_count: u8,
    has_glottal: bool,
    has_uvular: bool,
    has_labiodental: bool,
    has_linguodental: bool,
    has_retroflex_or_postalveolar: bool,
    has_palatal: bool
    has_labial: bool,
    has_velar: bool,
    has_pharyngeal_or_epiglottal: bool,
    has_vibrants: bool,
    gemination: bool,
    palatalization: bool,
    velarization: bool,
    labialization: bool,
    emphatics: bool
}

enum PredicateWordOrder {
    SVO,
    SOV,
    VSO,
    VOS,
    OSV,
    OVS
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct GrammaticalCase: u64 {
        const ADESSIVE = 0x1;
        const ANTESSIVE = 0x2;
        const ADUPESSIVE = 0x4;
        const INESSIVE = 0x8;
        const INTRATIVE = 0x10;
        const LOCATIVE = 0x20;
        const PERTINGENT = 0x40;
        const POSTESSIVE = 0x80;
        const SUBESSIVE = 0x100;
        const SUPERESSIVE = 0x200;
        const ABLATIVE = 0x400;
        const ADELATIVE = 0x800;
        const DELATIVE = 0x1000;
        const EGRESSIVE = 0x2000;
        const ELATIVE = 0x4000;
        const INITIATIVE = 0x8000;
        const POSTELATIVE = 0x10000;
        const ALLATIVE = 0x20000;
        const ILLATIVE = 0x40000;
        const LATIVE = 0x80000;
        const SUBLATIVE = 0x100000;
        const SUPERLATIVE = 0x200000;
        const TERMINATIVE = 0x400000;
        const PERLATIVE = 0x800000;
        const PROLATIVE = 0x1000000;
        const ACCUSATIVE = 0x2000000;
        const ESSIVE = 0x4000000;
        const LIMITATIVE = 0x8000000;
        const TEMPORAL = 0x10000000;
        const ABSOLUTIVE = 0x20000000;
        const AGENTIVE = 0x40000000;
        const DIRECT = 0x80000000;
        const ERGATIVE = 0x100000000;
        const INSTRUCTIVE = 0x200000000;
        const INSTRUMENTAL = 0x400000000;
        const NOMINATIVE = 0x800000000;
        const OBLIQUE = 0x1000000000;
        const INTRANSITIVE = 0x2000000000;
        const PEGATIVE = 0x4000000000;
        const AVERSISVE = 0x8000000000;
        const BENEFACTIVE = 0x10000000000;
        const CARITATIVE = 0x20000000000;
        const CAUSAL = 0x40000000000;
        const COMITATIVE = 0x80000000000;
        const DATIVE = 0x100000000000;
        const DISTRIBUTIVE = 0x200000000000;
        const GENITIVE = 0x400000000000;
        const ORNATIVE = 0x800000000000;
        const POSSESSED = 0x1000000000000;
        const POSSESSIVE = 0x2000000000000;
        const ABESSIVE = 0x4000000000000;
        const SEMBLATIVE = 0x8000000000000;
        const SOCIATIVE = 0x10000000000000;
        const SUBSTITUTIVE = 0x20000000000000;
        const PARTITIVE = 0x40000000000000;
        const PREPOSITIONAL = 0x80000000000000;
        const VOCATIVE = 0x100000000000000;
        const ADVERBIAL = 0x200000000000000;
        const COMPARATIVE = 0x400000000000000;
        const EXESSIVE = 0x800000000000000;
        const ORIENTATIVE = 0x1000000000000000;
        const REVERTIVE = 0x2000000000000000;
        const TRANSLATIVE = 0x4000000000000000;
        const REFLEXIVE = 0x8000000000000000;
    }
}

enum Finitivity {
    NONE,
    DEFINITE_ARTICLES_ONLY,
    INDEFINITE_ARTICLES_ONLY,
    DEFINITE_AND_INDEFINITE_ARTICLES,
    DEFINITE_DECLENSION_ONLY,
    INDEFINITE_DECLENSION_ONLY,
    DEFINITE_AND_INDEFINITE_DECLENSION
}

enum Accent {
    NONE,
    CONTRASTIVE_PITCH,
    CONTRASTIVE_STRESS,
    SYSTEMIC_PITCH,
    SYSTEMIC_STRESS
}