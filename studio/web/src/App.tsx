import { HOME_HEADING, homeHint } from './home';

export default function App() {
  return (
    <>
      <header className="top">
        <span className="brand">
          studio<span>.</span>
        </span>
      </header>
      <main className="wrap">
        <div className="home-head">
          <h1>{HOME_HEADING}</h1>
          <p>{homeHint(0)}</p>
        </div>
      </main>
    </>
  );
}
