import { useRef } from "react";
import { motion, useInView } from "framer-motion";
import "./BuildingSection.css";

const pillars: { name: string; desc: string }[] = [
  { name: "Observability", desc: "Trace every agent-to-agent call in one place." },
  { name: "Identity", desc: "Who is calling, who delegated, verified at the edge." },
  { name: "Trust", desc: "Delegation chains and attestations agents can rely on." },
  { name: "Guardrails", desc: "Rate limits, safety rules, and capability boundaries." },
  { name: "Wallet", desc: "Spend and budget per agent, per delegation." },
  { name: "Tx rails", desc: "Settlements and payments between agents." },
];

export default function BuildingSection() {
  const ref = useRef<HTMLElement>(null);
  const inView = useInView(ref, { once: true, margin: "-60px" });

  return (
    <section className="building-section section" ref={ref} id="what-we-build">
      <motion.h2
        className="section-heading building-heading"
        initial={{ opacity: 0, y: 24 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
      >
        A2A infrastructure
      </motion.h2>
      <motion.p
        className="section-sub building-sub"
        initial={{ opacity: 0, y: 12 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.05 }}
      >
        Infrastructure for agent-to-agent connect. We're building the full stack so AI agents can discover, call, and trust each other at scale.
      </motion.p>
      <motion.div
        className="building-pillars"
        initial={{ opacity: 0 }}
        animate={inView ? { opacity: 1 } : {}}
        transition={{ duration: 0.4, delay: 0.1 }}
      >
        {pillars.map((p, i) => (
          <motion.div
            key={p.name}
            className="building-pill-wrap"
            initial={{ opacity: 0, scale: 0.92, y: 8 }}
            animate={inView ? { opacity: 1, scale: 1, y: 0 } : {}}
            transition={{ duration: 0.4, delay: 0.1 + i * 0.06, ease: [0.16, 1, 0.3, 1] }}
          >
            <span className="building-pill">{p.name}</span>
            <span className="building-pill-desc">{p.desc}</span>
          </motion.div>
        ))}
      </motion.div>
    </section>
  );
}
