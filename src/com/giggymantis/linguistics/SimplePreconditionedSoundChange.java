package com.giggymantis.linguistics;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;

import com.giggymantis.probable_broccoli.UtilsConstants;

// Preconditioned: a -> b / c_
// Note that the array is a sequence (so {a, b} represents "ab" and not "a" or "b")
public class SimplePreconditionedSoundChange implements SoundChange {

	public final Object[] sourceSequence, outcomeSequence, environmentSequence;
	
	public SimplePreconditionedSoundChange(String sourceSequence, String outcomeSequence, String environmentSequence) {
		this.sourceSequence = sourceSequence.split("");
		this.outcomeSequence = outcomeSequence.split("");
		this.environmentSequence = environmentSequence.split("");
	}
	
	public SimplePreconditionedSoundChange(Object[] sourceSequence, Object[] outcomeSequence, Object[] environmentSequence) {
		this.sourceSequence = sourceSequence;
		this.outcomeSequence = outcomeSequence;
		this.environmentSequence = environmentSequence;
	}
	
	public SimplePreconditionedSoundChange(Object source, Object outcome, Object environment) {
		this.sourceSequence = new Object[] {source};
		this.outcomeSequence = new Object[] {outcome};
		this.environmentSequence = new Object[] {environment};
	}
	
	@Override
	public boolean doesApply(Word word) {
		Object[] units = word.getUnits();
		for (int i = 0; i < units.length; i++) {
			if (appliesAt(units, i)) {
				return true;
			}
		}
		return false;
	}

	public boolean appliesAt(Word word, int index) {
		return appliesAt(word.getUnits(), index);
	}
	
	public boolean appliesAt(Object[] units, int index) {
		if (units[index].equals(sourceSequence[0]) && index + sourceSequence.length <= units.length && index >= environmentSequence.length) {
			boolean simple = false;
			if (sourceSequence.length == 1) {
				simple = true;
			}
			boolean check = true;
			for (int i = 1; i < sourceSequence.length; i++) {
				if (units[index+i].equals(sourceSequence[i])) {
					check = false;
				}
			}
			if (check) {
				simple = true;
			}
			
			if (!simple) {
				return false;
			}
			
			check = true;
			for (int i = 0; i < environmentSequence.length; i++) {
				if (!units[index + i - environmentSequence.length].equals(environmentSequence[i])) {
					check = false;
				}
			}
			return check;
		}
		return false;
	}

	@Override
	public Word apply(Word word) {
		Object[] units = word.getUnits();
		List<Object> returnList = new ArrayList<Object>();
		
		for (int i = 0; i < units.length; i++) {
			if (appliesAt(units, i)) {
				returnList.addAll(Arrays.asList(outcomeSequence));
				i += sourceSequence.length - 1;
			} else {
				returnList.add(units[i]);
			}
		}
		return word.withNewUnits(returnList.toArray());
	}
	
	@Override
	public String toString() {
		return "%s -> %s / %s_".formatted(UtilsConstants.trimEdges(Arrays.toString(sourceSequence)), UtilsConstants.trimEdges(Arrays.toString(outcomeSequence)), UtilsConstants.trimEdges(Arrays.toString(environmentSequence)));
	}

	@Override
	public HashSet<Word> applyReverse(Word word) {
		HashSet<Word> ret = new HashSet<Word>();
		ret.add(this.converse().apply(word));
		ret.add(word);
		return ret;
	}
	
	@Override
	public SimplePreconditionedSoundChange converse() {
		return new SimplePreconditionedSoundChange(outcomeSequence, sourceSequence, environmentSequence);
	}

}
