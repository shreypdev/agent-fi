import { Link } from "react-router-dom";
import { motion } from "framer-motion";
import { CONNECT_PATH } from "../config";
import Hero from "../sections/Hero";
import BuildingSection from "../sections/BuildingSection";
import ProblemSection from "../sections/ProblemSection";
import SolutionSection from "../sections/SolutionSection";
import WhyUs from "../sections/WhyUs";
import RoadmapSection from "../sections/RoadmapSection";
import ContactSection from "../sections/ContactSection";
import "../App.css";

export default function HomePage() {
  return (
    <motion.main
      className="page"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.4, ease: [0.16, 1, 0.3, 1] }}
    >
      <Hero />
      <BuildingSection />
      <section aria-label="The problem">
        <ProblemSection />
      </section>
      <SolutionSection />
      <WhyUs />
      <section id="roadmap" aria-label="Roadmap">
        <RoadmapSection />
      </section>
      <section id="contact" aria-label="Contact">
        <ContactSection />
      </section>
      <section className="section cta-section">
        <Link to={CONNECT_PATH} className="btn-primary">
          Connect now
        </Link>
      </section>
    </motion.main>
  );
}
