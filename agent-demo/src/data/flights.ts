import type { City } from "../config.js";

export interface Flight {
  airline: string;
  from: string;
  to: string;
  departure: string;
  arrival: string;
  duration: string;
  price: number;
  stops: number;
}

export const FLIGHTS: Record<City, Flight[]> = {
  rome: [
    { airline: "Alitalia", from: "New York (JFK)", to: "Rome (FCO)", departure: "10:30 PM", arrival: "12:45 PM+1", duration: "8h 15m", price: 685, stops: 0 },
    { airline: "Delta", from: "New York (JFK)", to: "Rome (FCO)", departure: "6:00 PM", arrival: "8:30 AM+1", duration: "8h 30m", price: 742, stops: 0 },
    { airline: "United", from: "New York (EWR)", to: "Rome (FCO)", departure: "9:15 PM", arrival: "12:00 PM+1", duration: "8h 45m", price: 598, stops: 0 },
  ],
  florence: [
    { airline: "Alitalia", from: "New York (JFK)", to: "Florence (FLR)", departure: "8:00 PM", arrival: "1:30 PM+1", duration: "11h 30m", price: 812, stops: 1 },
    { airline: "Lufthansa", from: "New York (JFK)", to: "Florence (FLR)", departure: "5:45 PM", arrival: "12:15 PM+1", duration: "12h 30m", price: 756, stops: 1 },
  ],
  venice: [
    { airline: "Emirates", from: "New York (JFK)", to: "Venice (VCE)", departure: "11:00 PM", arrival: "3:45 PM+1", duration: "10h 45m", price: 895, stops: 1 },
    { airline: "KLM", from: "New York (JFK)", to: "Venice (VCE)", departure: "4:30 PM", arrival: "11:00 AM+1", duration: "12h 30m", price: 634, stops: 1 },
  ],
  milan: [
    { airline: "Alitalia", from: "New York (JFK)", to: "Milan (MXP)", departure: "9:00 PM", arrival: "10:45 AM+1", duration: "7h 45m", price: 620, stops: 0 },
    { airline: "Emirates", from: "New York (JFK)", to: "Milan (MXP)", departure: "11:30 PM", arrival: "4:00 PM+1", duration: "10h 30m", price: 710, stops: 1 },
  ],
};
