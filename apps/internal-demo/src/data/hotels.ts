import type { City } from "../config.js";

export interface Hotel {
  name: string;
  stars: number;
  neighborhood: string;
  pricePerNight: number;
  highlights: string[];
}

export const HOTELS: Record<City, Hotel[]> = {
  rome: [
    { name: "Hotel Colosseum View", stars: 4, neighborhood: "Monti", pricePerNight: 189, highlights: ["Rooftop terrace", "Colosseum views", "Free breakfast"] },
    { name: "Roma Luxury Suites", stars: 5, neighborhood: "Via Veneto", pricePerNight: 320, highlights: ["Spa", "Michelin restaurant", "Butler service"] },
    { name: "Trastevere Inn", stars: 3, neighborhood: "Trastevere", pricePerNight: 110, highlights: ["Charming neighborhood", "Local feel", "Great value"] },
  ],
  florence: [
    { name: "Palazzo Vecchio Hotel", stars: 4, neighborhood: "Historic Center", pricePerNight: 215, highlights: ["River views", "Rooftop pool", "Art gallery"] },
    { name: "Firenze Boutique", stars: 5, neighborhood: "Santa Croce", pricePerNight: 380, highlights: ["Renaissance decor", "Private garden", "Wine cellar"] },
    { name: "Arno Riverside Lodge", stars: 3, neighborhood: "Oltrarno", pricePerNight: 95, highlights: ["Artisan workshops nearby", "Quiet area", "Free bikes"] },
  ],
  venice: [
    { name: "Canal Grande Palace", stars: 5, neighborhood: "San Marco", pricePerNight: 450, highlights: ["Grand Canal views", "Private dock", "Murano chandeliers"] },
    { name: "Rialto Bridge Hotel", stars: 4, neighborhood: "San Polo", pricePerNight: 230, highlights: ["Near Rialto Market", "Water taxi service", "Rooftop bar"] },
    { name: "Venetian Dreams B&B", stars: 3, neighborhood: "Cannaregio", pricePerNight: 125, highlights: ["Authentic neighborhood", "Canal views", "Homemade breakfast"] },
  ],
  milan: [
    { name: "Duomo Grand Hotel", stars: 5, neighborhood: "Centro Storico", pricePerNight: 350, highlights: ["Duomo views", "Design district", "Concierge service"] },
    { name: "Navigli Urban Hotel", stars: 4, neighborhood: "Navigli", pricePerNight: 175, highlights: ["Nightlife district", "Modern design", "Aperitivo bar"] },
    { name: "Brera Art Hotel", stars: 3, neighborhood: "Brera", pricePerNight: 130, highlights: ["Art galleries nearby", "Boutique shops", "Courtyard garden"] },
  ],
};
