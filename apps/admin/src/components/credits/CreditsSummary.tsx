import { Skeleton, StatCard } from '@mungini/ui-web';

type Card = { key: string; label: string; value: string };

type Props = { cards: Card[]; loading: boolean };

export default function CreditsSummary({ cards, loading }: Props) {
  if (loading && cards.length === 0) {
    return (
      <div className="grid grid-cols-2 gap-3 md:grid-cols-3 lg:grid-cols-6">
        {Array.from({ length: 6 }, (_, index) => <Skeleton key={index} shape="block" className="h-[68px]" />)}
      </div>
    );
  }
  if (cards.length === 0) return null;
  return (
    <div className="grid grid-cols-2 gap-3 md:grid-cols-3 lg:grid-cols-6">
      {cards.map((card) => <StatCard key={card.key} label={card.label} value={card.value} />)}
    </div>
  );
}
