import { TouchableOpacity, View } from 'react-native';
import { Divider } from 'react-native-paper';

/**
 * Props for the ListItem component.
 */
export interface ListItemProps {
	/** Rendered on the left side of the element*/
	left: JSX.Element;
	/** Rendered on the right side of the element*/
	right: JSX.Element;
	/** Called when the item is clicked */
	onClick?: () => void;
}
/**
 * A styled wrapper for an item in a lists
 */
export function ListItem(props: ListItemProps) {
	const { left, right, onClick } = props;
	const innerView = (
		<>
			<View style={{
				display: 'flex',
				flexDirection: 'row',
				alignItems: 'center',
				height: 50,
				paddingHorizontal: 10
			}}>
				<View style={{ flex: 1 }}>
					{left}
				</View>
				<View style={{ marginLeft: 'auto' }}>
					{right}
				</View>

			</View>
			<Divider />
		</>);

	return (<>
		{
			onClick ?
				<TouchableOpacity onPress={() => onClick()}>
					{innerView}
				</TouchableOpacity >
				: innerView
		}
	</>);
}