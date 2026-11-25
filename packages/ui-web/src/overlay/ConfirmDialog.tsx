import { Button } from '../button/Button';
import { useUiCopy } from '../copy/useUiCopy';
import { Dialog } from './Dialog';

type ConfirmDialogProps = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  description: string;
  confirmLabel: string;
  cancelLabel?: string;
  destructive?: boolean;
  loading?: boolean;
  onConfirm: () => void;
};

export const ConfirmDialog = ({ open, onOpenChange, title, description, confirmLabel, cancelLabel, destructive = false, loading = false, onConfirm }: ConfirmDialogProps) => {
  const { common } = useUiCopy();
  return (
    <Dialog
      open={open}
      onOpenChange={loading ? () => undefined : onOpenChange}
      title={title}
      description={description}
      size="sm"
      footer={
        <>
          <Button variant="outline" disabled={loading} onClick={() => onOpenChange(false)}>
            {cancelLabel ?? common.cancel}
          </Button>
          <Button variant={destructive ? 'destructive' : 'primary'} loading={loading} onClick={onConfirm}>
            {confirmLabel}
          </Button>
        </>
      }
    />
  );
};
