package com.giggymantis.linguistics;

import java.util.HashSet;

public interface SoundChange {
	public boolean doesApply(Word word);
	public Word apply(Word word);
	public HashSet<Word> applyReverse(Word word);
	public SoundChange converse();
}
