import { useRef } from "react";
import { motion, useInView } from "framer-motion";
import "./ContactSection.css";

const INVESTOR_EMAIL = "investors@pronox.dev";

export default function ContactSection() {
  const ref = useRef<HTMLElement>(null);
  const inView = useInView(ref, { once: true, margin: "-80px" });

  return (
    <section className="contact-section section" ref={ref}>
      <motion.h2
        className="section-heading"
        initial={{ opacity: 0, y: 16 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4 }}
      >
        For investors
      </motion.h2>
      <motion.p
        className="section-sub"
        initial={{ opacity: 0, y: 12 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.05 }}
      >
        We're building the governance layer for the agent economy. Get in touch.
      </motion.p>
      <motion.div
        className="contact-actions"
        initial={{ opacity: 0, y: 12 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.1 }}
      >
        <a href={`mailto:${INVESTOR_EMAIL}`} className="btn-primary">
          Reach us
        </a>
        <span className="contact-email">{INVESTOR_EMAIL}</span>
      </motion.div>
    </section>
  );
}
