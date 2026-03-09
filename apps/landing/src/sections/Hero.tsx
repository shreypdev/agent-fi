import { Link } from "react-router-dom";
import { motion } from "framer-motion";
import { CONNECT_PATH } from "../config";
import "./Hero.css";

const oneLiner =
  "Unified observability and identity for AI agents. Transport-agnostic, compliance-native.";
const supporting =
  "The governance layer for the agent economy.";

export default function Hero() {
  return (
    <section className="hero" aria-label="Hero">
      <div className="hero-bg" aria-hidden />
      <div className="hero-inner section">
        <motion.h1
          className="hero-title"
          initial={{ opacity: 0, y: 32, scale: 0.98 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          transition={{ duration: 0.8, ease: [0.16, 1, 0.3, 1], delay: 0.15 }}
        >
          <span className="hero-title-gradient">{oneLiner}</span>
        </motion.h1>
        <motion.p
          className="hero-sub"
          initial={{ opacity: 0, y: 24 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.45, ease: [0.16, 1, 0.3, 1] }}
        >
          {supporting}
        </motion.p>
        <motion.div
          className="hero-cta"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, delay: 0.7, ease: [0.16, 1, 0.3, 1] }}
        >
          <Link to={CONNECT_PATH} className="btn-primary hero-cta-btn">
            Try the agent
          </Link>
        </motion.div>
      </div>
    </section>
  );
}
