import { View } from 'react-native';
import { IconButton, Text } from 'react-native-paper';

export interface NumberPickerProps {
	value: number;
	hideValue?: boolean;
	lbound?: number;
	onChange: (value: number) => void;
}
export function NumberPicker(props: NumberPickerProps) {
	const { value, hideValue, onChange, lbound = 0 } = props;

	return (
		<View style={{ marginLeft: 'auto', flexDirection: 'row', alignItems: 'center', gap: 10 }}>
			<IconButton mode="contained" size={18} icon="minus" onPress={() => value === lbound ? onChange(value) : onChange(value - 1)} />
			{
				!hideValue &&
				<Text variant="bodyLarge">{value}</Text>
			}
			<IconButton mode="contained" size={18} icon="plus" onPress={() => onChange(value + 1)} />
		</View >
	);
}