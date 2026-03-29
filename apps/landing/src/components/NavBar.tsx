import { useEffect, useRef, useState } from "react";
import { Link, useLocation } from "react-router-dom";
import { CONNECT_PATH, SEARCH_PATH } from "../config";
import "./NavBar.css";

export default function NavBar() {
  const location = useLocation();
  const isConnect = location.pathname === CONNECT_PATH;
  const isSearch = location.pathname === SEARCH_PATH || location.pathname.startsWith("/agents/");
  const [scrolled, setScrolled] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 24);
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  useEffect(() => {
    setMenuOpen(false);
  }, [location.pathname, location.hash]);

  useEffect(() => {
    if (!menuOpen) return;
    const close = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) setMenuOpen(false);
    };
    document.addEventListener("click", close);
    document.body.style.overflow = "hidden";
    return () => {
      document.removeEventListener("click", close);
      document.body.style.overflow = "";
    };
  }, [menuOpen]);

  return (
    <header className={`navbar ${scrolled ? "scrolled" : ""}`} role="banner">
      <div className="navbar-inner" ref={menuRef}>
        <Link to="/" className="navbar-logo" aria-label="Pronox home">
          Pronox
        </Link>
        <button
          type="button"
          className="navbar-menu-btn"
          onClick={(e) => {
            e.stopPropagation();
            setMenuOpen((o) => !o);
          }}
          aria-expanded={menuOpen}
          aria-label="Toggle menu"
        >
          <span className="navbar-menu-icon" />
        </button>
        <nav className={`navbar-links ${menuOpen ? "open" : ""}`} aria-label="Main">
          <Link to="/#what-we-build">What we build</Link>
          <Link to="/#roadmap">Roadmap</Link>
          <Link to={SEARCH_PATH} className={isSearch ? "active" : undefined}>
            Agents
          </Link>
          <Link to="/#contact">Contact</Link>
          <Link
            to={CONNECT_PATH}
            className={isConnect ? "navbar-cta active" : "navbar-cta"}
            aria-current={isConnect ? "page" : undefined}
          >
            Connect
          </Link>
        </nav>
      </div>
      {menuOpen && <div className="navbar-backdrop" aria-hidden />}
    </header>
  );
}
