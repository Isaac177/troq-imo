import { useCallback, useMemo } from 'react';
import type { EditorConfig } from 'ckeditor5';
import { BlockQuote, Bold, Essentials, Heading, Italic, Link, List, Paragraph, Underline } from 'ckeditor5';
import frTranslations from 'ckeditor5/translations/fr.js';
import { useUiCopy } from '../copy/useUiCopy';

type Options = {
  placeholder?: string;
  onChange?: (value: string) => void;
};

export const useRichTextEditor = ({ placeholder, onChange }: Options) => {
  const { locale, richText } = useUiCopy();
  const french = locale.startsWith('fr');
  const config = useMemo<EditorConfig>(
    () => ({
      licenseKey: 'GPL',
      plugins: [Essentials, Paragraph, Heading, Bold, Italic, Underline, Link, List, BlockQuote],
      toolbar: { items: ['undo', 'redo', '|', 'heading', '|', 'bold', 'italic', 'underline', '|', 'link', 'bulletedList', 'numberedList', 'blockQuote'], shouldNotGroupWhenFull: true },
      link: { addTargetToExternalLinks: true, defaultProtocol: 'https://' },
      placeholder: placeholder ?? richText.placeholder,
      language: french ? 'fr' : 'en',
      translations: french ? [frTranslations] : [],
    }),
    [placeholder, richText.placeholder, french],
  );
  const handleChange = useCallback(
    (_event: unknown, editor: { getData: () => string }) => {
      onChange?.(editor.getData());
    },
    [onChange],
  );
  return { config, handleChange };
};
