import { Link } from 'react-router-dom';
import { Plus } from 'lucide-react';
import { Button, EmptyState, PagedTable, Section, Select } from '@mungini/ui-web';
import PromotionDialog from '@/components/rates/PromotionDialog';
import RatePlanDialog from '@/components/rates/RatePlanDialog';
import RateRuleDialog from '@/components/rates/RateRuleDialog';
import { usePromotions } from '@/hooks/rates/usePromotions';
import { useRatePlans } from '@/hooks/rates/useRatePlans';
import { useRateRules } from '@/hooks/rates/useRateRules';

export default function RatesView() {
  const view = useRatePlans();
  const rules = useRateRules(view.propertyId, view.selectedPlan, view.canWrite);
  const promotions = usePromotions(view.propertyId, view.unitName, view.canPromote);

  if (!view.hasHost) {
    return (
      <Section title={view.labels.title} description={view.labels.subtitle}>
        <EmptyState title={view.labels.noHost} action={<Button asChild variant="primary" size="sm"><Link to="/onboarding">{view.labels.noHostAction}</Link></Button>} />
      </Section>
    );
  }

  return (
    <>
      <div className="flex flex-col gap-4 md:gap-6">
        <Section
          title={view.labels.plans}
          description={view.labels.subtitle}
          actions={
            <div className="flex w-full flex-col gap-2 sm:w-auto sm:flex-row">
              {view.hosts.length > 1 ? (
                <div className="w-full sm:w-[160px]">
                  <Select options={view.hosts.map((host) => ({ value: host.hostId, label: host.displayName }))} value={view.hostId} onValueChange={view.setHostId} ariaLabel={view.labels.title} />
                </div>
              ) : null}
              <div className="w-full sm:w-[200px]">
                <Select options={view.properties} value={view.propertyId} onValueChange={view.setPropertyId} placeholder={view.labels.chooseProperty} ariaLabel={view.labels.property} />
              </div>
              <Button variant="primary" size="sm" icon={<Plus />} collapseLabel aria-label={view.labels.newPlan} title={view.labels.newPlan} disabled={!view.propertyId || !view.canWrite} onClick={view.openCreate}>{view.labels.newPlan}</Button>
            </div>
          }
        >
          {view.error ? <p role="alert" className="text-label text-destructive">{view.error}</p> : null}
          {view.notice ? <p role="status" className="text-label text-muted-foreground">{view.notice}</p> : null}
          <PagedTable
            rows={view.rows}
            columns={view.columns}
            getRowId={(row) => row.id}
            page={view.page}
            pageSize={view.pageSize}
            total={view.total}
            onPageChange={view.setPage}
            ariaLabel={view.labels.plans}
            loading={view.loading}
            onRowClick={(row) => view.setSelectedPlan(row)}
            emptyState={<EmptyState title={view.labels.plansEmpty} />}
          />
        </Section>

        {view.selectedPlan ? (
          <Section
            title={`${rules.labels.rulesFor} ${view.unitName(view.selectedPlan.unitTypeId) || view.selectedPlan.unitTypeId}`}
            actions={<Button variant="outline" size="sm" icon={<Plus />} collapseLabel aria-label={rules.labels.addRule} title={rules.labels.addRule} disabled={!view.canWrite} onClick={rules.openCreate}>{rules.labels.addRule}</Button>}
          >
            {rules.error ? <p role="alert" className="text-label text-destructive">{rules.error}</p> : null}
            {rules.notice ? <p role="status" className="text-label text-muted-foreground">{rules.notice}</p> : null}
            <PagedTable
              rows={rules.rows}
              columns={rules.columns}
              getRowId={(row) => row.id}
              page={rules.page}
              pageSize={rules.pageSize}
              total={rules.total}
              onPageChange={rules.setPage}
              ariaLabel={rules.labels.rules}
              loading={rules.loading}
              emptyState={<EmptyState title={rules.labels.rulesEmpty} />}
            />
          </Section>
        ) : null}

        <Section
          title={promotions.labels.promotions}
          actions={<Button variant="primary" size="sm" icon={<Plus />} collapseLabel aria-label={promotions.labels.newPromotion} title={promotions.labels.newPromotion} disabled={!view.propertyId || !view.canPromote} onClick={promotions.openCreate}>{promotions.labels.newPromotion}</Button>}
        >
          {promotions.error ? <p role="alert" className="text-label text-destructive">{promotions.error}</p> : null}
          {promotions.notice ? <p role="status" className="text-label text-muted-foreground">{promotions.notice}</p> : null}
          <PagedTable
            rows={promotions.rows}
            columns={promotions.columns}
            getRowId={(row) => row.id}
            page={promotions.page}
            pageSize={promotions.pageSize}
            total={promotions.total}
            onPageChange={promotions.setPage}
            ariaLabel={promotions.labels.promotions}
            loading={promotions.loading}
            emptyState={<EmptyState title={promotions.labels.promotionsEmpty} />}
          />
        </Section>
      </div>

      <RatePlanDialog view={view} />
      <RateRuleDialog view={rules} planName={view.selectedPlan ? view.unitName(view.selectedPlan.unitTypeId) || view.selectedPlan.unitTypeId : ''} />
      <PromotionDialog view={promotions} />
    </>
  );
}
