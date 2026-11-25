import { Text, View } from 'react-native';
import { useHomeScreen } from '@/hooks/home/useHomeScreen';
import { styles } from './HomeScreen.styles';

export default function HomeScreen() {
  const { appName } = useHomeScreen();

  return (
    <View style={styles.container}>
      <Text style={styles.title}>{appName}</Text>
    </View>
  );
}
