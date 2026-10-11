import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import { startEngine } from './engine/start';
import './theme.css';
import './app.css';

const root = document.getElementById('root');
if (!root) {
  throw new Error('The page has no #root element.');
}

// The engine is chosen first: the app asks it for everything. For the browser engine this may
// reload the page once (cross-origin isolation), and then never returns. Until then the page
// says it is starting rather than staying blank.
const app = createRoot(root);
app.render(<p className="starting">Starting…</p>);
void startEngine().then(() => {
  app.render(
    <StrictMode>
      <App />
    </StrictMode>,
  );
});
