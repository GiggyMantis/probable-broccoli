use bitflags::bitflags;
use std::{cell::RefCell, rc::Rc};
pub mod compare;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: Option<Box<Languoid>>,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}
type TreeNodeRef = Rc<RefCell<TreeNode>>;

fn get_node_from_languoid(l: Box<Languoid>) -> TreeNode {
    TreeNode {
        val: SOME(l),
        left: NONE,
        right: NONE,
    };
}

struct Languoid {
    languoid_name: String,
    year: i32,
    leipzig_jakarta_list: [String, 100],
    grammar: Grammar,
    phonology: Phonology,
}

struct Grammar {
    predicate_word_order: PredicateWordOrder,
    adjective_before_noun: bool,
    prepositions: bool,
    pronominal_cases: u64,
    nominal_cases: u64,
    pronominal_numbers: u8,
    nominal_numbers: u8,
    finitivity: u8,
    determiner_before_noun: bool,
    word_classes: Vec<String>,
    declensions: bool,
    peronal_conjugation: bool,
    leipzig_jakarta_word_classes: [String, 100],
    third_person_personal_pronouns: bool,
    formality: bool,
    contraction: bool,
    obligate_contraction: bool,
    copula: Copula,
    part_of_speech_morphology: bool,
    tenses: u8,
    aspect: u32,
    mood: u32,
    has_verbal_voice: bool,
    double_negatives_are_positive: bool,
    reduplication: bool,
    has_augmentative: bool,
    has_diminuative: bool,
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
    emphatics: bool,
}

enum PredicateWordOrder {
    SVO,
    SOV,
    VSO,
    VOS,
    OSV,
    OVS,
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

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct GrammaticalAspect: u32 {
        const PERFECTIVE = 0x1;
        const MOMENTANE = 0x2;
        const PERFECT = 0x4;
        const RECENT_PERECT = 0x8;
        const DISCONTINOUS_PAST = 0x10;
        const PROSPECTIVE = 0x20;
        const IMPERFECTIVE = 0x40;
        const HABITUAL = 0x80;
        const CONTINUOUS = 0x100;
        const PROGRESSIVE = 0x200;
        const STATIVE = 0x400;
        const GNOMIC = 0x800;
        const EPISODIC = 0x1000;
        const CONTINUATIVE = 0x2000;
        const INGRESSIVE = 0x4000;
        const INCHOACTIVE = 0x8000;
        const CESSATIVE = 0x10000;
        const DEFECTIVE = 0x20000;
        const PAUSATIVE = 0x40000;
        const RESUMPTIVE = 0x80000;
        const PUNCTUAL = 0x100000;
        const DURATIVE = 0x200000;
        const PROTRACTIVE = 0x400000;
        const ITERATIVE = 0x800000;
        const FREQUENTIVE = 0x1000000;
        const EXPERENTIAL = 0x2000000;
        const INTENTIONAL = 0x4000000;
        const ACCIDENTAL = 0x8000000;
        const INTENSIVE = 0x10000000;
        const MODERATIVE = 0x20000000;
        const ATTENUATIVE = 0x40000000;
        const SEGMENTATIVE = 0x80000000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct GrammaticalMood: u32 {
        const INDICATIVE = 0x1;
        const ENERGETIC = 0x2;
        const DECLARATIVE = 0x4;
        const SUBJUNCTIVE = 0x8;
        const CONDITIONAL = 0x10;
        const OPTATIVE = 0x20;
        const JUSSIVE = 0x40;
        const POTENTIAL = 0x80;
        const IMPERATIVE = 0x100;
        const PROHIBITIVE = 0x200;
        const DESIDERATIVE = 0x400;
        const DUBITATIVE = 0x800;
        const HYPOTHETICAL = 0x1000;
        const PRESUMPTIVE = 0x2000;
        const PERMISSIVE = 0x4000;
        const ADMIRATIVE = 0x8000;
        const HORTATIVE = 0x10000;
        const PRECATIVE_VOLITIVE = 0x20000;
        const INFERENTIAL = 0x40000;
        const NECESSITATIVE = 0x80000;
        const INTERROGATIVE = 0x100000;
        const BENEDICTIVE = 0x200000;
        const CONCESSIVE = 0x400000;
        const PRESCRIPTIVE = 0x800000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct GrammaticalNumber: u8 {
        const SINGULAR = 0x1;
        const DUAL = 0x2;
        const TRIAL = 0x4;
        const PAUCAL = 0x8;
        const PLURAL = 0x10;
        const QUADRAL = 0x20;
        const SUPERPLURAL = 0x40;
        const DISTRIBUTIVE_PLURAL = 0x80;

    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Finitivity: u8 {
        const HAS_ARTICLES = 0x1;
        const DEFINITE = 0x2;
        const INDEFINITE = 0x4;
        const PARTITIVE = 0x8;
    }
}

enum Accent {
    NONE,
    CONTRASTIVE_PITCH,
    CONTRASTIVE_STRESS,
    SYSTEMIC_PITCH,
    SYSTEMIC_STRESS,
}

enum Copula {
    IMPLICIT,
    DROPPING,
    ONE,
    MULTIPLE,
}