use strsim::{normalized_levenshtein, normalized_damerau_levenshtein};
use crate::*;

/// This Module (compare) compares two languages and returns a "distance" score.
/// The higher a distance score is, the more *distantly* two languages are related.

const LEXICON_MULTIPLIER : f64 = 1.0;
const LEXICON_MULTIPLIER_GENDER_ONLY : f64 = 1.0; 
const GRAMMAR_MULTIPLIER : f64 = 1.0;
const GRAMMAR_MULTIPLIER_WORD_ORDER : f64 = 1.0;
const GRAMMAR_MULTIPLIER_COPULA : f64 = 1.0;
const GRAMMAR_MULTIPLIER_CASES : f64 = 1.0;
const GRAMMAR_MULTIPLIER_NUMBER : f64 = 1.0;
const GRAMMAR_MULTIPLIER_DETERMINATION : f64 = 1.0;
const GRAMMAR_MULTIPLIER_GENDER : f64 = 1.0;
const GRAMMAR_MULTIPLIER_CONJUGATION : f64 = 1.0;
const GRAMMAR_MULTIPLIER_MORPHOLOGY : f64 = 1.0;
const PHONOLOGY_MULTIPLIER : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_TONE : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_VOWEL_MODIFIERS : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_CONSONANTS : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_CONSONANT_MODIFIERS : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_ACCENT : f64 = 1.0;

/// Compares two individual languoids.
fn compare_individual(lect_a: Box<Languoid>, lect_b: Box<Languoid>) -> u16 {
    if lect_a.as_ref() == lect_b.as_ref() {
        return 0;
    }

    let mut normalized_levenshtein_distance = 0.0;
    let mut grammar_distance = 0.0;
    let mut phonological_distance = 0.0;

    // Loops over every word in the Leipzig-Jakarta List
    for n in 0..100 {
        normalized_levenshtein_distance += 1.0 - normalized_levenshtein(&*ipa_mapping::to_broccoli_sampa(&lect_a.leipzig_jakarta_list[n].0), &*ipa_mapping::to_broccoli_sampa(&lect_b.leipzig_jakarta_list[n].0))
        lexicon_distance += LEXICON_MULTIPLIER_GENDER_ONLY * gender_distance(lect_a.leipzig_jakarta_list[n].1, lect_b.leipzig_jakarta_list[n].1);
        // TODO: Add in distance for differing noun classes
    }
    let lexicon_distance += LEXICON_MULTIPLIER * (normalized_levenshtein_distance as f64);

    // Word Order
    grammar_distance += 1.0 - normalized_damerau_levenshtein(&lect_a.grammar.predicate_word_order, &lect_b.grammar.predicate_word_order);
    grammar_distance += if lect_a.grammar.adjective_before_noun != lect_b.grammar.adjective_before_noun {GRAMMAR_MULTIPLIER_WORD_ORDER} else {0.0};
    grammar_distance += if lect_a.grammar.prepositions != lect_b.grammar.prepositions {GRAMMAR_MULTIPLIER_WORD_ORDER} else {0.0};

    // Copula
    grammar_distance += if lect_a.grammar.explicit_copula != lect_b.grammar.explicit_copula {GRAMMAR_MULTIPLIER_COPULA} else {0.0};
    grammar_distance += if lect_a.grammar.ser_estar_distinction != lect_b.grammar.ser_estar_distinction {GRAMMAR_MULTIPLIER_COPULA} else {0.0};
    grammar_distance += if lect_a.grammar.contraction != lect_b.grammar.contraction {GRAMMAR_MULTIPLIER_MORPHOLOGY} else {if lect_a.grammar.obligate_contraction != lect_b.grammar.obligate_contraction {GRAMMAR_MULTIPLIER_MORPHOLOGY} else {0.0}};

    // Cases
    grammar_distance += GRAMMAR_MULTIPLIER_CASES * (lect_a.grammar.pronominal_cases ^ lect_b.grammar.pronominal_cases).count_ones() as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_CASES * (lect_a.grammar.nominal_cases ^ lect_b.grammar.nominal_cases).count_ones() as f64;

    // Number
    grammar_distance += GRAMMAR_MULTIPLIER_NUMBER * (lect_a.grammar.pronominal_numbers ^ lect_b.grammar.pronominal_numbers).count_ones() as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_NUMBER * (lect_a.grammar.nominal_numbers ^ lect_b.grammar.pronominal_numbers).count_ones() as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_NUMBER * i16::abs(lect_a.grammar.numeric_base as i16 - lect_b.grammar.numeric_base as i16) as f64;

    // Determination
    grammar_distance += GRAMMAR_MULTIPLIER_DETERMINATION * (lect_a.grammar.finitivity ^ lect_b.grammar.finitivity).count_ones() as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_DETERMINATION * (lect_a.grammar.determiner_before_noun ^ lect_b.grammar.determiner_before_noun) as u8 as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_DETERMINATION * (lect_a.grammar.third_person_personal_pronouns ^ lect_b.grammar.third_person_personal_pronouns) as u8 as f64;

    // Gender
    // TODO: Gender

    // Conjugation
    grammar_distance += GRAMMAR_MULTIPLIER_CONJUGATION * (lect_a.grammar.personal_conjugation ^ lect_b.grammar.personal_conjugation) as u8 as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_CONJUGATION * (lect_a.grammar.aspect ^ lect_b.grammar.aspect).count_ones() as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_CONJUGATION * (lect_a.grammar.mood ^ lect_b.grammar.mood).count_ones() as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_CONJUGATION * i16::abs(lect_a.grammar.tenses as i16 - lect_b.grammar.tenses as i16) as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_CONJUGATION * i16::abs(lect_a.grammar.evidentials as i16 - lect_b.grammar.evidentials as i16) as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_CONJUGATION * (lect_a.grammar.has_verbal_voice ^ lect_a.grammar.has_verbal_voice) as u8 as f64;

    // Morphology
    grammar_distance += GRAMMAR_MULTIPLIER_MORPHOLOGY * (lect_a.grammar.contraction ^ lect_b.grammar.contraction) as u8 as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_MORPHOLOGY * (lect_a.grammar.obligate_contraction ^ lect_b.grammar.obligate_contraction) as u8 as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_MORPHOLOGY * (lect_a.grammar.part_of_speech_morphology ^ lect_b.grammar.part_of_speech_morphology) as u8 as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_MORPHOLOGY * (lect_a.grammar.double_negatives_are_positive ^ lect_b.grammar.double_negatives_are_positive) as u8 as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_MORPHOLOGY * (lect_a.grammar.reduplication ^ lect_b.grammar.reduplication) as u8 as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_MORPHOLOGY * (lect_a.grammar.has_augmentative ^ lect_b.grammar.has_augmentative) as u8 as f64;
    grammar_distance += GRAMMAR_MULTIPLIER_MORPHOLOGY * (lect_a.grammar.has_diminutive ^ lect_b.grammar.has_diminutive) as u8 as f64;

    // Vowels
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_MODIFIERS * (lect_a.phonology.vowel_length ^ lect_b.phonology.vowel_length) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_MODIFIERS * (lect_a.phonology.nasal_vowels ^ lect_b.phonology.nasal_vowels) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_MODIFIERS * (lect_a.phonology.rhotic_vowels ^ lect_b.phonology.rhotic_vowels) as u8 as f64;
  
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES * (lect_a.phonology.front_rounded_vowels ^ lect_b.phonology.front_rounded_vowels) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES * (lect_a.phonology.back_unrounded_vowels ^ lect_b.phonology.back_unrounded_vowels) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES * (lect_a.phonology.schwa ^ lect_b.phonology.schwa) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES * (lect_a.phonology.reduction ^ lect_b.phonology.reduction) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES * (i16::abs(lect_a.phonology.basic_vowel_count as i16 - lect_b.phonology.basic_vowel_count as i16) as f64);

    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES * (lect_a.phonology.has_phonemic_diphthongs ^ lect_b.phonology.has_phonemic_diphthongs) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES * (lect_a.phonology.has_phonetic_diphthongs ^ lect_b.phonology.has_phonetic_diphthongs) as u8 as f64; 

    phonological_distance += PHONOLOGY_MULTIPLIER_TONE * (i16::abs(lect_a.phonology.tone_count as i16 - lect_b.phonology.tone_count as i16) as f64);

    phonological_distance += PHONOLOGY_MULTIPLIER_ACCENT * Accent::distance(lect_a.phonology.accent_system, lect_b.phonology.accent_system);
    phonological_distance += PHONOLOGY_MULTIPLIER_ACCENT * (lect_a.phonology.mora_timed ^ lect_b.phonology.mora_timed) as u8 as f64;

    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (i16::abs(lect_a.phonology.plosive_series_count as i16 - lect_b.phonology.plosive_series_count as i16) as f64);
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.fricative_voicedness_distinction ^ lect_a.phonology.fricative_voicedness_distinction) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.nasal_voicedness_distinction ^ lect_b.phonology.nasal_voicedness_distinction) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_laterals ^ lect_b.phonology.has_laterals) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_glottal ^ lect_b.phonology.has_glottal) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_uvular ^ lect_b.phonology.has_uvular) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_labiodental ^ lect_b.phonology.has_labiodental) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_linguodental ^ lect_b.phonology.has_linguodental) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_retroflex_or_postalveolar ^ lect_b.phonology.has_retroflex_or_postalveolar) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_palatal ^ lect_b.phonology.has_palatal) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_labial ^ lect_b.phonology.has_labial) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_velar ^ lect_b.phonology.has_velar) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_pharyngeal_or_epiglottal ^ lect_b.phonology.has_pharyngeal_or_epiglottal) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANTS * (lect_a.phonology.has_vibrants ^ lect_b.phonology.has_vibrants) as u8 as f64;

    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANT_MODIFIERS * (lect_a.phonology.gemination ^ lect_b.phonology.gemination) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANT_MODIFIERS * (lect_a.phonology.palatalization ^ lect_b.phonology.palatalization) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANT_MODIFIERS * (lect_a.phonology.velarization ^ lect_b.phonology.velarization) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANT_MODIFIERS * (lect_a.phonology.labialization ^ lect_b.phonology.labialization) as u8 as f64;
    phonological_distance += PHONOLOGY_MULTIPLIER_CONSONANT_MODIFIERS * (lect_a.phonology.emphatics ^ lect_b.phonology.emphatics) as u8 as f64;

    // TODO: More Comparison

    grammar_distance *= GRAMMAR_MULTIPLIER;
    phonological_distance *= PHONOLOGY_MULTIPLIER;
    return (lexicon_distance + grammar_distance + phonological_distance) as u16;
}

/// Compares two TreeNodeRefs.
/// Returns the distance away from each other they are, __not__ a similarity.
pub fn compare(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    if lect_a.val().is_some() && lect_b.val().is_some() {
        return compare_individual(lect_a.val().unwrap(), lect_b.val().unwrap());
    }
    if lect_a.val().is_some() {
        return compare_languoid_and_fam(lect_a, lect_b);
    }
    if lect_b.val().is_some() {
        return compare_languoid_and_fam(lect_b, lect_a);
    }

    return compare_fam_and_fam(lect_a, lect_b);
}

/// Compares two language families.
fn compare_fam_and_fam(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    let left_distance = compare(lect_a.left().unwrap(), lect_b.clone());
    let right_distance = compare(lect_a.right().unwrap(), lect_b.clone());
    
    (left_distance + right_distance) / 2
}

/// Compares a language family and a single language.
fn compare_languoid_and_fam(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    let mut fam: &TreeNodeRef;
    let mut lang: &TreeNodeRef; 
    if lect_a.is_family() {
        fam = &lect_a;
        lang = &lect_b;
    } else {
        fam = &lect_b;
        lang = &lect_a;
    }

      ( compare(lang.clone(), fam.left().unwrap()) + 
        compare(lang.clone(), fam.right().unwrap()) )
        / 2u16
}

/// Returns the difference of grammatical gender between two noun classes
fn gender_distance(a: NounClass, b: NounClass) -> f64 {
    if a.contains(NounClassCatergories::None) | b.contains(NounClassCatergories::None) {
        return 0.0;
    }

    let mut diff = 0.0;

    for cat in a {
        if !b.contains(cat) {
            diff -= 1.0;
        }
    }

    return diff;
}