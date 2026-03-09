import { useRef } from "react";
import { motion, useInView } from "framer-motion";
import "./SolutionSection.css";

const bullets = [
  "Unified observability: trace every agent-to-agent call in one dashboard, across any transport (A2A, MCP, REST).",
  "Identity and authorization: who is calling, who delegated, what they can do. Verified on every A2A handshake.",
  "Compliance-native: record-keeping, transparency, and human oversight for the EU AI Act, built for multi-agent systems.",
];

export default function SolutionSection() {
  const ref = useRef<HTMLElement>(null);
  const inView = useInView(ref, { once: true, margin: "-80px" });

  return (
    <section className="solution-section section" ref={ref}>
      <motion.h2
        className="section-heading"
        initial={{ opacity: 0, y: 16 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4 }}
      >
        One platform for agent-to-agent
      </motion.h2>
      <motion.p
        className="section-sub"
        initial={{ opacity: 0, y: 12 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.05 }}
      >
        Observability, identity, and compliance for A2A, designed together from day one, not retrofitted.
      </motion.p>
      <motion.ul
        className="solution-list"
        initial={{ opacity: 0, y: 16 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.1 }}
      >
        {bullets.map((b, i) => (
          <li key={i}>{b}</li>
        ))}
      </motion.ul>
    </section>
  );
}
