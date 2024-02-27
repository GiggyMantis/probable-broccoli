use serde::{Serialize, Deserialize};
use serde_arrays;
use std::{cell::RefCell, rc::Rc, fs};
use std::fs::read_to_string;
use crate::model::BinaryTree;

pub mod compare;
pub mod ipa_mapping;
pub mod model;

fn main() {
    let mut model = BinaryTree::from("Languages", vec!["Old_English", "Latin", "Old_Frisian", "Ancient_Greek", "Gothic", "Old_Norse", "Modern_Dutch", "Afrikaans", "Old_High_German", "Old_Polish", "Old_Irish", "Old_Czech", "Generic_Romani"]);

    model.naive_minimum_distance_model();


    println!("{}", model.get_debug_representation());
    //println!("{}", serde_json::to_string_pretty(&lang.clone()).unwrap());

    //println!("{}", MOOD_INDICATIVE + MOOD_CONDITIONAL + MOOD_IMPERATIVE)
}
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
}

impl TreeNodeTrait for TreeNodeRef {
    fn val(&self) -> Option<Box<Languoid>> {
        <RefCell<TreeNode> as Clone>::clone(self).into_inner().val
    }
    fn right(&self) -> Option<TreeNodeRef> {
        <RefCell<TreeNode> as Clone>::clone(self).into_inner().right
    }
    fn left(&self) -> Option<TreeNodeRef> {
        <RefCell<TreeNode> as Clone>::clone(self).into_inner().left
    }
    fn year(&self) -> i32 {
        <RefCell<TreeNode> as Clone>::clone(self).into_inner().year
    }

}

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
const CASE_ADESSIVE: u64 = 0x1;
const CASE_ANTESSIVE: u64 = 0x2;
const CASE_ADUPESSIVE: u64 = 0x4;
const CASE_INESSIVE: u64 = 0x8;
const CASE_INTRATIVE: u64 = 0x10;
const CASE_LOCATIVE: u64 = 0x20;
const CASE_PERTINGENT: u64 = 0x40;
const CASE_POSTESSIVE: u64 = 0x80;
const CASE_SUBESSIVE: u64 = 0x100;
const CASE_SUPERESSIVE: u64 = 0x200;
const CASE_ABLATIVE: u64 = 0x400;
const CASE_ADELATIVE: u64 = 0x800;
const CASE_DELATIVE: u64 = 0x1000;
const CASE_EGRESSIVE: u64 = 0x2000;
const CASE_ELATIVE: u64 = 0x4000;
const CASE_INITIATIVE: u64 = 0x8000;
const CASE_POSTELATIVE: u64 = 0x10000;
const CASE_ALLATIVE: u64 = 0x20000;
const CASE_ILLATIVE: u64 = 0x40000;
const CASE_LATIVE: u64 = 0x80000;
const CASE_SUBLATIVE: u64 = 0x100000;
const CASE_SUPERLATIVE: u64 = 0x200000;
const CASE_TERMINATIVE: u64 = 0x400000;
const CASE_PERLATIVE: u64 = 0x800000;
const CASE_PROLATIVE: u64 = 0x1000000;
const CASE_ACCUSATIVE: u64 = 0x2000000;
const CASE_ESSIVE: u64 = 0x4000000;
const CASE_LIMITATIVE: u64 = 0x8000000;
const CASE_TEMPORAL: u64 = 0x10000000;
const CASE_ABSOLUTIVE: u64 = 0x20000000;
const CASE_AGENTIVE: u64 = 0x40000000;
const CASE_DIRECT: u64 = 0x80000000;
const CASE_ERGATIVE: u64 = 0x100000000;
const CASE_INSTRUCTIVE: u64 = 0x200000000;
const CASE_INSTRUMENTAL: u64 = 0x400000000;
const CASE_NOMINATIVE: u64 = 0x800000000;
const CASE_OBLIQUE: u64 = 0x1000000000;
const CASE_INTRANSITIVE: u64 = 0x2000000000;
const CASE_PEGATIVE: u64 = 0x4000000000;
const CASE_AVERSISVE: u64 = 0x8000000000;
const CASE_BENEFACTIVE: u64 = 0x10000000000;
const CASE_CARITATIVE: u64 = 0x20000000000;
const CASE_CAUSAL: u64 = 0x40000000000;
const CASE_COMITATIVE: u64 = 0x80000000000;
const CASE_DATIVE: u64 = 0x100000000000;
const CASE_DISTRIBUTIVE: u64 = 0x200000000000;
const CASE_GENITIVE: u64 = 0x400000000000;
const CASE_ORNATIVE: u64 = 0x800000000000;
const CASE_POSSESSED: u64 = 0x1000000000000;
const CASE_POSSESSIVE: u64 = 0x2000000000000;
const CASE_ABESSIVE: u64 = 0x4000000000000;
const CASE_SEMBLATIVE: u64 = 0x8000000000000;
const CASE_SOCIATIVE: u64 = 0x10000000000000;
const CASE_SUBSTITUTIVE: u64 = 0x20000000000000;
const CASE_PARTITIVE: u64 = 0x40000000000000;
const CASE_PREPOSITIONAL: u64 = 0x80000000000000;
const CASE_VOCATIVE_OR_EMPHATIC: u64 = 0x100000000000000;
const CASE_ADVERBIAL: u64 = 0x200000000000000;
const CASE_COMPARATIVE: u64 = 0x400000000000000;
const CASE_EXESSIVE: u64 = 0x800000000000000;
const CASE_ORIENTATIVE: u64 = 0x1000000000000000;
const CASE_REVERTIVE: u64 = 0x2000000000000000;
const CASE_TRANSLATIVE: u64 = 0x4000000000000000;
const CASE_REFLEXIVE: u64 = 0x8000000000000000;
// Cases End
// Aspects Start
const ASPECT_PERFECTIVE: u32 = 1;
const ASPECT_MOMENTANE: u32 = 2;
const ASPECT_PERFECT: u32 = 4;
const ASPECT_RECENT_PERFECT: u32 = 8;
const ASPECT_IMPERFECT: u32 = 16;
const ASPECT_PROSPECTIVE: u32 = 32;
const ASPECT_IMPERFECTIVE: u32 = 64;
const ASPECT_HABITUAL: u32 = 128;
const ASPECT_CONTINUOUS: u32 = 256;
const ASPECT_PROGRESSIVE: u32 = 512;
const ASPECT_STATIVE: u32 = 1024;
const ASPECT_GNOMIC: u32 = 2048;
const ASPECT_EPISODIC: u32 = 4096;
const ASPECT_CONTINUATIVE: u32 = 8192;
const ASPECT_INGRESSIVE: u32 = 0x4000;
const ASPECT_INCHOACTIVE: u32 = 0x8000;
const ASPECT_CESSATIVE: u32 = 0x10000;
const ASPECT_DEFECTIVE: u32 = 0x20000;
const ASPECT_PAUSATIVE: u32 = 0x40000;
const ASPECT_RESUMPTIVE: u32 = 0x80000;
const ASPECT_PUNCTUAL: u32 = 0x100000;
const ASPECT_DURATIVE: u32 = 0x200000;
const ASPECT_PROTRACTIVE: u32 = 0x400000;
const ASPECT_ITERATIVE: u32 = 0x800000;
const ASPECT_FREQUENTIVE: u32 = 0x1000000;
const ASPECT_EXPERENTIAL: u32 = 0x2000000;
const ASPECT_INTENTIONAL: u32 = 0x4000000;
const ASPECT_ACCIDENTAL: u32 = 0x8000000;
const ASPECT_INTENSIVE: u32 = 0x10000000;
const ASPECT_MODERATIVE: u32 = 0x20000000;
const ASPECT_ATTENUATIVE: u32 = 0x40000000;
const ASPECT_SEGMENTATIVE: u32 = 0x80000000;
// Aspects End
// Moods Start
const MOOD_INDICATIVE: u32 = 0x1;
const MOOD_ENERGETIC: u32 = 0x2;
const MOOD_DECLARATIVE: u32 = 0x4;
const MOOD_SUBJUNCTIVE: u32 = 0x8;
const MOOD_CONDITIONAL: u32 = 0x10;
const MOOD_OPTATIVE: u32 = 0x20;
const MOOD_JUSSIVE: u32 = 0x40;
const MOOD_POTENTIAL: u32 = 0x80;
const MOOD_IMPERATIVE: u32 = 0x100;
const MOOD_PROHIBITIVE: u32 = 0x200;
const MOOD_DESIDERATIVE: u32 = 0x400;
const MOOD_DUBITATIVE: u32 = 0x800;
const MOOD_HYPOTHETICAL: u32 = 0x1000;
const MOOD_PRESUMPTIVE: u32 = 0x2000;
const MOOD_PERMISSIVE: u32 = 0x4000;
const MOOD_ADMIRATIVE: u32 = 0x8000;
const MOOD_HORTATIVE: u32 = 0x10000;
const MOOD_PRECATIVE_VOLITIVE: u32 = 0x20000;
const MOOD_INFERENTIAL: u32 = 0x40000;
const MOOD_NECESSITATIVE: u32 = 0x80000;
const MOOD_INTERROGATIVE: u32 = 0x100000;
const MOOD_BENEDICTIVE: u32 = 0x200000;
const MOOD_CONCESSIVE: u32 = 0x400000;
const MOOD_PRESCRIPTIVE: u32 = 0x800000;
// Moods End
// Number Start
const NUM_SINGULAR: u8 = 0x1;
const NUM_DUAL: u8 = 0x2;
const NUM_TRIAL: u8 = 0x4;
const NUM_PAUCAL: u8 = 0x8;
const NUM_PLURAL: u8 = 0x10;
const NUM_QUADRAL: u8 = 0x20;
const NUM_SUPERPLURAL: u8 = 0x40;
const NUM_DISTRIBUTIVE_PLURAL: u8 = 0x80;
// Number End
// Finitivity Start
const FINITIVITY_HAS_ARTICLES: u8 = 0x1;
const FINITIVITY_DEFINITE: u8 = 0x2;
const FINITIVITY_INDEFINITE: u8 = 0x4;
const FINITIVITY_PARTITIVE: u8 = 0x8;
const FINITIVITY_NEGATIVE: u8 = 0x10;
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