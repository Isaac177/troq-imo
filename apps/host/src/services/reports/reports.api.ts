import type { SessionClient } from '@mungini/session';
import type { ReportRange, ReportSummary } from '@/types/reports/reports';

export const loadReport = async (client: SessionClient, hostId: string, range: ReportRange, propertyId?: string): Promise<ReportSummary> => {
  const response = await client.api.GET('/api/host/analytics/summary', {
    params: {
      query: {
        hostId,
        ...(propertyId ? { propertyId } : {}),
        ...(range.start ? { from: range.start } : {}),
        ...(range.end ? { to: range.end } : {}),
      },
    },
  });
  if (response.error || !response.data) throw new Error('REPORT_LOAD_FAILED');
  return response.data.data;
};
