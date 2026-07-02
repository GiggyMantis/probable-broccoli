package com.giggymantis.linguistics;

// Interfaced to allow for multiple modes of comparison.
//TODO: Create phonetic mode.
public interface Word {
	public Object[] getUnits();
	public void setUnits(Object[] units);
	
	public Object getUnit(int index);
	public void setUnit(int index, Object object);
	
	public double distance(Word q);
	
	public String getGloss();
	public void setGloss(String newGloss);
}
