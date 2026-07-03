package com.giggymantis.linguistics;

public class SimpleSoundChange implements SoundChange {

	public final Object source, outcome;
	
	public SimpleSoundChange(Object source, Object outcome) {
		this.source = source;
		this.outcome = outcome;
	}
	
	@Override
	public boolean doesApply(Word word) {
		Object[] units = word.getUnits();
		for (int i = 0; i < units.length; i++) {
			if (units[i].equals(source)) {
				return true;
			}
		}
		return false;
	}

	@Override
	public Word apply(Word word) {
		Object[] units = word.getUnits();
		for (int i = 0; i < units.length; i++) {
			if (units[i].equals(source)) {
				units[i] = outcome;
			}
		}
		
		return word.withNewUnits(units);
	}
	
	@Override
	public String toString() {
		return "%s → %s".formatted(source, outcome);
	}

}
