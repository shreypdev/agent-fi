import { useState, useEffect } from "react";
import { motion } from "framer-motion";
import "./IntroOverlay.css";

type Props = {
  onComplete: () => void;
};

export default function IntroOverlay({ onComplete }: Props) {
  const [phase, setPhase] = useState<"welcome" | "handshake" | "ready" | "exit">("welcome");

  useEffect(() => {
    const t1 = setTimeout(() => setPhase("handshake"), 600);
    const t2 = setTimeout(() => setPhase("ready"), 2200);
    const t3 = setTimeout(() => setPhase("exit"), 3200);
    return () => {
      clearTimeout(t1);
      clearTimeout(t2);
      clearTimeout(t3);
    };
  }, []);

  useEffect(() => {
    if (phase !== "exit") return;
    const t = setTimeout(onComplete, 650);
    return () => clearTimeout(t);
  }, [phase, onComplete]);

  return (
    <motion.div
      className="intro-overlay"
      initial={{ opacity: 1 }}
      animate={phase === "exit" ? { opacity: 0 } : {}}
      transition={{ duration: 0.6, ease: [0.16, 1, 0.3, 1] }}
    >
      <div className="intro-bg" aria-hidden />
      <div className="intro-content">
        <motion.h1
          className="intro-title"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
        >
          Welcome to Pronox
        </motion.h1>
        <motion.p
          className="intro-sub"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.4, delay: 0.3 }}
        >
          {phase === "welcome" && "Initializing agent handshake..."}
          {phase === "handshake" && "Message passed between agents."}
          {phase === "ready" && "You're in. Let's go."}
          {phase === "exit" && "You're in. Let's go."}
        </motion.p>

        <div className="intro-visual">
          <svg
            viewBox="0 0 400 120"
            className="intro-svg"
            aria-hidden
          >
            <defs>
              <linearGradient id="introGrad" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" stopColor="var(--accent)" stopOpacity="0.2" />
                <stop offset="50%" stopColor="var(--accent)" stopOpacity="0.8" />
                <stop offset="100%" stopColor="var(--accent)" stopOpacity="0.2" />
              </linearGradient>
              <filter id="introGlow">
                <feGaussianBlur stdDeviation="2" result="blur" />
                <feMerge>
                  <feMergeNode in="blur" />
                  <feMergeNode in="SourceGraphic" />
                </feMerge>
              </filter>
            </defs>
            <line
              x1="80"
              y1="60"
              x2="320"
              y2="60"
              className="intro-line"
              stroke="url(#introGrad)"
            />
            <circle cx="80" cy="60" r="20" className="intro-node intro-node-a" />
            <circle cx="320" cy="60" r="20" className="intro-node intro-node-b" />
            <circle r="8" className="intro-message" filter="url(#introGlow)">
              <animateMotion
                dur="1.2s"
                repeatCount="indefinite"
                path="M 80 60 L 320 60"
              />
            </circle>
            <circle r="6" className="intro-message intro-message-2" filter="url(#introGlow)">
              <animateMotion
                dur="1.2s"
                repeatCount="indefinite"
                begin="0.6s"
                path="M 320 60 L 80 60"
              />
            </circle>
          </svg>
          <div className="intro-labels">
            <span className="intro-label">Agent A</span>
            <span className="intro-label">Agent B</span>
          </div>
        </div>

        <motion.div
          className="intro-progress-wrap"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.5 }}
        >
          <div className="intro-progress-bar">
            <motion.div
              className="intro-progress-fill"
              initial={{ width: "0%" }}
              animate={{ width: "100%" }}
              transition={{ duration: 3.2, ease: "linear" }}
            />
          </div>
        </motion.div>

        <button
          type="button"
          className="intro-skip"
          onClick={() => setPhase("exit")}
          aria-label="Skip intro"
        >
          Enter
        </button>
      </div>
    </motion.div>
  );
}
