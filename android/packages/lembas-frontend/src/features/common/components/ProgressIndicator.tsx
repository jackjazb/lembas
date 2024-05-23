import { View } from 'react-native';
import { useTheme } from 'react-native-paper';

export interface ProgressIndicatorProps {
	/** The value to be displayed by the progress bar */
	value: number;
	/** The maximum number the progress bar can reach */
	max: number;
}

/**
 * Renders a linear progress bar given
*/
export function ProgressIndicator(props: ProgressIndicatorProps) {
	const { value, max } = props;
	const theme = useTheme();
	const percentage = (value / max) * 100;

	return (
		<View style={{ backgroundColor: theme.colors.primaryContainer, flexDirection: 'row', height: 10, borderRadius: 50 }}>
			<View style={{ backgroundColor: theme.colors.primary, width: `${percentage}%`, borderRadius: 50 }} />
		</View >
	);
}