import { useEffect, useState } from "react";
import { Routes, Route, useLocation } from "react-router-dom";
import { AGENT_DETAIL_PATH, CONNECT_PATH, SEARCH_PATH } from "./config";
import NavBar from "./components/NavBar";
import IntroOverlay from "./components/IntroOverlay";
import HomePage from "./pages/HomePage";
import ConnectPage from "./pages/ConnectPage";
import SearchPage from "./pages/SearchPage";
import AgentDetailPage from "./pages/AgentDetailPage";
import "./App.css";

function DocumentTitle() {
  const { pathname } = useLocation();
  useEffect(() => {
    document.title =
      pathname === CONNECT_PATH ? "Connect | Pronox" : "Pronox | A2A infrastructure for the agent economy";
  }, [pathname]);
  return null;
}

function ScrollToTop() {
  const { pathname, hash } = useLocation();
  useEffect(() => {
    if (pathname === "/" && hash) return;
    window.scrollTo(0, 0);
  }, [pathname, hash]);
  return null;
}

function ScrollToHash() {
  const { pathname, hash } = useLocation();
  useEffect(() => {
    if (pathname !== "/" || !hash) return;
    const id = hash.slice(1);
    const t = setTimeout(() => {
      const el = document.getElementById(id);
      el?.scrollIntoView({ behavior: "smooth", block: "start" });
    }, 100);
    return () => clearTimeout(t);
  }, [pathname, hash]);
  return null;
}

export default function App() {
  const [showIntro, setShowIntro] = useState(true);

  return (
    <>
      {showIntro && (
        <IntroOverlay onComplete={() => setShowIntro(false)} />
      )}
      <DocumentTitle />
      <ScrollToTop />
      <ScrollToHash />
      <NavBar />
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path={SEARCH_PATH} element={<SearchPage />} />
        <Route path={AGENT_DETAIL_PATH} element={<AgentDetailPage />} />
        <Route path={CONNECT_PATH} element={<ConnectPage />} />
      </Routes>
    </>
  );
}
