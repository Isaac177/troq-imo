import { I18nextProvider } from '@mungini/i18n';
import { i18n } from '@/i18n/instance';
import HomeScreen from '@/screens/home/HomeScreen';

export default function App() {
  return (
    <I18nextProvider i18n={i18n}>
      <HomeScreen />
    </I18nextProvider>
  );
}
