import { CKEditor } from '@ckeditor/ckeditor5-react';
import { ClassicEditor } from 'ckeditor5';
import 'ckeditor5/ckeditor5.css';
import './rich-text.css';
import { cn } from '../lib/cn';
import { useRichTextEditor } from './useRichTextEditor';

type RichTextEditorProps = {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  className?: string;
};

export const RichTextEditor = ({ value, onChange, placeholder, disabled = false, className }: RichTextEditorProps) => {
  const view = useRichTextEditor({ placeholder, onChange });
  return (
    <div className={cn('mungini-rich-text', className)}>
      <CKEditor editor={ClassicEditor} config={view.config} data={value} disabled={disabled} onChange={view.handleChange} />
    </div>
  );
};
