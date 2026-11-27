import { useEffect, useState } from 'react';
import { useSession } from '@mungini/session';
import { listCities } from '@/services/cases/cases.api';

export const useCityTimezone = () => {
  const { client, city } = useSession();
  const [timezone, setTimezone] = useState('UTC');

  useEffect(() => {
    let active = true;
    void listCities(client).then(({ data }) => {
      const match = data?.data.find((item) => item.code === city);
      if (active && match) setTimezone(match.timezone);
    }).catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, city]);

  return timezone;
};
