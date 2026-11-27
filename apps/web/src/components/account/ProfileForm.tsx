'use client';

import { Button, Input, Select } from '@mungini/ui-web';
import { useProfileForm } from '@/hooks/account/useProfileForm';

export const ProfileForm = () => {
  const form = useProfileForm();

  return (
    <form onSubmit={form.save} className="flex flex-col gap-3 rounded-card border border-border bg-card p-4 transition-all duration-200 ease-in-out">
      <h2 className="text-body font-semibold">{form.labels.title}</h2>
      <div className="grid grid-cols-1 gap-3 sm:grid-cols-2">
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {form.labels.firstName}
          <Input value={form.firstName} onChange={form.changeFirstName} autoComplete="given-name" maxLength={160} />
        </label>
        <label className="flex flex-col gap-1 text-label text-muted-foreground">
          {form.labels.lastName}
          <Input value={form.lastName} onChange={form.changeLastName} autoComplete="family-name" maxLength={160} />
        </label>
      </div>
      <label className="flex flex-col gap-1 text-label text-muted-foreground">
        {form.labels.language}
        <Select value={form.language} onValueChange={form.setLanguage} options={form.languageOptions} ariaLabel={form.labels.language} disabled={form.busy || form.languageOptions.length === 0} />
      </label>
      {form.languageError ? <p role="alert" className="text-label text-destructive">{form.labels.languageError}</p> : null}
      {form.error ? <p role="alert" className="text-label text-destructive">{form.error}</p> : null}
      {form.saved ? <p role="status" className="text-label text-muted-foreground">{form.labels.saved}</p> : null}
      <div><Button type="submit" size="sm" disabled={form.busy}>{form.busy ? form.labels.saving : form.labels.save}</Button></div>
    </form>
  );
};
