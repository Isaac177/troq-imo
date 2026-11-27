import { ChevronLeft, ChevronRight } from 'lucide-react';
import { Button } from '@mungini/ui-web';

type Props = {
  photo: { id: string; url: string; alt: string };
  hasMultiplePhotos: boolean;
  previous: () => void;
  next: () => void;
  previousLabel: string;
  nextLabel: string;
};

export const TripGallery = ({ photo, hasMultiplePhotos, previous, next, previousLabel, nextLabel }: Props) => (
  <div className="relative overflow-hidden rounded-card">
    <img src={photo.url} alt={photo.alt} className="h-[180px] w-full object-cover transition-opacity duration-200 ease-in-out" />
    {hasMultiplePhotos ? <div className="absolute inset-x-2 top-1/2 flex -translate-y-1/2 justify-between">
      <Button size="icon" variant="secondary" aria-label={previousLabel} title={previousLabel} onClick={previous}><ChevronLeft className="size-4" /></Button>
      <Button size="icon" variant="secondary" aria-label={nextLabel} title={nextLabel} onClick={next}><ChevronRight className="size-4" /></Button>
    </div> : null}
  </div>
);
