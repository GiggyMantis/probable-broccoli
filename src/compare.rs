use strsim::{normalized_levenshtein,normalized_damerau_levenshtein};
use crate::*;

// This Module (compare) compares two languages and returns a "distance" score.
// The higher a distance score is, the more *distantly* two languages are related.

const LEXICON_MULTIPLIER : f64 = 1.0;
const GRAMMAR_MULTIPLIER : f64 = 1.0;
const GRAMMAR_MULTIPLIER_WORD_ORDER : f64 = 1.0;
const GRAMMAR_MULTIPLIER_CASES : f64 = 1.0;
const GRAMMAR_MULTIPLIER_NUMBER : f64 = 1.0;
const GRAMMAR_MULTIPLIER_DETERMINATION : f64 = 1.0;
const GRAMMAR_MULTIPLIER_GENDER : f64 = 1.0;
const GRAMMAR_MULTIPLIER_PRONOMINAL : f64 = 1.0;
const GRAMMAR_MULTIPLIER_CONJUGATION : f64 = 1.0;
const GRAMMAR_MULTIPLIER_MORPHOLOGY : f64 = 1.0;
const GRAMMAR_MULTIPLIER_COPULA : f64 = 1.0;
const PHONOLOGY_MULTIPLIER : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_TONE : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_VOWEL_MODIFIERS : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_VOWEL_QUALITIES : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_CONSONANTS : f64 = 1.0;
const PHONOLOGY_MULTIPLIER_ACCENT : f64 = 1.0;


fn compare_individual(lect_a: Box<Languoid>, lect_b: Box<Languoid>) -> u16 {
    if lect_a.as_ref() == lect_b.as_ref() {
        println!("Same langs");
        return 0;
    }

    let mut normalized_levenshtein_distance = 0.0;
    let mut grammar_distance = 0.0;
    let mut phonological_distance = 0.0;
    for n in 0..100 {
        normalized_levenshtein_distance += 1.0 - normalized_levenshtein(&*ipa_mapping::to_broccoli_sampa(&lect_a.leipzig_jakarta_list[n].0), &*ipa_mapping::to_broccoli_sampa(&lect_b.leipzig_jakarta_list[n].0))
    }
    let mut lexicon_distance = LEXICON_MULTIPLIER * (normalized_levenshtein_distance as f64);

    // Word Order
    grammar_distance += 1.0 - normalized_damerau_levenshtein(&lect_a.grammar.predicate_word_order, &lect_b.grammar.predicate_word_order);
    grammar_distance += if lect_a.grammar.adjective_before_noun != lect_b.grammar.adjective_before_noun {GRAMMAR_MULTIPLIER_WORD_ORDER} else {0.0};
    grammar_distance += if lect_a.grammar.prepositions != lect_b.grammar.prepositions {GRAMMAR_MULTIPLIER_WORD_ORDER} else {0.0};

    // Copula
    grammar_distance += if lect_a.grammar.explicit_copula != lect_b.grammar.explicit_copula {GRAMMAR_MULTIPLIER_COPULA} else {0.0};
    grammar_distance += if lect_a.grammar.ser_estar_distinction != lect_b.grammar.ser_estar_distinction {GRAMMAR_MULTIPLIER_COPULA} else {0.0};
    grammar_distance += if lect_a.grammar.contraction != lect_b.grammar.contraction {GRAMMAR_MULTIPLIER_MORPHOLOGY} else {if lect_a.grammar.obligate_contraction != lect_b.grammar.obligate_contraction {GRAMMAR_MULTIPLIER_MORPHOLOGY} else {0.0}};



    grammar_distance *= GRAMMAR_MULTIPLIER;
    phonological_distance *= PHONOLOGY_MULTIPLIER;
    return (lexicon_distance + grammar_distance + phonological_distance) as u16;
}

pub(crate) fn compare(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    if lect_a.val() != None && lect_b.val() != None {
        return compare_individual(lect_a.val().unwrap(), lect_b.val().unwrap());
    }
    if lect_a.val() != None {
        return compare_languoid_and_fam(lect_a, lect_b);
    }
    if lect_b.val() != None {
        return compare_languoid_and_fam(lect_b, lect_a);
    }

    return compare_fam_and_fam(lect_a, lect_b);
}

fn compare_fam_and_fam(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    let left_distance = compare(lect_a.left().unwrap(), lect_b.clone());
    let right_distance = compare(lect_a.right().unwrap(), lect_b.clone());
    
    (left_distance + right_distance) / 2
}

fn compare_languoid_and_fam(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    let left_distance = compare(lect_a.clone(), lect_b.left().unwrap());
    let right_distance = compare(lect_a.clone(), lect_b.right().unwrap());

    (left_distance + right_distance) / 2
}

