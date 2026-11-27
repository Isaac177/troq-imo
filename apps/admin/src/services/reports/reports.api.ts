import type { SessionClient } from '@mungini/session';
import type { ReportRange, ReportSummary } from '@/types/reports/reports';


export const loadReport = async (client: SessionClient, range: ReportRange): Promise<ReportSummary> => {
  const response = await client.api.GET('/api/admin/reports/summary', {
    params: { query: { ...(range.start ? { from: range.start } : {}), ...(range.end ? { to: range.end } : {}) } },
  });
  if (response.error || !response.data) throw new Error('REPORT_LOAD_FAILED');
  return response.data.data;
};
