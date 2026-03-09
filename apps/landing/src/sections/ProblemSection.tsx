import { useRef } from "react";
import { motion, useInView } from "framer-motion";
import "./ProblemSection.css";

const cards = [
  {
    id: "blindness",
    title: "Blindness",
    subtitle: "The observability gap",
    copy: "When a chain of agents produces a bad outcome, there's no unified way to trace what happened across independent services. Existing tools trace inside one agent, not between agents. Pronox gives you full visibility into every agent-to-agent call.",
  },
  {
    id: "lawlessness",
    title: "Lawlessness",
    subtitle: "The identity gap",
    copy: "Agents call other agents with no standardized identity, authorization, or boundary enforcement. Any agent can impersonate any other. No guardrails. We're building identity and attestation so every A2A call is verified.",
  },
  {
    id: "liability",
    title: "Liability",
    subtitle: "The compliance gap",
    copy: "The EU AI Act (August 2026) mandates record-keeping, transparency, and human oversight. Non-compliance: up to €35M or 7% of global turnover. No product today unifies this for multi-agent systems. Pronox is compliance-native for the agent economy.",
  },
];

const container = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: { staggerChildren: 0.12, delayChildren: 0.1 },
  },
};

const item = {
  hidden: { opacity: 0, y: 20 },
  visible: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.4, ease: "easeOut" as const },
  },
};

export default function ProblemSection() {
  const ref = useRef<HTMLElement>(null);
  const inView = useInView(ref, { once: true, margin: "-80px" });

  return (
    <section className="problem-section section" ref={ref} id="problem">
      <motion.h2
        className="section-heading"
        initial={{ opacity: 0, y: 16 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4 }}
      >
        The problem
      </motion.h2>
      <motion.p
        className="section-sub"
        initial={{ opacity: 0, y: 12 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.05 }}
      >
        When AI agents connect to other agents, three gaps block enterprise adoption. We're building the infrastructure to fix them.
      </motion.p>
      <motion.div
        className="problem-cards"
        variants={container}
        initial="hidden"
        animate={inView ? "visible" : "hidden"}
      >
        {cards.map((c) => (
          <motion.article
            key={c.id}
            className="problem-card"
            variants={item}
          >
            <h3 className="problem-card-title">{c.title}</h3>
            <p className="problem-card-sub">{c.subtitle}</p>
            <p className="problem-card-copy">{c.copy}</p>
          </motion.article>
        ))}
      </motion.div>
    </section>
  );
}
