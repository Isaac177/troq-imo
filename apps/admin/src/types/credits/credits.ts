import type { components } from '@mungini/api';

export type CreditSummary = components['schemas']['CreditSummary'];
export type CreditAccount = components['schemas']['AccountRow'];
export type CreditAccountPage = components['schemas']['Envelope_Page_AccountRow']['data'];
export type CreditDetail = components['schemas']['CreditDetail'];
export type CreditLedgerRow = components['schemas']['AdminLedgerRow'];
export type CreditLedgerPage = components['schemas']['Envelope_Page_AdminLedgerRow']['data'];
export type CreditBalance = components['schemas']['BalanceView'];
export type CommissionRate = components['schemas']['CurrentRate'];
export type CommissionRateRow = components['schemas']['RateRow'];
export type CommissionRatePage = components['schemas']['Envelope_Page_RateRow']['data'];
export type TopupInput = components['schemas']['TopupInput'];
export type AdjustInput = components['schemas']['AdjustInput'];
export type RateInput = components['schemas']['RateInput'];

export type CreditStatus = 'ENABLED' | 'LOW' | 'DISABLED';
export type CreditDirection = 'CREDIT' | 'DEBIT';
export type CreditEntryType = 'TOPUP' | 'COMMISSION' | 'REFUND' | 'ADJUSTMENT';
export type CreditEntryMode = 'topup' | 'adjust';
export type RequestResult<T> = { data: T | null; error: string | null };
