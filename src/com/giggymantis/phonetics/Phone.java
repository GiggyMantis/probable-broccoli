package com.giggymantis.phonetics;

import com.giggymantis.probable_broccoli.Constants;

public class Phone {

	public MethodOfArticulation methodOfArticulation;
	// These can probably be vectorized or sumn.
	public ArticulationType labial, dental, lingual, alveolar, postalveolar, dorsopalatal, velar, uvular, pharyngeal, glottal;
	public boolean lateral;
	public Phonation phonation;
	
	public Phone(MethodOfArticulation methodOfArticulation, ArticulationType labial, ArticulationType dental, ArticulationType lingual, ArticulationType alveolar,
			ArticulationType postalveolar, ArticulationType dorsopalatal, ArticulationType velar, ArticulationType uvular, ArticulationType pharyngeal,
			ArticulationType glottal, boolean lateral, Phonation phonation) {
		super();
		this.methodOfArticulation = methodOfArticulation;
		this.labial = labial;
		this.dental = dental;
		this.lingual = lingual;
		this.alveolar = alveolar;
		this.postalveolar = postalveolar;
		this.dorsopalatal = dorsopalatal;
		this.velar = velar;
		this.uvular = uvular;
		this.pharyngeal = pharyngeal;
		this.glottal = glottal;
		this.lateral = lateral;
		this.phonation = phonation;
	}
	
	
	// # Get features: overarching place(s) of articulation
	public boolean isLabial()		{ return labial.bool(); }
	public boolean isCoronal() 	{ return lingual.bool() && (dental.bool() || alveolar.bool() || postalveolar.bool()); }
	public boolean isPalatal() 	{ return alveolar.bool() || postalveolar.bool() || dorsopalatal.bool(); } // That is, *any* palatal consonant, including alveolars, retroflexes, etc.
	public boolean isDorsal()		{ return dorsopalatal.bool() || velar.bool() || uvular.bool(); }
	public boolean isGuttural()	{ return velar.bool() || uvular.bool() || pharyngeal.bool() || glottal.bool(); }
	public boolean isLaryngeal()	{ return pharyngeal.bool() || glottal.bool(); }
	
	public double distance(Phone q) {
		double dist = 0.0;
		//TODO: place of articulation distance
		dist += ((lateral ^ q.lateral) ? Constants.LATERAL_MEDIAL_WEIGHT : 0);
		dist += phonation.distance(q.phonation) * Constants.PHONATION_WEIGHT;
		dist += methodOfArticulation.distance(q.methodOfArticulation) * Constants.METHOD_OF_ARTICULATION_WEIGHT;
		
		
		return dist;
	}
	
	
}
