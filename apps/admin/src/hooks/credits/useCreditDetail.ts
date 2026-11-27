import { useEffect, useState } from 'react';
import { useTranslation } from '@mungini/i18n';
import { useSession } from '@mungini/session';
import { formatCdf, formatDateTime } from '@mungini/utils';
import { getCommissionRate, getCreditDetail } from '@/services/credits/credits.api';
import type { CommissionRate, CreditDetail, CreditStatus } from '@/types/credits/credits';
import { useCityTimezone } from '@/hooks/credits/useCityTimezone';

export type CreditDialog = 'topup' | 'adjust' | 'rate' | null;

export const useCreditDetail = (hostId: string | null, onChanged: () => void) => {
  const { t, i18n } = useTranslation('adminCredits');
  const translate = t as unknown as (key: string) => string;
  const { client, city } = useSession();
  const { timezone } = useCityTimezone();
  const [detail, setDetail] = useState<CreditDetail | null>(null);
  const [rate, setRate] = useState<CommissionRate | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [forbidden, setForbidden] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [dialog, setDialog] = useState<CreditDialog>(null);
  const [refreshToken, setRefreshToken] = useState(0);

  useEffect(() => {
    setDetail(null);
    setRate(null);
    setError(null);
    setForbidden(false);
    setNotice(null);
    setDialog(null);
  }, [hostId]);

  useEffect(() => {
    if (!hostId) return;
    let active = true;
    setError(null);
    setForbidden(false);
    void Promise.all([getCreditDetail(client, hostId), getCommissionRate(client, hostId)]).then(([detailResult, rateResult]) => {
      if (!active) return;
      if (detailResult.error || !detailResult.data) {
        const denied = detailResult.error === 'PERMISSION_MISSING';
        setForbidden(denied);
        setError(denied ? translate('detail.forbidden') : translate('detail.error'));
        setDetail(null);
        return;
      }
      setDetail(detailResult.data);
      if (rateResult.data) setRate(rateResult.data);
    });
    return () => {
      active = false;
    };
  }, [client, city, hostId, refreshToken, translate]);

  const status: CreditStatus = !detail ? 'ENABLED' : !detail.enabled ? 'DISABLED' : detail.low ? 'LOW' : 'ENABLED';
  const money = (value: string | number | null | undefined) => (value === null || value === undefined ? '—' : formatCdf(value, i18n.language));
  const percent = (bps: number) => new Intl.NumberFormat(i18n.language, { style: 'percent', minimumFractionDigits: 0, maximumFractionDigits: 2 }).format(bps / 10000);
  const tone = (value: CreditStatus): 'success' | 'warning' | 'danger' => (value === 'DISABLED' ? 'danger' : value === 'LOW' ? 'warning' : 'success');

  return {
    detail, rate, error, forbidden, notice, dialog, timezone,
    currentBps: detail?.rateBps ?? rate?.rateBps ?? 0,
    openTopup: () => setDialog('topup'),
    openAdjust: () => setDialog('adjust'),
    openRate: () => setDialog('rate'),
    closeDialog: () => setDialog(null),
    reloadToken: refreshToken,
    markSaved: () => {
      setNotice(translate('detail.saved'));
      setRefreshToken((value) => value + 1);
      onChanged();
    },
    status,
    statusTone: tone(status),
    stateLabel: (value: CreditStatus) => translate(`states.${value}`),
    money,
    percent,
    time: (value: string | null | undefined) => (value ? formatDateTime(value, timezone) : ''),
    labels: {
      title: translate('detail.title'), balance: translate('detail.balance'), lowThreshold: translate('detail.lowThreshold'),
      commissionRate: translate('detail.commissionRate'), activeSince: translate('detail.activeSince'), platformDefault: translate('detail.platformDefault'),
      close: translate('detail.close'),
      forbidden: translate('detail.forbidden'),
      topup: translate('detail.topup'), adjust: translate('detail.adjust'), setRate: translate('detail.setRate'),
    },
  };
};

export type CreditDetailView = ReturnType<typeof useCreditDetail>;
