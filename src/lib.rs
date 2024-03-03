#![crate_name = "probable_broccoli"]

pub mod compare;
pub mod ipa_mapping;
pub mod model;
pub mod dendrogram;

use serde::{Serialize, Deserialize};
use serde_arrays;
use std::{cell::RefCell, rc::Rc, fs, fs::read_to_string, collections::HashMap, collections};
use std::hash::{Hash, Hasher};
use crate::model::BinaryTree;
use crate::dendrogram::ConnectionTuple;

#[derive(Debug, Clone)]
pub struct TreeNode {
    val: Option<Box<Languoid>>,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
    year: i32,
}
type TreeNodeRef = Rc<RefCell<TreeNode>>;

trait TreeNodeTrait {
    fn val(&self) -> Option<Box<Languoid>>;
    fn right(&self) -> Option<TreeNodeRef>;
    fn left(&self) -> Option<TreeNodeRef>;
    fn year(&self) -> i32;
    fn check_for_loops(&self) -> bool;
    fn is_family(&self) -> bool;
    fn get_children(&self) -> (Option<(String, i32)>, Option<TreeNodeRef>, Option<TreeNodeRef>);
}

impl TreeNodeTrait for TreeNodeRef {
    /// Returns the Languoid, if a languoid is associated with this node at all.
    fn val(&self) -> Option<Box<Languoid>> {
        <RefCell<TreeNode> as Clone>::clone(self).into_inner().val
    }
    /// Returns the TreeNodeRef that is down and right in the binary tree.
    fn right(&self) -> Option<TreeNodeRef> {
        <RefCell<TreeNode> as Clone>::clone(self).into_inner().right
    }
    /// Returns the TreeNodeRef that is down and left in the binary tree.
    fn left(&self) -> Option<TreeNodeRef> {
        <RefCell<TreeNode> as Clone>::clone(self).into_inner().left
    }
    /// Returns the year of thre TreeNodeRef.
    fn year(&self) -> i32 {
        <RefCell<TreeNode> as Clone>::clone(self).into_inner().year
    }

    /// Returns true if the TreeNodeRef is recursive at any point in its branch.
    /// This helps prevent memory leaks.
    fn check_for_loops(&self) -> bool {
       // TODO: Implement this
        false
    }

    /// Returns true i the TreeNodeRef is a family, false if it is a lone languoid. 
    fn is_family(&self) -> bool {
        if self.val().is_some() {
            false
        } else {
            true
        }
    }

    /// Returns a tuple of the names and children of a TreeNode
    fn get_children(&self) -> (Option<(String, i32)>, Option<TreeNodeRef>, Option<TreeNodeRef>) {
    (
        if self.val().is_some() {
            Some((self.val().unwrap().languoid_name, self.val().unwrap().year))
        } else {
            None
        },
        self.left(), self.right()
    )
}

}

/// Creates a *new* TreeNodeRef (not already in the tree) using the languoid *l*.
fn get_node_from_languoid(l: Box<Languoid>) -> TreeNodeRef {
    return Rc::new(RefCell::new(TreeNode {
        year: l.year.clone(),
        val: Some(l),
        left: None,
        right: None,
    }));
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Languoid {
    languoid_name: String,
    year: i32,
    #[serde(with = "serde_arrays")]
    leipzig_jakarta_list: [(String, usize); 100], // usize is index of noun class
    grammar: Grammar,
    phonology: Phonology,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grammar {
    predicate_word_order: String,
    adjective_before_noun: bool,
    prepositions: bool,
    pronominal_cases: u64,
    nominal_cases: u64,
    pronominal_numbers: u8,
    nominal_numbers: u8,
    numeric_base: u8,
    finitivity: u8,
    determiner_before_noun: bool,
    noun_classes: Vec<NounClass>,
    pronoun_classes: Vec<NounClass>, // Note to not include neologisms or neopronouns unless they are in common usage, as they do not indicate language relationships.
    has_inclusive_exclusive_distinction: bool,
    declensions: bool,
    personal_conjugation: bool,
    third_person_personal_pronouns: bool,
    formality: bool,
    contraction: bool,
    obligate_contraction: bool,
    explicit_copula: bool,
    ser_estar_distinction: bool,
    part_of_speech_morphology: bool,
    tenses: u8,
    aspect: u32,
    mood: u32,
    has_verbal_voice: bool,
    double_negatives_are_positive: bool,
    reduplication: bool,
    has_augmentative: bool,
    has_diminutive: bool,
    evidentials: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phonology {
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
    has_palatal: bool,
    has_labial: bool,
    has_velar: bool,
    has_pharyngeal_or_epiglottal: bool,
    has_vibrants: bool,
    gemination: bool,
    palatalization: bool,
    velarization: bool,
    labialization: bool,
    emphatics: bool,
    pub mora_timed: bool,
}

fn array_str_to_nongendered_array_string(a: [&str; 100]) -> [(String, usize); 100] {
    a.iter().map(|&x| (x.to_string(), 0)).collect::<Vec<(String, usize)>>().try_into().unwrap()
}

fn array_str_to_gendered_array_string(a: [(&str, usize); 100]) -> [(String, usize); 100] {
    a.iter().map(|&x| (x.0.to_string(), x.1)).collect::<Vec<(String, usize)>>().try_into().unwrap()
}

// #Constants

// Cases Start
pub const _CASE_ADESSIVE: u64 = 0x1;
pub const _CASE_ANTESSIVE: u64 = 0x2;
pub const _CASE_ADUPESSIVE: u64 = 0x4;
pub const _CASE_INESSIVE: u64 = 0x8;
pub const _CASE_INTRATIVE: u64 = 0x10;
pub const _CASE_LOCATIVE: u64 = 0x20;
pub const _CASE_PERTINGENT: u64 = 0x40;
pub const _CASE_POSTESSIVE: u64 = 0x80;
pub const _CASE_SUBESSIVE: u64 = 0x100;
pub const _CASE_SUPERESSIVE: u64 = 0x200;
pub const _CASE_ABLATIVE: u64 = 0x400;
pub const _CASE_ADELATIVE: u64 = 0x800;
pub const _CASE_DELATIVE: u64 = 0x1000;
pub const _CASE_EGRESSIVE: u64 = 0x2000;
pub const _CASE_ELATIVE: u64 = 0x4000;
pub const _CASE_INITIATIVE: u64 = 0x8000;
pub const _CASE_POSTELATIVE: u64 = 0x10000;
pub const _CASE_ALLATIVE: u64 = 0x20000;
pub const _CASE_ILLATIVE: u64 = 0x40000;
pub const _CASE_LATIVE: u64 = 0x80000;
pub const _CASE_SUBLATIVE: u64 = 0x100000;
pub const _CASE_SUPERLATIVE: u64 = 0x200000;
pub const _CASE_TERMINATIVE: u64 = 0x400000;
pub const _CASE_PERLATIVE: u64 = 0x800000;
pub const _CASE_PROLATIVE: u64 = 0x1000000;
pub const _CASE_ACCUSATIVE: u64 = 0x2000000;
pub const _CASE_ESSIVE: u64 = 0x4000000;
pub const _CASE_LIMITATIVE: u64 = 0x8000000;
pub const _CASE_TEMPORAL: u64 = 0x10000000;
pub const _CASE_ABSOLUTIVE: u64 = 0x20000000;
pub const _CASE_AGENTIVE: u64 = 0x40000000;
pub const _CASE_DIRECT: u64 = 0x80000000;
pub const _CASE_ERGATIVE: u64 = 0x100000000;
pub const _CASE_INSTRUCTIVE: u64 = 0x200000000;
pub const _CASE_INSTRUMENTAL: u64 = 0x400000000;
pub const _CASE_NOMINATIVE: u64 = 0x800000000;
pub const _CASE_OBLIQUE: u64 = 0x1000000000;
pub const _CASE_INTRANSITIVE: u64 = 0x2000000000;
pub const _CASE_PEGATIVE: u64 = 0x4000000000;
pub const _CASE_AVERSISVE: u64 = 0x8000000000;
pub const _CASE_BENEFACTIVE: u64 = 0x10000000000;
pub const _CASE_CARITATIVE: u64 = 0x20000000000;
pub const _CASE_CAUSAL: u64 = 0x40000000000;
pub const _CASE_COMITATIVE: u64 = 0x80000000000;
pub const _CASE_DATIVE: u64 = 0x100000000000;
pub const _CASE_DISTRIBUTIVE: u64 = 0x200000000000;
pub const _CASE_GENITIVE: u64 = 0x400000000000;
pub const _CASE_ORNATIVE: u64 = 0x800000000000;
pub const _CASE_POSSESSED: u64 = 0x1000000000000;
pub const _CASE_POSSESSIVE: u64 = 0x2000000000000;
pub const _CASE_ABESSIVE: u64 = 0x4000000000000;
pub const _CASE_SEMBLATIVE: u64 = 0x8000000000000;
pub const _CASE_SOCIATIVE: u64 = 0x10000000000000;
pub const _CASE_SUBSTITUTIVE: u64 = 0x20000000000000;
pub const _CASE_PARTITIVE: u64 = 0x40000000000000;
pub const _CASE_ADPOSITIONAL: u64 = 0x80000000000000;
pub const _CASE_VOCATIVE_OR_EMPHATIC: u64 = 0x100000000000000;
pub const _CASE_ADVERBIAL: u64 = 0x200000000000000;
pub const _CASE_COMPARATIVE: u64 = 0x400000000000000;
pub const _CASE_EXESSIVE: u64 = 0x800000000000000;
pub const _CASE_ORIENTATIVE: u64 = 0x1000000000000000;
pub const _CASE_REVERTIVE: u64 = 0x2000000000000000;
pub const _CASE_TRANSLATIVE: u64 = 0x4000000000000000;
pub const _CASE_REFLEXIVE: u64 = 0x8000000000000000;
// Cases End
// Aspects Start
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
pub const _ASPECT_INGRESSIVE: u32 = 0x4000;
pub const _ASPECT_INCHOACTIVE: u32 = 0x8000;
pub const _ASPECT_CESSATIVE: u32 = 0x10000;
pub const _ASPECT_DEFECTIVE: u32 = 0x20000;
pub const _ASPECT_PAUSATIVE: u32 = 0x40000;
pub const _ASPECT_RESUMPTIVE: u32 = 0x80000;
pub const _ASPECT_PUNCTUAL: u32 = 0x100000;
pub const _ASPECT_DURATIVE: u32 = 0x200000;
pub const _ASPECT_PROTRACTIVE: u32 = 0x400000;
pub const _ASPECT_ITERATIVE: u32 = 0x800000;
pub const _ASPECT_FREQUENTIVE: u32 = 0x1000000;
pub const _ASPECT_EXPERENTIAL: u32 = 0x2000000;
pub const _ASPECT_INTENTIONAL: u32 = 0x4000000;
pub const _ASPECT_ACCIDENTAL: u32 = 0x8000000;
pub const _ASPECT_INTENSIVE: u32 = 0x10000000;
pub const _ASPECT_MODERATIVE: u32 = 0x20000000;
pub const _ASPECT_ATTENUATIVE: u32 = 0x40000000;
pub const _ASPECT_SEGMENTATIVE: u32 = 0x80000000;
// Aspects End
// Moods Start
pub const _MOOD_INDICATIVE: u32 = 0x1;
pub const _MOOD_ENERGETIC: u32 = 0x2;
pub const _MOOD_DECLARATIVE: u32 = 0x4;
pub const _MOOD_SUBJUNCTIVE: u32 = 0x8;
pub const _MOOD_CONDITIONAL: u32 = 0x10;
pub const _MOOD_OPTATIVE: u32 = 0x20;
pub const _MOOD_JUSSIVE: u32 = 0x40;
pub const _MOOD_POTENTIAL: u32 = 0x80;
pub const _MOOD_IMPERATIVE: u32 = 0x100;
pub const _MOOD_PROHIBITIVE: u32 = 0x200;
pub const _MOOD_DESIDERATIVE: u32 = 0x400;
pub const _MOOD_DUBITATIVE: u32 = 0x800;
pub const _MOOD_HYPOTHETICAL: u32 = 0x1000;
pub const _MOOD_PRESUMPTIVE: u32 = 0x2000;
pub const _MOOD_PERMISSIVE: u32 = 0x4000;
pub const _MOOD_ADMIRATIVE: u32 = 0x8000;
pub const _MOOD_HORTATIVE: u32 = 0x10000;
pub const _MOOD_PRECATIVE_VOLITIVE: u32 = 0x20000;
pub const _MOOD_INFERENTIAL: u32 = 0x40000;
pub const _MOOD_NECESSITATIVE: u32 = 0x80000;
pub const _MOOD_INTERROGATIVE: u32 = 0x100000;
pub const _MOOD_BENEDICTIVE: u32 = 0x200000;
pub const _MOOD_CONCESSIVE: u32 = 0x400000;
pub const _MOOD_PRESCRIPTIVE: u32 = 0x800000;
// Moods End
// Number Start
pub const _NUM_SINGULAR: u8 = 0x1;
pub const _NUM_DUAL: u8 = 0x2;
pub const _NUM_TRIAL: u8 = 0x4;
pub const _NUM_PAUCAL: u8 = 0x8;
pub const _NUM_PLURAL: u8 = 0x10;
pub const _NUM_QUADRAL: u8 = 0x20;
pub const _NUM_SUPERPLURAL: u8 = 0x40;
pub const _NUM_DISTRIBUTIVE_PLURAL: u8 = 0x80;
// Number End
// Finitivity Start
pub const _FINITIVITY_HAS_ARTICLES: u8 = 0x1;
pub const _FINITIVITY_DEFINITE: u8 = 0x2;
pub const _FINITIVITY_INDEFINITE: u8 = 0x4;
pub const _FINITIVITY_PARTITIVE: u8 = 0x8;
pub const _FINITIVITY_NEGATIVE: u8 = 0x10;
// Finitivity End

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Accent {
    None,
    ContrastivePitch,
    ContrastiveStress,
    SystemicPitch,
    SystemicStress,
}

pub type NounClass = Vec<NounClassCatergories>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
enum NounClassCatergories {
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
    Other(u8),
}