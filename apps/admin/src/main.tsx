import '@mungini/theme/tokens.css';
import '@mungini/ui-web/theme-toggle.css';
import '@mungini/ui-web/shell.css';
import './shell.css';
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { I18nextProvider } from '@mungini/i18n';
import { i18n } from '@/i18n/instance';
import App from '@/App';

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
