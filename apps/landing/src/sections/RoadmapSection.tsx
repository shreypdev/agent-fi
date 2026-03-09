import { useRef } from "react";
import { motion, useInView } from "framer-motion";
import { roadmapByWeek, completedCount, totalCount } from "../data/roadmap";
import "./RoadmapSection.css";

export default function RoadmapSection() {
  const ref = useRef<HTMLElement>(null);
  const inView = useInView(ref, { once: true, margin: "-80px" });

  const pct = totalCount > 0 ? Math.round((completedCount / totalCount) * 100) : 0;

  return (
    <section className="roadmap-section section" ref={ref}>
      <motion.h2
        className="section-heading roadmap-heading"
        initial={{ opacity: 0, y: 16 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4 }}
      >
        Our track
      </motion.h2>
      <motion.p
        className="section-sub roadmap-sub"
        initial={{ opacity: 0, y: 12 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.05 }}
      >
        We update this live. {completedCount} of {totalCount} milestones done.
      </motion.p>
      <motion.div
        className="roadmap-progress-wrap"
        initial={{ opacity: 0, y: 12 }}
        animate={inView ? { opacity: 1, y: 0 } : {}}
        transition={{ duration: 0.4, delay: 0.1 }}
      >
        <div className="roadmap-progress-bar">
          <motion.div
            className="roadmap-progress-fill"
            initial={{ width: 0 }}
            animate={inView ? { width: `${pct}%` } : { width: 0 }}
            transition={{ duration: 1, ease: [0.16, 1, 0.3, 1] }}
          />
        </div>
        <span className="roadmap-progress-label">{pct}% complete</span>
      </motion.div>
      <motion.div
        className="roadmap-timeline"
        initial={{ opacity: 0 }}
        animate={inView ? { opacity: 1 } : {}}
        transition={{ duration: 0.4, delay: 0.2 }}
      >
        {roadmapByWeek.map((group, gi) => (
          <motion.div
            key={group.week}
            className="roadmap-week-card"
            initial={{ opacity: 0, y: 20 }}
            animate={inView ? { opacity: 1, y: 0 } : {}}
            transition={{ duration: 0.4, delay: 0.25 + gi * 0.06 }}
          >
            <h3 className="roadmap-week-label">
              Week {group.week}: {group.label}
            </h3>
            <ul className="roadmap-items">
              {group.items.map((item, ii) => (
                <motion.li
                  key={item.id}
                  className={`roadmap-item ${item.done ? "done" : ""}`}
                  initial={{ opacity: 0, x: -8 }}
                  animate={inView ? { opacity: 1, x: 0 } : {}}
                  transition={{ duration: 0.3, delay: 0.3 + gi * 0.06 + ii * 0.02 }}
                >
                  {item.done ? (
                    <span className="roadmap-check" aria-hidden>✓</span>
                  ) : (
                    <span className="roadmap-dot" aria-hidden />
                  )}
                  <span>{item.title}</span>
                </motion.li>
              ))}
            </ul>
          </motion.div>
        ))}
      </motion.div>
    </section>
  );
}
