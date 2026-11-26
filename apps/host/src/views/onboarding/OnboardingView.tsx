import { Upload } from 'lucide-react';
import { Button, EmptyState, Input, Section, Select } from '@mungini/ui-web';
import { useOnboarding } from '@/hooks/onboarding/useOnboarding';

export default function OnboardingView() {
  const view = useOnboarding();

  return (
    <div className="flex flex-col gap-4 md:gap-6">
      <Section title={view.labels.title} description={view.labels.subtitle}>
        {view.error ? <p className="text-label text-destructive">{view.error}</p> : null}
        {view.loading ? <p className="text-body text-muted-foreground">{view.labels.noApplication}</p> : null}
        {!view.loading && view.application ? (
          <div className="flex flex-col gap-2">
            <div className="flex flex-wrap items-center justify-between gap-2">
              <div>
                <p className="text-body font-medium text-foreground">{view.application.displayName}</p>
                <p className="text-label text-muted-foreground">{view.labels.status(view.application.status)}</p>
              </div>
              <div className="flex gap-2">
                {view.application.status === 'PENDING_REVIEW' ? (
                  <Button variant="outline" size="sm" onClick={() => void view.withdraw()}>
                    {view.labels.withdraw}
                  </Button>
                ) : null}
                {view.application.status === 'DRAFT' ? (
                  <Button variant="primary" size="sm" onClick={() => void view.submit()}>
                    {view.labels.submit}
                  </Button>
                ) : null}
              </div>
            </div>
          </div>
        ) : null}
        {!view.loading && view.editable ? (
          <div className="flex flex-col gap-3 border-t border-border pt-3">
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.displayName}
              <Input value={view.displayName} onChange={(event) => view.setDisplayName(event.target.value)} />
            </label>
            <label className="flex flex-col gap-1 text-label text-muted-foreground">
              {view.labels.kind}
              <Select
                options={[
                  { value: 'INDIVIDUAL', label: view.labels.individual },
                  { value: 'BUSINESS', label: view.labels.business },
                ]}
                value={view.kind}
                onValueChange={view.setKind}
                ariaLabel={view.labels.kind}
              />
            </label>
            <div>
              <Button variant="outline" size="sm" onClick={() => void view.saveDraft()} disabled={!view.displayName.trim()}>
                {view.labels.saveDraft}
              </Button>
            </div>
          </div>
        ) : null}
      </Section>

      {view.application ? (
        <Section title={view.labels.documents}>
          {view.application.documents.length === 0 ? (
            <EmptyState title={view.labels.empty} />
          ) : (
            <ul className="divide-y divide-border">
              {view.application.documents.map((document) => (
                <li key={document.id} className="flex items-center justify-between gap-2 py-2">
                  <div className="min-w-0">
                    <p className="truncate text-body font-medium text-foreground">{document.fileName}</p>
                    <p className="text-label text-muted-foreground">
                      {view.labels.documentKindLabel(document.kind)} · {document.status}
                    </p>
                  </div>
                  {view.application?.status === 'DRAFT' && document.status === 'PENDING' ? (
                    <Button variant="ghost" size="sm" onClick={() => void view.removeDocument(document.id)}>
                      {view.labels.remove}
                    </Button>
                  ) : null}
                </li>
              ))}
            </ul>
          )}
          {view.application.status === 'DRAFT' ? (
            <div className="flex flex-col gap-2 border-t border-border pt-3 sm:flex-row sm:items-end">
              <label className="flex flex-1 flex-col gap-1 text-label text-muted-foreground">
                {view.labels.documentKind}
                <Select
                  options={view.documentKinds.map((value) => ({ value, label: view.labels.documentKindLabel(value) }))}
                  value={view.documentKind}
                  onValueChange={view.setDocumentKind}
                  placeholder={view.labels.documentKind}
                  ariaLabel={view.labels.documentKind}
                />
              </label>
              <label className="flex flex-1 flex-col gap-1 text-label text-muted-foreground">
                {view.labels.documentFile}
                <Input type="text" value={view.file?.name ?? ''} readOnly placeholder={view.labels.documentFile} />
                <input
                  type="file"
                  accept="application/pdf,image/jpeg,image/png,image/webp"
                  className="text-label text-muted-foreground"
                  onChange={(event) => view.readFile(event.target.files?.[0] ?? null)}
                  aria-label={view.labels.documentFile}
                />
              </label>
              <Button
                variant="primary"
                size="sm"
                icon={<Upload />}
                onClick={() => void view.addDocument()}
                disabled={!view.file || !view.documentKind}
              >
                {view.labels.addDocument}
              </Button>
            </div>
          ) : null}
        </Section>
      ) : null}
    </div>
  );
}
