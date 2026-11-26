import { TripDetailView } from '@/components/trips/TripDetailView';

type Props = { params: Promise<{ reservationId: string }> };

export default async function TripDetailPage({ params }: Props) {
  const { reservationId } = await params;
  return <TripDetailView reservationId={reservationId} />;
}
