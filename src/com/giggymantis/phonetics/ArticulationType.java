package com.giggymantis.phonetics;

public enum ArticulationType {
	FALSE,
	SECONDARY,
	PRIMARY;
	
	public boolean bool() { return this.bool(false); }
	public boolean bool(boolean countSecondary) {
		switch (this) {
			case PRIMARY:
				return true;
			case SECONDARY:
				return countSecondary;
			default:
				return false;
		}
	}
	
	public boolean isSecondary() { return (this == SECONDARY); }
	
	public double distance(ArticulationType q) { return Math.abs(this.ordinal() - q.ordinal()); }
}
