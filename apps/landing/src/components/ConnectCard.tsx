import "./ConnectCard.css";

interface ConnectCardProps {
  title: string;
  description: string;
  href?: string;
  external?: boolean;
  copyText?: string | null;
  logo?: string;
  logoBg?: "white" | "transparent";
  /** When set, card is a button that runs this instead of linking. Use for copy-to-clipboard flows. */
  onClick?: () => void;
}

export default function ConnectCard({
  title,
  description,
  href,
  external = true,
  copyText,
  logo,
  logoBg,
  onClick,
}: ConnectCardProps) {
  const content = (
    <>
      {logo && (
        <div
          className={`connect-card-logo${logoBg === "white" ? " connect-card-logo-bg-white" : ""}`}
        >
          <img src={logo} alt="" width={40} height={40} />
        </div>
      )}
      <h3 className="connect-card-title">{title}</h3>
      <p className="connect-card-desc">{description}</p>
      {copyText && <p className="connect-card-copy">{copyText}</p>}
    </>
  );

  if (onClick) {
    return (
      <button type="button" className="connect-card" onClick={onClick}>
        {content}
        <span className="connect-card-arrow" aria-hidden>→</span>
      </button>
    );
  }

  if (href) {
    return (
      <a
        href={href}
        className="connect-card"
        target={external ? "_blank" : undefined}
        rel={external ? "noopener noreferrer" : undefined}
      >
        {content}
        <span className="connect-card-arrow" aria-hidden>→</span>
      </a>
    );
  }

  return <div className="connect-card connect-card-static">{content}</div>;
}
