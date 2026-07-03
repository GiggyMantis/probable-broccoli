package com.giggymantis.linguistics;

public interface SoundChange {
	public boolean doesApply(Word word);
	public Word apply(Word word);
}
