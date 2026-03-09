import type { City } from "../config.js";

export interface Restaurant {
  name: string;
  cuisine: string;
  priceRange: string;
  mustTry: string;
  neighborhood: string;
  rating: number;
}

export const RESTAURANTS: Record<City, Restaurant[]> = {
  rome: [
    { name: "Da Enzo al 29", cuisine: "Traditional Roman", priceRange: "$$", mustTry: "Cacio e Pepe", neighborhood: "Trastevere", rating: 4.7 },
    { name: "Roscioli", cuisine: "Italian Deli & Wine Bar", priceRange: "$$$", mustTry: "Carbonara", neighborhood: "Campo de' Fiori", rating: 4.8 },
    { name: "Pizzarium Bonci", cuisine: "Pizza al Taglio", priceRange: "$", mustTry: "Mortadella & Burrata Pizza", neighborhood: "Prati", rating: 4.6 },
  ],
  florence: [
    { name: "Trattoria Mario", cuisine: "Tuscan Home Cooking", priceRange: "$", mustTry: "Bistecca alla Fiorentina", neighborhood: "San Lorenzo", rating: 4.5 },
    { name: "All'Antico Vinaio", cuisine: "Panini & Street Food", priceRange: "$", mustTry: "La Favolosa Sandwich", neighborhood: "Historic Center", rating: 4.7 },
    { name: "Osteria dell'Enoteca", cuisine: "Fine Tuscan Dining", priceRange: "$$$$", mustTry: "Pappardelle al Cinghiale", neighborhood: "Santa Croce", rating: 4.9 },
  ],
  venice: [
    { name: "Osteria Alle Testiere", cuisine: "Venetian Seafood", priceRange: "$$$", mustTry: "Spaghetti alle Vongole", neighborhood: "Castello", rating: 4.8 },
    { name: "Cantina Do Spade", cuisine: "Cicchetti Bar", priceRange: "$$", mustTry: "Baccalà Mantecato", neighborhood: "San Polo", rating: 4.6 },
    { name: "Trattoria Da Romano", cuisine: "Traditional Venetian", priceRange: "$$", mustTry: "Risotto al Nero di Seppia", neighborhood: "Burano", rating: 4.5 },
  ],
  milan: [
    { name: "Trattoria Milanese", cuisine: "Milanese Classic", priceRange: "$$", mustTry: "Risotto alla Milanese", neighborhood: "Centro Storico", rating: 4.4 },
    { name: "Piz", cuisine: "Neapolitan Pizza", priceRange: "$", mustTry: "Margherita DOC", neighborhood: "Navigli", rating: 4.6 },
    { name: "Ratanà", cuisine: "Modern Lombardy", priceRange: "$$$", mustTry: "Ossobuco con Gremolata", neighborhood: "Porta Nuova", rating: 4.7 },
  ],
};
