package com.giggymantis.phonetics;

public enum Phonation {
	VOICELESS,
	BREATHY,
	SLACK,
	MODAL,
	STIFF,
	CREAKY;
	
	public double distance(Phonation q) { return Math.abs(this.ordinal() - q.ordinal()); }
}
