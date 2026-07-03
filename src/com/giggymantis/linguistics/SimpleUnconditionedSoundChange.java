package com.giggymantis.linguistics;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;

import com.giggymantis.probable_broccoli.UtilsConstants;

// Simple: a -> b
// Note that the array is a sequence (so {a, b} represents "ab" and not "a" or "b")
public class SimpleUnconditionedSoundChange implements SoundChange {

	public final Object[] sourceSequence, outcomeSequence;
	
	public SimpleUnconditionedSoundChange(String sourceSequence, String outcomeSequence) {
		this.sourceSequence = sourceSequence.chars().mapToObj(c -> (char) c).toArray(Character[]::new);
		this.outcomeSequence = outcomeSequence.chars().mapToObj(c -> (char) c).toArray(Character[]::new);
	}
	
	public SimpleUnconditionedSoundChange(Object[] sourceSequence, Object[] outcomeSequence) {
		this.sourceSequence = sourceSequence;
		this.outcomeSequence = outcomeSequence;
	}
	
	public SimpleUnconditionedSoundChange(Object[] sourceSequence, Object outcome) {
		this.sourceSequence = sourceSequence;
		this.outcomeSequence = new Object[] {outcome};
	}
	
	public SimpleUnconditionedSoundChange(Object source, Object[] outcomeSequence) {
		this.sourceSequence = new Object[] {source};
		this.outcomeSequence = outcomeSequence;
	}
	
	public SimpleUnconditionedSoundChange(Object source, Object outcome) {
		this.sourceSequence = new Object[] {source};
		this.outcomeSequence = new Object[] {outcome};
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
		if (units[index].equals(sourceSequence[0]) && index + sourceSequence.length <= units.length) {
			if (sourceSequence.length == 1) {
				return true;
			}
			boolean check = true;
			for (int j = 1; j < sourceSequence.length; j++) {
				if (units[index+j].equals(sourceSequence[j])) {
					check = false;
				}
			}
			if (check) {
				return true;
			}
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
		return "%s -> %s".formatted(UtilsConstants.trimEdges(Arrays.toString(sourceSequence)), UtilsConstants.trimEdges(Arrays.toString(outcomeSequence)));
	}

	@Override
	public HashSet<Word> applyReverse(Word word) {
		HashSet<Word> ret = new HashSet<Word>();
		ret.add(this.converse().apply(word));
		ret.add(word);
		return ret;
	}
	
	@Override
	public SimpleUnconditionedSoundChange converse() {
		return new SimpleUnconditionedSoundChange(outcomeSequence, sourceSequence);
	}

}
