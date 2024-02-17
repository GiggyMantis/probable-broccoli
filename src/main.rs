use serde::{Serialize, Deserialize};
use serde_arrays;
use std::{cell::RefCell, rc::Rc};
use crate::model::BinaryTree;
use crate::NounClassCatergories::{Epicene, Feminine, Masculine};

pub mod compare;
pub mod ipa_mapping;
pub mod model;

fn main() {

    let lang: Languoid = Languoid {
        languoid_name: "English".to_string(),
        year: 2024,
        leipzig_jakarta_list: array_str_to_nongendered_array_string([
            "fˈajɚ",
            "nowz",
            "ɡow",
            "wˈɑɾɚ",
            "mawθ",
            "tʰəŋ",
            "bɫəd",
            "bown",
            "jʉw",
            "ɹʷʉwʔ",
            "kʰəm",
            "bɹʷɛst",
            "ɹʷɛjn",
            "aj",
            "nɛjm",
            "laws",
            "wɪŋ",
            "fɫɛʃʷ",
            "ɑɹʷm",
            "fɫaj",
            "najʔ",
            "ɪjɹʷ",
            "nɛkʰ",
            "fɑɹʷ",
            "dʉw",
            "haws",
            "stown",
            "bˈɪɾɚ",
            "sɛj",
            "tʰʉwθ",
            "hɛjɹʷ",
            "bɪɡ",
            "wən",
            "hʉw",
            "çɪj",
            "hɪʔ",
            "lɛɡ",
            "hoɹʷn",
            "ðɪs",
            "fɪʃʷ",
            "jˈɛstɝdaj",
            "ʤɹʷɪŋkʰ",
            "bɫækʰ",
            "nˈɛjvʟ",
            "stænd",
            "bajʔ",
            "bækʰ",
            "wɪnd",
            "smowkʰ",
            "wəʔ",
            "ʧʷʰajɫd",
            "ɛɡ",
            "ɡɪv",
            "nʉw",
            "bɝn",
            "nɑʔ",
            "ɡʊd",
            "now",
            "nɪj",
            "sænd",
            "læf",
            "çɪjɹʷ",
            "sˈɔjʟ",
            "lɪjf",
            "ɹʷɛd",
            "lˈɪvɚ",
            "hajd",
            "skɪn",
            "səkʰ",
            "kˈɛɹʷɪj",
            "ænʔ",
            "hˈɛvɪj",
            "tʰɛjkʰ",
            "owɫd",
            "ɪjʔ",
            "θaj",
            "θɪkʰ",
            "lɑŋ",
            "bɫow",
            "wʊd",
            "ɹʷən",
            "fɑʟ",
            "aj",
            "æʃʷ",
            "tʰɛjʟ",
            "dɑɡ",
            "kʰɹʷaj",
            "tʰaj",
            "sɪj",
            "swɪjʔ",
            "ɹʷowpʰ",
            "ʃʷˈædow",
            "bɝd",
            "sɑɫʔ",
            "smɑʟ",
            "wajd",
            "stɑɹʷ",
            "ɪn",
            "hɑɹʷd",
            "kʰɹʷəʃʷ"
        ]),
        grammar: Grammar {
            predicate_word_order: "SVO".to_string(),
            adjective_before_noun: true,
            prepositions: true,
            pronominal_cases: CASE_NOMINATIVE | CASE_ACCUSATIVE | CASE_REFLEXIVE | CASE_GENITIVE | CASE_POSSESSIVE,
            nominal_cases: CASE_NOMINATIVE | CASE_POSSESSIVE,
            pronominal_numbers: NUM_SINGULAR | NUM_PLURAL,
            nominal_numbers: NUM_SINGULAR | NUM_PLURAL,
            numeric_base: 10,
            finitivity: FINITIVITY_HAS_ARTICLES | FINITIVITY_DEFINITE | FINITIVITY_INDEFINITE | FINITIVITY_PARTITIVE,
            determiner_before_noun: true,
            noun_classes: vec![vec![NounClassCatergories::None]],
            pronoun_classes: vec![vec![NounClassCatergories::Feminine], vec![NounClassCatergories::Masculine], vec![NounClassCatergories::Epicene]],
            has_inclusive_exclusive_distinction: false,
            declensions: false,
            personal_conjugation: true,
            third_person_personal_pronouns: true,
            formality: false,
            contraction: true,
            obligate_contraction: false,
            explicit_copula: true,
            ser_estar_distinction: false,
            part_of_speech_morphology: false,
            tenses: 2,
            aspect: ASPECT_PERFECT | ASPECT_CONTINUOUS | ASPECT_PERFECTIVE | ASPECT_IMPERFECT,
            mood: MOOD_INDICATIVE | MOOD_SUBJUNCTIVE | MOOD_INTERROGATIVE,
            evidentials: 0,
            has_verbal_voice: true,
            double_negatives_are_positive: true,
            reduplication: false,
            has_augmentative: false,
            has_diminutive: true,
        },
        phonology: Phonology {
            tone_count: 0,
            vowel_length: false,
            nasal_vowels: false,
            rhotic_vowels: true,
            front_rounded_vowels: false,
            back_unrounded_vowels: false,
            schwa: true,
            reduction: true,
            accent_system: Accent::ContrastiveStress,
            mora_timed: false,
            plosive_series_count: 4,
            fricative_voicedness_distinction: true,
            nasal_voicedness_distinction: false,
            has_laterals: true,
            has_phonemic_diphthongs: true,
            has_phonetic_diphthongs: true,
            basic_vowel_count: 13,
            has_glottal: true,
            has_uvular: false,
            has_labiodental: true,
            has_linguodental: true,
            has_retroflex_or_postalveolar: true,
            has_palatal: true,
            has_labial: true,
            has_velar: true,
            has_pharyngeal_or_epiglottal: false,
            has_vibrants: true,
            gemination: false,
            palatalization: false,
            velarization: false,
            labialization: false,
            emphatics: false,
        },
    };
    let lang_2: Languoid = Languoid {
        languoid_name: "Latin".to_string(),
        year: -75,
        leipzig_jakarta_list: array_str_to_gendered_array_string([
            ("ˈɪŋnɪs", 2),
            ("nˈaːsʊs", 2),
            ("ˈiːrɛ", 0),
            ("ˈakʷa", 1),
            ("oːs", 3),
            ("dˈɪŋɡʷa", 1),
            ("sˈaŋɡʷiːs", 2),
            ("ɔs", 3),
            ("tuː", 0),
            ("rˈaːdiːks", 1),
            ("wɛnˈiːrɛ", 0),
            ("mˈamma", 1),
            ("pɫˈʊwiä", 1),
            ("ˈɛɡoː", 0),
            ("nˈoːmɛn", 3),
            ("pˈeːdɪs", 2),
            ("ˈaːɫa", 1),
            ("kˈarnɪs", 1),
            ("brˈakkʰiʊm", 3),
            ("mˈʊska", 1),
            ("nɔks", 1),
            ("ˈawrɪs", 1),
            ("kˈɛrwiːks", 1),
            ("prˈɔkʊɫ", 0),
            ("fˈakɛrɛ", 0),
            ("dˈɔmʊs", 1),
            ("ˈɫapɪs", 2),
            ("amˈaːrʊs", 0),
            ("dˈiːkɛrɛ", 0),
            ("dɛns", 2),
            ("pˈɪɫʊs", 2),
            ("mˈaŋnʊs", 0),
            ("ˈojnos", 0),
            ("kʷɪd", 3),
            ("ɪd", 3),
            ("fˈɛriːrɛ", 0),
            ("kruːs", 3),
            ("kˈɔrnuː", 3),
            ("hɔk", 3),
            ("pˈɪskɪs", 2),
            ("hˈɛriː", 0),
            ("bˈɪbɛrɛ", 0),
            ("nˈɪɡɛr", 0),
            ("ʊmbɪlʲˈiːkʊs", 2),
            ("stˈaːrɛ", 0),
            ("mˈɔrdɛrɛ", 0),
            ("dˈɔrsʊm", 3),
            ("wˈɛntʊs", 2),
            ("fˈuːmʊs", 2),
            ("kʷɪd", 0),
            ("fˈiːlʲiʊs", 2),
            ("ˈoːwʊm", 3),
            ("dˈarɛ", 0),
            ("nˈowʊs", 0),
            ("ˈuːrɛrɛ", 0),
            ("noːn", 0),
            ("dwˈɛnɔs", 0),
            ("skˈiːrɛ", 0),
            ("ɡˈɛnuː", 3),
            ("harˈeːna", 1),
            ("rˈiːdeːrɛ", 0),
            ("awdˈiːrɛ", 0),
            ("hˈʊmʊs", 1),
            ("fˈɔlʲiʊm", 3),
            ("rˈʊbeʊs", 0),
            ("jˈɛkʊr", 3),
            ("ɫˈateːrɛ", 0),
            ("kˈʊtɪs", 1),
            ("sˈuːɡɛrɛ", 0),
            ("fˈɛrrɛ", 0),
            ("fɔrmˈiːka", 1),
            ("ɡrˈawɪs", 0),
            ("kˈapɛrɛ", 0),
            ("wˈɛtʊs", 0),
            ("ˈɛdɛrɛ", 0),
            ("fˈɛmʊr", 3),
            ("krˈassʊs", 0),
            ("ɫˈɔŋɡʊs", 0),
            ("fɫˈaːrɛ", 0),
            ("lʲˈɪŋnʊm", 3),
            ("kˈʊrrɛrɛ", 0),
            ("kˈadɛrɛ", 0),
            ("ˈɔkʊɫ̪ʊs", 2),
            ("kˈɪnɪs", 1),
            ("kˈawda", 1),
            ("kˈanɪs", 2),
            ("fɫˈeːrɛ", 0),
            ("lˈɪɡaːrɛ", 0),
            ("wɪdˈeːrɛ", 0),
            ("dˈʊɫkɪs", 0),
            ("fˈuːnɪs", 2),
            ("ˈʊmbra", 1),
            ("ˈawɪs", 1),
            ("saːɫ", 2),
            ("pˈarwʊs", 0),
            ("ɫˈaːtʊs", 0),
            ("stˈeːllʲa", 1),
            ("ɪn", 0),
            ("dˈuːrʊs", 0),
            ("pˈiːnsɛrɛ", 0)
        ]),
        grammar: Grammar {
            predicate_word_order: "SOV".to_string(),
            adjective_before_noun: true,
            prepositions: true,
            pronominal_cases: CASE_NOMINATIVE | CASE_ACCUSATIVE | CASE_GENITIVE | CASE_ABLATIVE | CASE_DATIVE,
            nominal_cases: CASE_NOMINATIVE | CASE_GENITIVE | CASE_DATIVE | CASE_ACCUSATIVE | CASE_ABLATIVE | CASE_LOCATIVE | CASE_VOCATIVE,
            pronominal_numbers: NUM_SINGULAR | NUM_PLURAL,
            nominal_numbers: NUM_SINGULAR | NUM_PLURAL,
            numeric_base: 10,
            finitivity: 0,
            determiner_before_noun: true,
            noun_classes: vec![vec![NounClassCatergories::None], vec![NounClassCatergories::Feminine], vec![NounClassCatergories::Masculine], vec![NounClassCatergories::Neuter]],
            pronoun_classes: vec![vec![NounClassCatergories::Feminine], vec![NounClassCatergories::Masculine], vec![NounClassCatergories::Epicene]],
            has_inclusive_exclusive_distinction: false,
            declensions: true,
            personal_conjugation: true,
            third_person_personal_pronouns: true,
            formality: false,
            contraction: false,
            obligate_contraction: false,
            explicit_copula: true,
            ser_estar_distinction: false,
            part_of_speech_morphology: true,
            tenses: 3,
            aspect: ASPECT_PERFECTIVE | ASPECT_IMPERFECTIVE,
            mood: MOOD_INDICATIVE | MOOD_SUBJUNCTIVE | MOOD_IMPERATIVE,
            evidentials: 0,
            has_verbal_voice: true,
            double_negatives_are_positive: true,
            reduplication: false,
            has_augmentative: false,
            has_diminutive: true,
        },
        phonology: Phonology {
            tone_count: 0,
            vowel_length: true,
            nasal_vowels: false,
            rhotic_vowels: false,
            front_rounded_vowels: true,
            back_unrounded_vowels: false,
            schwa: false,
            reduction: true,
            accent_system: Accent::ContrastiveStress,
            mora_timed: true,
            plosive_series_count: 4,
            fricative_voicedness_distinction: false,
            nasal_voicedness_distinction: false,
            has_laterals: true,
            has_phonemic_diphthongs: true,
            has_phonetic_diphthongs: false,
            basic_vowel_count: 6,
            has_glottal: true,
            has_uvular: false,
            has_labiodental: true,
            has_linguodental: false,
            has_retroflex_or_postalveolar: false,
            has_palatal: false,
            has_labial: true,
            has_velar: true,
            has_pharyngeal_or_epiglottal: false,
            has_vibrants: true,
            gemination: true,
            palatalization: true,
            velarization: true,
            labialization: true,
            emphatics: false,
        },
    };

    let mut model: BinaryTree = BinaryTree {
        val: vec![get_node_from_languoid(Box::new(lang.clone())), get_node_from_languoid(Box::new(lang_2.clone()))],
    };

    model.iterate_minimum_distance_model();


    println!("{:?}", model);

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
    pub evidentials: i32,
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

fn has_bitflag(val: &usize, flag: &usize) -> bool {
    (val & flag) != 0
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
const CASE_VOCATIVE: u64 = 0x100000000000000;
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

