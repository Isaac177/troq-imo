import './index.css';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { I18nextProvider } from '@mungini/i18n';
import { i18n } from '@/i18n/instance';
import App from '@/App';

document.documentElement.lang = i18n.language;

const container = document.getElementById('root');

if (container) {
  createRoot(container).render(
    <StrictMode>
      <I18nextProvider i18n={i18n}>
        <App />
      </I18nextProvider>
    </StrictMode>,
  );
}
