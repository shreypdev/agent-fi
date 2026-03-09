import { useRef } from "react";
import { motion, useInView } from "framer-motion";
import "./WhyUs.css";

const items = [
  "EU AI Act deadline August 2026: timing is right for compliance. We're building for it now.",
  "Unified data model: observability and identity for A2A designed together, so every span carries identity context.",
  "Open-source wedge: MIT npm packages for the stack, commercial dashboard and compliance for scale.",
  "One-click connect: try our agent from ChatGPT, Claude, or Cursor in under 60 seconds. See A2A in action.",
];

export default function WhyUs() {
  const ref = useRef<HTMLElement>(null);
  const inView = useInView(ref, { once: true, margin: "-80px" });

  return (
    <section className="whyus-section section" ref={ref}>
      <motion.h2
        className="section-heading"
        initial={{ opacity: 0, y: 16 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4 }}
      >
        Why Pronox
      </motion.h2>
      <motion.p
        className="section-sub"
        initial={{ opacity: 0, y: 12 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.05 }}
      >
        We're building A2A infrastructure so the agent economy can scale safely. Built for agent-to-agent from day one.
      </motion.p>
      <motion.ul
        className="whyus-list"
        initial={{ opacity: 0 }}
        animate={inView ? { opacity: 1 } : {}}
        transition={{ duration: 0.4, delay: 0.1 }}
      >
        {items.map((text, i) => (
          <motion.li
            key={i}
            initial={{ opacity: 0, x: -12 }}
            animate={inView ? { opacity: 1, x: 0 } : {}}
            transition={{ duration: 0.35, delay: 0.12 + i * 0.06 }}
          >
            {text}
          </motion.li>
        ))}
      </motion.ul>
    </section>
  );
}
