import { Text } from 'react-native-paper';
export function CentredText(props): JSX.Element {
	const { children, style } = props;
	return (
		<Text variant='titleMedium'
			style={{
				flex: 1,
				textAlign: 'center',
				textAlignVertical: 'center',
				...style,
			}}
		>
			{children}
		</Text>
	);
}