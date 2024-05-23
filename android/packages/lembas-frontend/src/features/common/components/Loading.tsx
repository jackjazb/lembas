import { View } from 'react-native';
import { ActivityIndicator, } from 'react-native-paper';

/**
 * A full screen component with a centred loading icon.
 * @returns 
 */
export function Loading(): JSX.Element {
	return (
		<View style={{ flex: 1, justifyContent: 'center' }}>
			<ActivityIndicator animating={true} />
		</View>
	);
}